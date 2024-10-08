//! Windows User Definitions
//!
//! This header is used by multiple technologies such as:
//!  - Data Exchange
//!  - Desktop Window Manager (DWM)
//!  - Developer Notes
//!  - Dialog Boxes
//!  - Display Devices Reference
//!  - High DPI
//!  - Input Feedback Configuration
//!  - Input Source Identification
//!  - Internationalization for Windows Applications
//!  - Keyboard and Mouse Input
//!  - Menus and Other Resources
//!  - Mobile Device Management Settings Provider
//!  - Pointer Device Input Stack
//!  - Pointer Input Messages and Notifications
//!  - Remote Desktop Services
//!  - Security and Identity
//!  - System Services
//!  - The Windows Shell
//!  - Touch Hit Testing
//!  - Touch Injection
//!  - Touch Input
//!  - Window Stations and Desktops
//!  - Windows Accessibility Features
//!  - Windows and Messages
//!  - Windows Controls
//!  - Windows GDI

mod constants;
mod functions;
mod macros;
mod structures;
mod types;

pub use constants::*;
pub use functions::{
    BeginPaint, CallWindowProc, CallWindowProcW, CreateWindowEx, CreateWindowExW, DefWindowProc,
    DefWindowProcW, DestroyWindow, DispatchMessage, DispatchMessageW, EndPaint, GetClassInfo,
    GetClassInfoEx, GetClassInfoExW, GetClassInfoW, GetClientRect, GetWindowLongPtr,
    GetWindowLongPtrW, MessageBox, MessageBoxEx, MessageBoxExW, MessageBoxW, PeekMessage,
    PeekMessageW, PostQuitMessage, RegisterClass, RegisterClassEx, RegisterClassExW,
    RegisterClassW, SetLastErrorEx, SetWindowLongPtr, SetWindowLongPtrW, ShowWindow,
    TranslateMessage, UnregisterClass, UnregisterClassW,
};
pub use macros::{CreateWindow, MAKEINTRESOURCE};
pub use structures::{
    CLIENTCREATESTRUCT, CREATESTRUCT, CREATESTRUCTW, MDICREATESTRUCT, MDICREATESTRUCTW, MSG,
    PAINTSTRUCT, WNDCLASS, WNDCLASSEX, WNDCLASSEXW, WNDCLASSW,
};
pub use types::{LPMSG, LPPAINTSTRUCT, LPWNDCLASSEXW, LPWNDCLASSW, WNDPROC};
