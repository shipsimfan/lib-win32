//! This header is used by Component Object Model (COM)

mod structures;
mod types;

pub use structures::GUID;
pub use types::{IID, LPGUID, REFGUID, REFIID};
