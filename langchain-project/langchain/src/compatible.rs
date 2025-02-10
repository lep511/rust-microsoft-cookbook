use std::time::Duration;

pub mod chat;
pub mod libs;
pub mod utils;
pub mod requests;

pub const RETRY_BASE_DELAY: Duration = Duration::from_secs(2);

pub const DEBUG_PRE: bool = true;
pub const DEBUG_POST: bool = false;