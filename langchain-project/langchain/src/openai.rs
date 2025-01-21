pub mod chat;
pub mod embed;
pub mod libs;
pub mod utils;
pub mod requests;

pub static OPENAI_BASE_URL: &str = "https://api.openai.com/v1/chat/completions";
pub static OPENAI_EMBED_URL: &str = "https://api.openai.com/v1/embeddings";

pub const DEBUG_PRE: bool = true;
pub const DEBUG_POST: bool = false;