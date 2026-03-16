use crate::util::colored_print::print_error;
use std::process::exit;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Find the root of a Git repository, supporting both normal repos and worktrees.
///
/// Rules:
/// - If `.git` is a directory → this is the repository root.
/// - If `.git` is a file → parse the `gitdir:` entry and ensure it points to a valid gitdir.
/// - Walk up through all ancestor directories until found.
pub fn find_repo_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        find_repo_root_from(cwd)
    } else {
        print_error("unable to get current directory");
        exit(1);
    }
}

fn find_repo_root_from(start: impl AsRef<Path>) -> PathBuf {
    for dir in start.as_ref().ancestors() {
        let git_path = dir.join(".git");

        if git_path.is_dir() {
            // Normal Git repository
            return dir.to_path_buf();
        }

        if git_path.is_file() {
            // Worktree: parse the gitdir file
            if let Ok(content) = fs::read_to_string(&git_path) {
                if let Some(first_line) = content.lines().next() {
                    if let Some(path) = first_line.strip_prefix("gitdir:").map(str::trim) {
                        let gitdir = resolve_gitdir(dir, path);
                        if gitdir.exists() {
                            return dir.to_path_buf();
                        }
                    }
                }
            }
        }
    }

    panic!("Could not find git repo directory");
}

/// Resolve the gitdir path:
/// - If the path is absolute → use it directly.
/// - If the path is relative → resolve it relative to the directory containing `.git`.
fn resolve_gitdir(base: &Path, gitdir_raw: &str) -> PathBuf {
    let gitdir_raw = gitdir_raw.trim();
    let gitdir = Path::new(gitdir_raw);

    if gitdir.is_absolute() {
        gitdir.to_path_buf()
    } else {
        base.join(gitdir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn write(path: impl AsRef<Path>, content: &str) {
        fs::write(path, content).unwrap();
    }

    // Normal repository test
    #[test]
    fn test_normal_repo_root() {
        let dir = TempDir::new().unwrap();
        let root = dir.path();

        fs::create_dir(root.join(".git")).unwrap();

        let nested = root.join("a/b/c");
        fs::create_dir_all(&nested).unwrap();

        let found = find_repo_root_from(&nested);
        assert_eq!(found, root);
    }

    // Worktree (absolute gitdir path)
    #[test]
    fn test_worktree_absolute_gitdir() {
        let dir = TempDir::new().unwrap();
        let root = dir.path();

        let real_gitdir = root.join("real/.git/worktrees/foo");
        fs::create_dir_all(&real_gitdir).unwrap();

        let worktree = root.join("worktree");
        fs::create_dir_all(&worktree).unwrap();

        write(
            worktree.join(".git"),
            &format!("gitdir: {}", real_gitdir.display()),
        );

        let found = find_repo_root_from(&worktree);
        assert_eq!(found, worktree);
    }

    // Worktree (relative gitdir path)
    #[test]
    fn test_worktree_relative_gitdir() {
        let dir = TempDir::new().unwrap();
        let root = dir.path();

        let real_gitdir = root.join(".git/worktrees/foo");
        fs::create_dir_all(&real_gitdir).unwrap();

        let worktree = root.join("wt");
        fs::create_dir_all(&worktree).unwrap();

        write(worktree.join(".git"), "gitdir: ../.git/worktrees/foo");

        let found = find_repo_root_from(&worktree);
        assert_eq!(found, worktree);
    }

    // Worktree with nested directories
    #[test]
    fn test_nested_inside_worktree() {
        let dir = TempDir::new().unwrap();
        let root = dir.path();

        let real_gitdir = root.join(".git/worktrees/foo");
        fs::create_dir_all(&real_gitdir).unwrap();

        let worktree = root.join("wt");
        let nested = worktree.join("deep/dir");
        fs::create_dir_all(&nested).unwrap();

        write(worktree.join(".git"), "gitdir: ../.git/worktrees/foo");

        let found = find_repo_root_from(&nested);
        assert_eq!(found, worktree);
    }

    // Not a Git repository
    #[test]
    #[should_panic]
    fn test_not_a_git_repo() {
        let dir = TempDir::new().unwrap();
        find_repo_root_from(dir.path());
    }

    // Corrupted .git file
    #[test]
    #[should_panic]
    fn test_invalid_git_file() {
        let dir = TempDir::new().unwrap();
        write(dir.path().join(".git"), "this is not a valid gitdir");
        find_repo_root_from(dir.path());
    }

    // gitdir points to a non-existent path
    #[test]
    #[should_panic]
    fn test_gitdir_points_to_nonexistent_path() {
        let dir = TempDir::new().unwrap();
        write(dir.path().join(".git"), "gitdir: /non/existent/path");
        find_repo_root_from(dir.path());
    }
}
