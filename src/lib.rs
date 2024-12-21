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
pub mod consoleapi2;
pub mod errhandlingapi;
pub mod fileapi;
pub mod guiddef;
pub mod handleapi;
pub mod heapapi;
pub mod intsafe;
pub mod libloaderapi;
pub mod memoryapi;
pub mod processenv;
pub mod synchapi;
pub mod sysinfoapi;
pub mod timezoneapi;
pub mod winbase;
pub mod wincontypes;
pub mod windef;
pub mod winerror;
pub mod winnls;
pub mod winnt;
pub mod winreg;
pub mod winuser;

pub use basetsd::*;
pub use consoleapi2::*;
pub use errhandlingapi::*;
pub use fileapi::*;
pub use guiddef::*;
pub use handleapi::*;
pub use heapapi::*;
pub use intsafe::*;
pub use libloaderapi::*;
pub use memoryapi::*;
pub use processenv::*;
pub use shlwapi::*;
pub use synchapi::*;
pub use sysinfoapi::*;
pub use timezoneapi::*;
pub use winbase::*;
pub use wincontypes::*;
pub use windef::*;
pub use winerror::*;
pub use winnls::*;
pub use winnt::*;
pub use winreg::*;
pub use winuser::*;

// Utilities for easier working with the raw bindings
mod utility;

pub use utility::*;
