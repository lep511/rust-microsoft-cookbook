pub mod chat;
pub mod embed;
pub mod gen_config;
pub mod libs;
pub mod utils;
pub mod requests;
pub mod errors;

pub static GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
pub static UPLOAD_BASE_URL: &str = "https://generativelanguage.googleapis.com/upload/v1beta";

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;