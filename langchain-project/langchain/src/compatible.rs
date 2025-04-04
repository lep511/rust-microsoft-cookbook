use std::time::Duration;

pub mod chat;
pub mod libs;
pub mod error;
pub mod utils;
pub mod requests;

pub static CHAT_COMPLETION: &str = "chat/completions";
pub static DEFERRED_COMPLETION: &str = "chat/deferred-completion";
pub const RETRY_BASE_DELAY: Duration = Duration::from_secs(2);

pub const DEBUG_PRE: bool = false;
pub const DEBUG_POST: bool = false;