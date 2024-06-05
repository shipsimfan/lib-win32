//! Windows Definitions
//!
//! This header is used by multiple technologies such as:
//!  - Display Devices Reference
//!  - High DPI
//!  - Windows Accessibility Features
//!  - Windows and Messages
//!  - Windows GDI

mod constants;
mod structures;
mod types;

pub use constants::*;
pub use structures::POINT;
pub use types::{
    ATOM, BOOL, BYTE, HBRUSH, HCURSOR, HGLOBAL, HICON, HINSTANCE, HKEY, HLOCAL, HMODULE, HWND, INT,
    LPARAM, LPBYTE, LPCVOID, LPDWORD, LPINT, LPVOID, LRESULT, PDWORD, PHKEY, UINT, ULONG, WORD,
    WPARAM,
};
