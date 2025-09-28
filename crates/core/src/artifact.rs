use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ArtifactMetadata {
    pub path: String,
    pub summary: String,
    pub checksum: Option<String>,
}

impl fmt::Display for ArtifactMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.path, self.summary)
    }
}

