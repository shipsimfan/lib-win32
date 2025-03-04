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
pub use structures::{POINT, RECT};
pub use types::{
    ATOM, BOOL, BYTE, HBRUSH, HCURSOR, HDC, HGLOBAL, HICON, HINSTANCE, HKEY, HLOCAL, HMENU,
    HMODULE, HMONITOR, HWND, INT, LPARAM, LPBYTE, LPCVOID, LPDWORD, LPINT, LPRECT, LPVOID, LRESULT,
    PDWORD, PHKEY, PULONG, UINT, ULONG, WORD, WPARAM,
};
