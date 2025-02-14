use std::time::Duration;

pub mod engine;
pub mod libs;
pub mod requests;
pub mod utils;

pub static ASSEMBLYAI_BASE_URL: &str = "https://api.assemblyai.com/v2";
pub static SPEECH_ACCEPT_MODEL: [&str; 2] = ["best", "nano"];

pub const RETRY_BASE_DELAY: Duration = Duration::from_secs(2);

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;