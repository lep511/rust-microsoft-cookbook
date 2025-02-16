pub mod client;
pub mod error;
pub mod libs;
pub mod utils;
pub mod requests;

pub static LANGSMITH_BASE_URL: &str = "https://api.smith.langchain.com";

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;