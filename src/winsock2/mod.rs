//! Windows socket

mod constants;
mod functions;
mod types;

pub use constants::*;
pub use functions::{closesocket, socket};
pub use types::SOCKET;
