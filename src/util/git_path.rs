//! Git repository type detection and path conventions
//!
//! This module models three kinds of Git repositories based on Git’s actual
//! filesystem behavior:
//!
//! - **Normal repository**: the working directory contains a `.git/` directory.
//! - **Worktree repository**: the working directory contains a `.git` file
//!   pointing to `.git/worktrees/<name>`. Hooks still reside in the main
//!   repository’s `.git/hooks`. The `commondir` file points back to the main
//!   `.git` directory.
//! - **Submodule repository**: the working directory contains a `.git` file
//!   pointing to `.git/modules/<submodule_name>`. Hooks live inside
//!   `.git/modules/<submodule_name>/hooks`. The parent repository’s `.git`
//!   directory is treated as the `super_git_dir`.
//!
//! Field conventions:
//! - `GitKind::NormalRepo.git_dir` always points to the repository’s own `.git` directory.
//! - `GitKind::Worktree.git_dir` points to the worktree’s gitdir (`.git/worktrees/<name>`).
//! - `GitKind::Worktree.main_git_dir` points to the main repository’s `.git` directory.
//! - `GitKind::Submodule.git_dir` points to the submodule’s gitdir (`.git/modules/<name>`).
//! - `GitKind::Submodule.super_git_dir` points to the parent repository’s `.git` directory.
//!
//! Provided functionality:
//! - `detect_git_kind(path)` detects whether the repository is a normal repo,
//!   worktree, or submodule by inspecting the `.git` file/directory.
//! - `GitKind::hook_path(hook_name)` returns the actual hook installation path.
//! - `GitKind::config_path(repo, file_name)` returns the configuration file path
//!   located in the working directory root.
//!
//! Design goals:
//! - Avoid relying on Git version–specific behavior.
//! - Centralize repository‑type detection and path derivation logic.
//! - Ensure hook and config paths behave consistently across all repo types.

use crate::error::git_error::GitKindError;
use std::fs;
use std::path::{Path, PathBuf};

pub enum GitKind {
    NormalRepo {
        git_dir: PathBuf,
        workdir: PathBuf,
    },
    Worktree {
        git_dir: PathBuf,
        main_git_dir: PathBuf,
        workdir: PathBuf,
    },
    Submodule {
        git_dir: PathBuf,
        super_git_dir: PathBuf,
        workdir: PathBuf,
    },
}

impl GitKind {
    /// Returns the working directory of this repository.
    pub fn workdir(&self) -> &Path {
        match self {
            GitKind::NormalRepo { workdir, .. } => workdir,
            GitKind::Worktree { workdir, .. } => workdir,
            GitKind::Submodule { workdir, .. } => workdir,
        }
    }

    /// Returns the root directory where Git hooks are installed,
    /// following Git’s actual behavior for normal repos, worktrees, and submodules.
    pub fn hooks_root(&self) -> PathBuf {
        match self {
            GitKind::NormalRepo { git_dir, .. } => git_dir.join("hooks"),
            GitKind::Worktree { main_git_dir, .. } => main_git_dir.join("hooks"),
            GitKind::Submodule { git_dir, .. } => git_dir.join("hooks"),
        }
    }

    /// Returns the full installation path of a specific Git hook.
    /// The hook location depends on the repository type.
    pub fn hook_path(&self, hook_name: &str) -> PathBuf {
        self.hooks_root().join(hook_name)
    }

    /// Returns the installation path of a configuration file.
    /// Config files are always stored at the root of the working directory.
    pub fn config_path(&self, file_name: &str) -> PathBuf {
        self.workdir().join(file_name)
    }
}

pub fn detect_current_repo() -> Result<GitKind, GitKindError> {
    detect_git_kind(".")
}

