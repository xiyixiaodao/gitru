use serde::{Deserialize, Serialize};

// Footer validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct FooterConfig {
    pub allowed_keys: Vec<String>,
}
