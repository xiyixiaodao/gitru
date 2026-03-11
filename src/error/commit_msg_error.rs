use crate::error::body_error::BodyError;
use crate::error::footer_error::FooterError;
use crate::error::header_error::HeaderError;

#[derive(Debug)]
pub enum CommitMsgError {
    Header(HeaderError),

    Body(BodyError),

    Footer(FooterError),
}

impl std::fmt::Display for CommitMsgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitMsgError::Header(e) => write!(f, "{}", e),
            CommitMsgError::Body(e) => write!(f, "{}", e),
            CommitMsgError::Footer(e) => write!(f, "{}", e),
        }
    }
}