pub fn detect_git_kind(path: impl AsRef<Path>) -> Result<GitKind, GitKindError> {
    let workdir = find_workdir(path)?;
    let workdir = workdir.canonicalize()?;
    let git_path = workdir.join(".git");

    // 1) Normal repository
    if git_path.is_dir() {
        return Ok(GitKind::NormalRepo {
            git_dir: git_path.canonicalize()?,
            workdir,
        });
    }

    // 2) Worktree or submodule
    let content = fs::read_to_string(&git_path)?;
    let gitdir_raw = content
        .strip_prefix("gitdir:")
        .ok_or(GitKindError::InvalidGitDir)?
        .trim();

    let gitdir = resolve_gitdir(&git_path, gitdir_raw)?.canonicalize()?;

    // 2.1 Worktree
    let commondir_path = gitdir.join("commondir");
    if commondir_path.exists() {
        let commondir_raw = fs::read_to_string(&commondir_path)?.trim().to_string();
        let main_git_dir = resolve_gitdir(&gitdir, &commondir_raw)?.canonicalize()?;

        return Ok(GitKind::Worktree {
            git_dir: gitdir.clone(),
            main_git_dir,
            workdir,
        });
    }

    // 2.2 Submodule
    if gitdir.to_string_lossy().contains("/.git/modules/") {
        let super_git_dir = gitdir
            .ancestors()
            .find(|p| p.ends_with(".git"))
            .ok_or(GitKindError::InvalidPath)?
            .canonicalize()?;

        return Ok(GitKind::Submodule {
            git_dir: gitdir.clone(),
            super_git_dir,
            workdir,
        });
    }

    // Fallback: treat as a normal repository
    Ok(GitKind::NormalRepo {
        git_dir: gitdir,
        workdir,
    })
}

fn find_workdir(start: impl AsRef<Path>) -> Result<PathBuf, GitKindError> {
    let mut dir = start.as_ref().canonicalize()?;

    loop {
        if dir.join(".git").exists() {
            return Ok(dir);
        }

        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => return Err(GitKindError::InvalidPath),
        }
    }
}

/// Resolves a gitdir path that may be relative to the `.git` file location.
fn resolve_gitdir(base: &Path, gitdir: &str) -> Result<PathBuf, GitKindError> {
    let p = PathBuf::from(gitdir);
    if p.is_absolute() {
        Ok(p)
    } else {
        let parent = base.parent().ok_or(GitKindError::InvalidPath)?;
        Ok(parent.join(p))
    }
}

#[cfg(test)]
mod tests {
    use crate::util::git_path::{GitKind, detect_git_kind};

    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use tempfile::TempDir;

    fn run(cmd: &mut Command) {
        let status = cmd.status().expect("failed to run command");
        assert!(status.success());
    }

    /// Initialize a Git repository and create a `master` branch
    fn init_repo_with_master(path: &Path) {
        run(Command::new("git").arg("init").arg(path));

        // Create the master branch
        run(Command::new("git")
            .current_dir(path)
            .arg("checkout")
            .arg("-b")
            .arg("master"));

        // Commit at least one file (submodules require an initial commit)
        fs::write(path.join("init.txt"), "init").unwrap();
        run(Command::new("git").current_dir(path).arg("add").arg("."));
        run(Command::new("git")
            .current_dir(path)
            .arg("commit")
            .arg("-m")
            .arg("init"));
    }

    #[test]
    #[ignore]
    fn test_normal_repo() {
        let dir = TempDir::new().unwrap();
        init_repo_with_master(dir.path());

        let kind = detect_git_kind(dir.path()).unwrap();

        match kind {
            GitKind::NormalRepo { git_dir, workdir } => {
                assert!(git_dir.join("HEAD").exists());
                assert!(workdir.join(".git").exists());
            }
            _ => panic!("should detect NormalRepo"),
        }
    }

    #[test]
    #[ignore]
    fn test_worktree() {
        let main = TempDir::new().unwrap();
        let wt = TempDir::new().unwrap();

        // Initialize the main repository (with master)
        init_repo_with_master(main.path());

        // Create a new branch for the worktree
        run(Command::new("git")
            .current_dir(main.path())
            .arg("branch")
            .arg("wt-branch"));

        // Add a worktree using the new branch
        run(Command::new("git")
            .current_dir(main.path())
            .arg("worktree")
            .arg("add")
            .arg(wt.path())
            .arg("wt-branch"));

        let kind = detect_git_kind(wt.path()).unwrap();

        match kind {
            GitKind::Worktree {
                git_dir,
                main_git_dir,
                workdir: _,
            } => {
                assert!(git_dir.join("commondir").exists());
                println!("git_dir: {:?}", git_dir);
                println!("main_git_dir: {:?}", main_git_dir);
                let canonical = main_git_dir.canonicalize().unwrap();
                println!("canonical: {:?}", canonical);
                assert!(canonical.join(".git").join("HEAD").exists());
            }
            _ => panic!("should detect Worktree"),
        }
    }

