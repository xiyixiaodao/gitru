// Automatically locate the repository root directory
// If not found, it means the current project is not a Git repository, panic with a user message
pub fn find_repo_root() -> std::path::PathBuf {
    let mut dir = std::env::current_dir().expect("unable to get current directory");

    loop {
        if dir.join(".git").exists() {
            return dir;
        }
        if !dir.pop() {
            panic!("unable to find repository root, please run this command in a git repository");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_repo_root() {
        let repo_root = super::find_repo_root();
        assert!(repo_root.join(".git").exists());
        println!("{:?}", repo_root);
    }
}
