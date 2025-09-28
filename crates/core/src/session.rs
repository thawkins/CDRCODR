use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PromptEntry {
    pub prompt_text: String,
    pub response_summary: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CallLogEntry {
    pub timestamp: String,
    pub duration_ms: u64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Session {
    pub id: Uuid,
    pub created_at: String,
    pub project_id: Option<String>,
    pub backend: Option<String>,
    pub model: Option<String>,
    pub prompts: Vec<PromptEntry>,
    pub artifacts: Vec<crate::artifact::ArtifactMetadata>,
    pub call_log: Vec<CallLogEntry>,
}

impl Session {
    pub fn new(project_id: Option<String>) -> Self {
        let now = std::time::SystemTime::now();
        let created_at = match now.duration_since(std::time::SystemTime::UNIX_EPOCH) {
            Ok(d) => format!("{}", d.as_secs()),
            Err(_) => "0".to_string(),
        };

        Session {
            id: Uuid::new_v4(),
            created_at,
            project_id,
            backend: None,
            model: None,
            prompts: Vec::new(),
            artifacts: Vec::new(),
            call_log: Vec::new(),
        }
    }

    pub fn save(&self, dir: &std::path::Path) -> std::io::Result<()> {
        let mut p = PathBuf::from(dir);
        fs::create_dir_all(&p)?;
        p.push(format!("{}.json", self.id));
        let data = serde_json::to_vec(self).map_err(std::io::Error::other)?;
        fs::write(p, data)
    }

    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        let data = fs::read(path)?;
        let s: Session = serde_json::from_slice(&data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(s)
    }
}
