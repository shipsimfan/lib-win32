//! This header is used by System Services

mod functions;
mod types;

pub use functions::{SetWaitableTimer, SleepEx};
pub use types::PTIMERAPCROUTINE;
