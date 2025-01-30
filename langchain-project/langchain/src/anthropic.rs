use std::time::Duration;

pub mod chat;
pub mod embed;
pub mod libs;
pub mod utils;
pub mod requests;

pub static ANTHROPIC_BASE_URL: &str = "https://api.anthropic.com/v1/messages";
pub static ANTHROPIC_EMBED_URL: &str = "https://api.voyageai.com/v1/embeddings";
pub static ANTHROPIC_EMBEDMUL_URL: &str = "https://api.voyageai.com/v1/multimodalembeddings";
pub static ANTHROPIC_EMBEDRANK_URL: &str = "https://api.voyageai.com/v1/rerank";
pub static ANTHROPIC_VERSION: &str = "2023-06-01";

pub const RETRY_BASE_DELAY: Duration = Duration::from_secs(2);

pub static MIME_TYPE_SUPPORTED: [&str; 6] = [
    "image/png", 
    "image/jpeg",
    "image/jpg",
    "image/gif", 
    "image/webp", 
    "image/gif"
];

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;