use serde::{Deserialize, Serialize};

// Scope validation configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct ScopeConfig {
    pub allow_empty: bool,
    pub allow_custom_scopes: bool,
    pub allowed_scopes: Vec<String>,
}
