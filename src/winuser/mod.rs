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
    AllowSetForegroundWindow, BeginPaint, CallWindowProc, CallWindowProcA, CallWindowProcW,
    CreateDialogParam, CreateDialogParamA, CreateDialogParamW, CreateWindowEx, CreateWindowExA,
    CreateWindowExW, DefWindowProc, DefWindowProcA, DefWindowProcW, DestroyWindow, DialogBoxParam,
    DialogBoxParamA, DialogBoxParamW, DispatchMessage, DispatchMessageA, DispatchMessageW,
    EnableWindow, EndDialog, EndPaint, GetClassInfo, GetClassInfoA, GetClassInfoEx,
    GetClassInfoExA, GetClassInfoExW, GetClassInfoW, GetClientRect, GetSubMenu, GetVersionEx,
    GetVersionExA, GetVersionExW, GetWindowLong, GetWindowLongA, GetWindowLongPtr,
    GetWindowLongPtrA, GetWindowLongPtrW, GetWindowLongW, IsDialogMessage, IsDialogMessageA,
    IsDialogMessageW, LockSetForegroundWindow, MessageBox, MessageBoxA, MessageBoxEx,
    MessageBoxExA, MessageBoxExW, MessageBoxW, PeekMessage, PeekMessageA, PeekMessageW,
    PostQuitMessage, RegisterClass, RegisterClassA, RegisterClassEx, RegisterClassExA,
    RegisterClassExW, RegisterClassW, SetActiveWindow, SetForegroundWindow, SetLastErrorEx,
    SetParent, SetWindowLong, SetWindowLongA, SetWindowLongPtr, SetWindowLongPtrA,
    SetWindowLongPtrW, SetWindowLongW, SetWindowPos, ShowWindow, SystemParametersInfo,
    SystemParametersInfoA, SystemParametersInfoForDpi, SystemParametersInfoW, TranslateMessage,
    UnregisterClass, UnregisterClassA, UnregisterClassW,
};
pub use macros::{CreateDialog, CreateWindow, DialogBox, MAKEINTRESOURCE};
pub use structures::{
    CLIENTCREATESTRUCT, CREATESTRUCT, CREATESTRUCTA, CREATESTRUCTW, HIGHCONTRAST, HIGHCONTRASTA,
    HIGHCONTRASTW, MDICREATESTRUCT, MDICREATESTRUCTA, MDICREATESTRUCTW, MINIMIZEDMETRICS, MSG,
    NONCLIENTMETRICS, NONCLIENTMETRICSA, NONCLIENTMETRICSW, OSVERSIONINFO, OSVERSIONINFOA,
    OSVERSIONINFOEX, OSVERSIONINFOEXA, OSVERSIONINFOEXW, OSVERSIONINFOW, PAINTSTRUCT, WNDCLASS,
    WNDCLASSA, WNDCLASSEX, WNDCLASSEXA, WNDCLASSEXW, WNDCLASSW,
};
pub use types::{
    DLGPROC, LPMSG, LPOSVERSIONINFOA, LPOSVERSIONINFOW, LPPAINTSTRUCT, LPWNDCLASSEXW, LPWNDCLASSW,
    WNDPROC,
};
