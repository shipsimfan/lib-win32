//! Utilities for interacting with consoles

mod constants;
mod functions;
mod types;

pub use constants::*;
pub use functions::{
    ReadConsoleInput, ReadConsoleInputW, SetConsoleCtrlHandler, SetConsoleMode, WriteConsole,
    WriteConsoleW,
};
pub use types::PHANDLER_ROUTINE;
