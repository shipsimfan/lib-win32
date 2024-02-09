//! This header is used by System Services

mod constants;
mod functions;

pub use constants::*;
pub use functions::{VirtualAlloc, VirtualAllocEx, VirtualProtect};
