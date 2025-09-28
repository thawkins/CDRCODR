use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectConfig {
    pub id: Option<String>,
    // Use string for backend to keep initial parsing simple ("ollama", "lmstudio", or "other")
    pub default_backend: Option<String>,
    pub default_model: Option<String>,
}

impl ProjectConfig {
}

impl std::str::FromStr for ProjectConfig {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}
