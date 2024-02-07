//! Raw Win32 bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(c_variadic)]

pub mod basetsd;
pub mod intsafe;
pub mod winbase;
pub mod windef;
pub mod winerror;
pub mod winnt;

pub use basetsd::*;
pub use intsafe::*;
pub use winbase::*;
pub use windef::*;
pub use winerror::*;
pub use winnt::*;
