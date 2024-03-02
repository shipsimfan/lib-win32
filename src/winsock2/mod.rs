//! Windows socket

mod constants;
mod functions;
mod types;

pub use constants::*;
pub use functions::{closesocket, socket, WSAGetLastError};
pub use types::SOCKET;
