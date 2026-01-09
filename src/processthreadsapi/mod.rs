//! API for processes and threads

mod functions;

pub use functions::{GetCurrentProcess, GetCurrentThreadId, OpenProcessToken};
