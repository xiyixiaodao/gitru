use serde::{Deserialize, Serialize};

// Subject validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct SubjectConfig {
    pub pre_whitespace: bool,
    pub min_length: u8,
    pub max_length: u8,
}
