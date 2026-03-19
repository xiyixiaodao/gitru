use crate::error::body_error::BodyError;
use crate::error::footer_error::FooterError;
use crate::error::git_error::{ConfigStatusCheckError, GitKindError};
use crate::error::header_error::HeaderError;
use std::path::PathBuf;

use thiserror::Error;

#[derive(thiserror::Error, Debug)]
pub enum CommitMsgError {
    #[error("{0}")]
    Header(#[from] HeaderError),

    #[error("{0}")]
    Body(#[from] BodyError),

    #[error("{0}")]
    Footer(#[from] FooterError),

    #[error("{0}")]
    ConfigStatus(#[from] ConfigStatusCheckError),

    #[error("{0}")]
    GitKind(#[from] GitKindError),

    #[error("{0}")]
    System(#[from] SystemError),
}

// impl std::fmt::Display for CommitMsgError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CommitMsgError::Header(e) => write!(f, "{}", e),
//             CommitMsgError::Body(e) => write!(f, "{}", e),
//             CommitMsgError::Footer(e) => write!(f, "{}", e),
//             CommitMsgError::Config(e) => write!(f, "{}", e),
//             CommitMsgError::Other(e) => write!(f, "{}", e),
//         }
//     }
// }

#[derive(Error, Debug)]
pub enum SystemError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("failed to find git repo root: {0}")]
    RepoRootNotFound(String),

    #[error("failed to read file `{path}`: {source}")]
    IoPath {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("system error: {0}")]
    Other(String),
}
