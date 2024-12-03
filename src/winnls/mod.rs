//! Windows Native Language Support

mod constants;
mod functions;

pub use constants::*;
pub use functions::{GetUserDefaultLocaleName, GetUserDefaultUILanguage};
