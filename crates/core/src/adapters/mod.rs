pub mod lmstudio;
pub mod mock;
pub mod ollama;
pub mod trait_adapter;

pub use lmstudio::LMStudioAdapter;
pub use mock::LLMMockAdapter;
pub use mock::MockAdapter;
pub use ollama::OllamaAdapter;
pub use trait_adapter::LLMAdapter;
