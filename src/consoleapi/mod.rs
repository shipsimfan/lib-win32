//! Utilities for interacting with consoles

mod constants;
mod functions;

pub use constants::*;
pub use functions::{ReadConsoleInput, SetConsoleMode};
