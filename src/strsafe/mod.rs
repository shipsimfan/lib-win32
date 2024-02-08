//! This header is used by Menus and Other Resources

mod constants;
mod functions;
mod types;

pub use constants::*;
pub use functions::{StringCchPrintf, StringCchPrintfW};
pub use types::{STRSAFE_LPCWSTR, STRSAFE_LPWSTR};
