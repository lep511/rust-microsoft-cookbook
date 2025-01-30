use std::time::Duration;

pub mod chat;
pub mod embed;
pub mod libs;
pub mod utils;
pub mod requests;
pub mod errors;

pub const RETRY_BASE_DELAY: Duration = Duration::from_secs(2);

pub static GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
pub static UPLOAD_BASE_URL: &str = "https://generativelanguage.googleapis.com/upload/v1beta";

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;