    #[test]
    #[ignore]
    fn test_submodule() {
        let super_repo = TempDir::new().unwrap();

        // The sub_repo must be inside the super_repo
        let sub_repo = super_repo.path().join("sub_src");
        fs::create_dir(&sub_repo).unwrap();

        // Initialize the super repo
        init_repo_with_master(super_repo.path());

        // Initialize the sub repo (must have at least one commit)
        init_repo_with_master(&sub_repo);

        // ⭐ Use ./sub_src (important)
        // Allow file:// protocol (Git 2.38+ disables it by default)
        run(Command::new("git")
            .current_dir(super_repo.path())
            .arg("-c")
            .arg("protocol.file.allow=always")
            .arg("submodule")
            .arg("add")
            .arg("./sub_src")
            .arg("sub"));

        let sub_path = super_repo.path().join("sub");
        let kind = detect_git_kind(&sub_path).unwrap();

        match kind {
            GitKind::Submodule {
                git_dir,
                super_git_dir,
                workdir: _,
            } => {
                assert!(git_dir.ends_with("modules/sub"));
                assert!(super_git_dir.ends_with(".git"));
            }
            _ => panic!("should detect Submodule"),
        }
    }

    // hook_path / config_path for a Normal Repo
    #[test]
    #[ignore]
    fn test_hook_and_config_normal_repo() {
        let repo_dir = TempDir::new().unwrap();
        init_repo_with_master(repo_dir.path());

        let kind = detect_git_kind(repo_dir.path()).unwrap();

        // hook_path
        let hook = kind.hook_path("commit-msg");
        assert_eq!(
            hook,
            repo_dir
                .path()
                .join(".git")
                .join("hooks")
                .join("commit-msg")
        );

        // config_path
        let cfg = kind.config_path("gitru.toml");
        assert_eq!(cfg, repo_dir.path().join("gitru.toml"));
    }

    // hook_path / config_path for a Worktree
    #[test]
    #[ignore]
    fn test_hook_and_config_worktree() {
        let main = TempDir::new().unwrap();
        init_repo_with_master(main.path());

        // Create a worktree
        let wt = main.path().join("wt");

        run(Command::new("git")
            .current_dir(main.path())
            .arg("worktree")
            .arg("add")
            .arg("-b")
            .arg("wt-branch")
            .arg(&wt));

        let kind = detect_git_kind(&wt).unwrap();

        match kind {
            GitKind::Worktree {
                ref main_git_dir, ..
            } => {
                // hook_path
                let hook = kind.hook_path("commit-msg");
                assert_eq!(hook, main_git_dir.join("hooks").join("commit-msg"));
                println!("hook: {:?}", hook);
                println!("main_git_dir: {:?}", main_git_dir);

                // config_path
                let cfg = kind.config_path("gitru.toml");
                println!("cfg: {:?}", cfg);
                assert_eq!(cfg, wt.join("gitru.toml"));
            }
            _ => panic!("should detect Worktree"),
        }
    }

    // hook_path / config_path for a Submodule
    #[test]
    #[ignore]
    fn test_hook_and_config_submodule() {
        let super_repo = TempDir::new().unwrap();
        init_repo_with_master(super_repo.path());

        // The submodule must be inside the super_repo
        let sub_src = super_repo.path().join("sub_src");
        fs::create_dir(&sub_src).unwrap();
        init_repo_with_master(&sub_src);

        // Add the submodule (important: use ./sub_src + temporarily enable file protocol)
        run(Command::new("git")
            .current_dir(super_repo.path())
            .arg("-c")
            .arg("protocol.file.allow=always")
            .arg("submodule")
            .arg("add")
            .arg("./sub_src")
            .arg("sub"));

        let sub_path = super_repo.path().join("sub");
        let kind = detect_git_kind(&sub_path).unwrap();

        match &kind {
            GitKind::Submodule { git_dir, .. } => {
                // hook_path
                let hook = kind.hook_path("commit-msg");
                println!("hook: {:?}", hook);
                assert_eq!(hook, git_dir.join("hooks").join("commit-msg"));

                // config_path
                let cfg = kind.config_path("gitru.toml");
                println!("cfg: {:?}", cfg);
                assert_eq!(cfg, sub_path.join("gitru.toml"));
            }
            _ => panic!("should detect Submodule"),
        }
    }
}
