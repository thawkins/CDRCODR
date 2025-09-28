pub mod adapters;
pub mod artifact;
pub mod async_backend;
pub mod backend;
pub mod config;
pub mod git;
pub mod patch;
pub mod session;

// artifact::Artifact is not exported; only ArtifactMetadata is exported below
pub use artifact::ArtifactMetadata;
pub use async_backend::{AsyncBackend, AsyncMockAdapter};
pub use backend::{Artifacts, Backend, BackendError};
pub use config::ProjectConfig;
pub use git::create_branch_and_commit;
pub use patch::{Hunk, Patch, PatchReport};
pub use session::Session;
