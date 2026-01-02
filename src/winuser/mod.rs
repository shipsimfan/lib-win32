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
    AdjustWindowRect, AdjustWindowRectEx, AllowSetForegroundWindow, BeginPaint, CallWindowProc,
    CallWindowProcA, CallWindowProcW, ClipCursor, CreateDialogParam, CreateDialogParamA,
    CreateDialogParamW, CreateWindowEx, CreateWindowExA, CreateWindowExW, DefRawInputProc,
    DefWindowProc, DefWindowProcA, DefWindowProcW, DestroyWindow, DialogBoxParam, DialogBoxParamA,
    DialogBoxParamW, DispatchMessage, DispatchMessageA, DispatchMessageW, EnableWindow, EndDialog,
    EndPaint, GetClassInfo, GetClassInfoA, GetClassInfoEx, GetClassInfoExA, GetClassInfoExW,
    GetClassInfoW, GetClientRect, GetMessage, GetMessageA, GetMessageW, GetRawInputBuffer,
    GetRawInputData, GetRawInputDeviceInfo, GetRawInputDeviceInfoA, GetRawInputDeviceInfoW,
    GetRawInputDeviceList, GetRegisteredRawInputDevices, GetSubMenu, GetVersionEx, GetVersionExA,
    GetVersionExW, GetWindowLong, GetWindowLongA, GetWindowLongPtr, GetWindowLongPtrA,
    GetWindowLongPtrW, GetWindowLongW, GetWindowThreadProcessId, IsDialogMessage, IsDialogMessageA,
    IsDialogMessageW, LockSetForegroundWindow, MapWindowPoints, MessageBox, MessageBoxA,
    MessageBoxEx, MessageBoxExA, MessageBoxExW, MessageBoxW, PeekMessage, PeekMessageA,
    PeekMessageW, PostMessage, PostMessageA, PostMessageW, PostQuitMessage, PostThreadMessage,
    PostThreadMessageA, PostThreadMessageW, RegisterClass, RegisterClassA, RegisterClassEx,
    RegisterClassExA, RegisterClassExW, RegisterClassW, RegisterRawInputDevices, ScrollWindowEx,
    SetActiveWindow, SetForegroundWindow, SetLastErrorEx, SetParent, SetWindowDisplayAffinity,
    SetWindowLong, SetWindowLongA, SetWindowLongPtr, SetWindowLongPtrA, SetWindowLongPtrW,
    SetWindowLongW, SetWindowPos, SetWindowText, SetWindowTextA, SetWindowTextW, ShowWindow,
    SystemParametersInfo, SystemParametersInfoA, SystemParametersInfoForDpi, SystemParametersInfoW,
    TranslateMessage, UnregisterClass, UnregisterClassA, UnregisterClassW, WaitMessage,
};
pub use macros::{CreateDialog, CreateWindow, DialogBox, MAKEINTRESOURCE};
pub use structures::{
    CLIENTCREATESTRUCT, CREATESTRUCT, CREATESTRUCTA, CREATESTRUCTW, HIGHCONTRAST, HIGHCONTRASTA,
    HIGHCONTRASTW, MDICREATESTRUCT, MDICREATESTRUCTA, MDICREATESTRUCTW, MINIMIZEDMETRICS, MSG,
    NONCLIENTMETRICS, NONCLIENTMETRICSA, NONCLIENTMETRICSW, OSVERSIONINFO, OSVERSIONINFOA,
    OSVERSIONINFOEX, OSVERSIONINFOEXA, OSVERSIONINFOEXW, OSVERSIONINFOW, PAINTSTRUCT, RAWHID,
    RAWINPUT, RAWINPUTDEVICE, RAWINPUTDEVICELIST, RAWINPUTHEADER, RAWINPUTUNION, RAWKEYBOARD,
    RAWMOUSE, RAWMOUSEUNION, RAWMOUSEUNIONSTRUCT, RID_DEVICE_INFO, RID_DEVICE_INFO_HID,
    RID_DEVICE_INFO_KEYBOARD, RID_DEVICE_INFO_MOUSE, RID_DEVICE_INFO_UNION, WINDOWPOS, WNDCLASS,
    WNDCLASSA, WNDCLASSEX, WNDCLASSEXA, WNDCLASSEXW, WNDCLASSW,
};
pub use types::{
    DLGPROC, HRAWINPUT, LPMSG, LPOSVERSIONINFOA, LPOSVERSIONINFOW, LPPAINTSTRUCT, LPWNDCLASSEXW,
    LPWNDCLASSW, PCRAWINPUTDEVICE, PRAWINPUT, PRAWINPUTDEVICE, PRAWINPUTDEVICELIST, WNDPROC,
};
