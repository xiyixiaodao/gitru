use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigStatusCheckError {
    #[error("configuration file '{0}' does not exist")]
    ConfigNotExist(String),

    #[error(
        "configuration file '{file}' was modified but not included in this commit. \
         Please add it to the commit or revert the changes."
    )]
    ConfigNotCommitted { file: String },

    #[error("git command failed: {0}")]
    GitError(String),

    #[error("invalid git status output: {0}")]
    InvalidGitStatusOutput(String),
}
