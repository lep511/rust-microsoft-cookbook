use std::time::Duration;

pub mod chat;
pub mod embed;
pub mod error;
pub mod libs;
pub mod utils;
pub mod requests;

pub const RETRY_BASE_DELAY: Duration = Duration::from_secs(2);

pub static OPENAI_BASE_URL: &str = "https://api.openai.com/v1/chat/completions";
pub static OPENAI_EMBED_URL: &str = "https://api.openai.com/v1/embeddings";

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;