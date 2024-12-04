//! This header is used by multiple technologies. For more information, see:
//!  - Data Access and Storage
//!  - System Services

mod constants;
mod functions;

pub use constants::*;
pub use functions::{CreateFile, CreateFileW, FlushFileBuffers, ReadFile, WriteFile};
