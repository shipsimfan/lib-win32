//! Definitions for the I/O API

mod functions;

pub use functions::{CreateIoCompletionPort, GetOverlappedResult, GetQueuedCompletionStatus};
