//! Raw Win32 bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(c_variadic)]
#![feature(c_size_t)]

// Header not automatically included with `Windows.h`
pub mod ktmw32;
pub mod shlwapi;
pub mod string;
pub mod strsafe;
pub mod winsock2;

// Headers automatically included with `Windows.h`
pub mod basetsd;
pub mod errhandlingapi;
pub mod guiddef;
pub mod handleapi;
pub mod heapapi;
pub mod intsafe;
pub mod libloaderapi;
pub mod memoryapi;
pub mod synchapi;
pub mod sysinfoapi;
pub mod timezoneapi;
pub mod winbase;
pub mod windef;
pub mod winerror;
pub mod winnt;
pub mod winreg;
pub mod winuser;

pub use timezoneapi::*;
pub use basetsd::*;
pub use errhandlingapi::*;
pub use guiddef::*;
pub use handleapi::*;
pub use heapapi::*;
pub use intsafe::*;
pub use libloaderapi::*;
pub use memoryapi::*;
pub use shlwapi::*;
pub use synchapi::*;
pub use sysinfoapi::*;
pub use winbase::*;
pub use windef::*;
pub use winerror::*;
pub use winnt::*;
pub use winreg::*;
pub use winuser::*;

// Utilities for easier working with the raw bindings
mod utility;

pub use utility::*;
