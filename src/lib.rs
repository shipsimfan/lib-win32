//! Raw Win32 bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(c_variadic)]
#![feature(c_size_t)]

// Header not automatically included with `Windows.h`
pub mod strsafe;

// Headers automatically included with `Windows.h`
pub mod basetsd;
pub mod errhandlingapi;
pub mod handleapi;
pub mod intsafe;
pub mod memoryapi;
pub mod sysinfoapi;
pub mod winbase;
pub mod windef;
pub mod winerror;
pub mod winnt;
pub mod winreg;
pub mod winuser;

pub use basetsd::*;
pub use errhandlingapi::*;
pub use handleapi::*;
pub use intsafe::*;
pub use memoryapi::*;
pub use sysinfoapi::*;
pub use winbase::*;
pub use windef::*;
pub use winerror::*;
pub use winnt::*;
pub use winreg::*;
pub use winuser::*;
