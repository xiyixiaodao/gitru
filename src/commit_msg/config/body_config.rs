use serde::{Deserialize, Serialize};

// Body validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct BodyConfig {
    pub blank_line_at_start: bool,
    pub blank_lines_number: u8,
    pub max_line_length: u8,
}
