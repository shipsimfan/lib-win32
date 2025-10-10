//! Raw Win32 bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

// Header not automatically included with `Windows.h`
#[cfg(feature = "d3d10")]
pub mod d3d10;
#[cfg(feature = "d3d11")]
pub mod d3d11;
#[cfg(feature = "d3d11")]
pub mod d3d11_1;
#[cfg(feature = "d3d11")]
pub mod d3d11_2;
#[cfg(feature = "d3d11")]
pub mod d3d11_3;
#[cfg(feature = "d3dcommon")]
pub mod d3dcommon;
#[cfg(feature = "dxgi")]
pub mod dxgi;
#[cfg(feature = "dxgi")]
pub mod dxgi1_2;
#[cfg(feature = "dxgi")]
pub mod dxgi1_3;
#[cfg(feature = "dxgi")]
pub mod dxgi1_4;
#[cfg(feature = "dxgi")]
pub mod dxgi1_5;
#[cfg(feature = "dxgi")]
pub mod dxgi_debug;
pub mod ktmw32;
pub mod shlwapi;
pub mod string;
pub mod strsafe;
pub mod unknwn;
pub mod winsock2;

// Headers automatically included with `Windows.h`
pub mod basetsd;
pub mod combaseapi;
pub mod consoleapi;
pub mod consoleapi2;
pub mod errhandlingapi;
pub mod fileapi;
pub mod guiddef;
pub mod handleapi;
pub mod heapapi;
pub mod intsafe;
pub mod ioapiset;
pub mod libloaderapi;
pub mod memoryapi;
pub mod ntdef;
pub mod processenv;
pub mod profileapi;
pub mod synchapi;
pub mod sysinfoapi;
pub mod timezoneapi;
pub mod winbase;
pub mod wincon;
pub mod wincontypes;
pub mod windef;
pub mod winerror;
pub mod wingdi;
pub mod winnls;
pub mod winnt;
pub mod winreg;
pub mod winuser;

pub use basetsd::*;
pub use combaseapi::*;
pub use consoleapi::*;
pub use consoleapi2::*;
pub use errhandlingapi::*;
pub use fileapi::*;
pub use guiddef::*;
pub use handleapi::*;
pub use heapapi::*;
pub use intsafe::*;
pub use ioapiset::*;
pub use libloaderapi::*;
pub use memoryapi::*;
pub use ntdef::*;
pub use processenv::*;
pub use profileapi::*;
pub use shlwapi::*;
pub use synchapi::*;
pub use sysinfoapi::*;
pub use timezoneapi::*;
pub use winbase::*;
pub use wincon::*;
pub use wincontypes::*;
pub use windef::*;
pub use winerror::*;
pub use wingdi::*;
pub use winnls::*;
pub use winnt::*;
pub use winreg::*;
pub use winuser::*;

// Utilities for easier working with the raw bindings
mod utility;

pub use utility::*;
