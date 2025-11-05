use crate::{
    HANDLE, HWND, INT_PTR, LPARAM, LRESULT, MSG, OSVERSIONINFOA, OSVERSIONINFOW, PAINTSTRUCT,
    RAWINPUTDEVICE, RAWINPUTDEVICELIST, UINT, WNDCLASSEXW, WNDCLASSW, WPARAM,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateDialog, DefWindowProc, DialogBox, SetWindowLong, DWL_MSGRESULT, FALSE, RAWINPUT, TRUE,
    WM_CHARTOITEM, WM_COMPAREITEM, WM_CTLCOLORBTN, WM_CTLCOLORDLG, WM_CTLCOLOREDIT,
    WM_CTLCOLORLISTBOX, WM_CTLCOLORSCROLLBAR, WM_CTLCOLORSTATIC, WM_INITDIALOG, WM_QUERYDRAGICON,
    WM_VKEYTOITEM,
};

/// Application-defined callback function used with the [`CreateDialog`] and [`DialogBox`] families
/// of functions. It processes messages sent to a modal or modeless dialog box. The [`DLGPROC`]
/// type defines a pointer to this callback function. `DialogProc` is a placeholder for the
/// application-defined function name.
///
/// # Parameters
///  * `unnamed_param1` - A handle to the dialog box.
///  * `unnamed_param2` - The message.
///  * `unnamed_param3` - Additional message-specific information.
///  * `unnamed_param4` - Additional message-specific information.
///
/// # Return Value
/// Typically, the dialog box procedure should return [`TRUE`] if it processed the message, and
/// [`FALSE`] if it did not. If the dialog box procedure returns [`FALSE`], the dialog manager
/// performs the default dialog operation in response to the message.
///
/// If the dialog box procedure processes a message that requires a specific return value, the
/// dialog box procedure should set the desired return value by calling
/// `SetWindowLong(hwndDlg, DWL_MSGRESULT, lResult)` immediately before returning [`TRUE`]. Note
/// that you must call [`SetWindowLong`] immediately before returning TRUE; doing so earlier may
/// result in the [`DWL_MSGRESULT`] value being overwritten by a nested dialog box message.
///
/// The following messages are exceptions to the general rules stated above. Consult the
/// documentation for the specific message for details on the semantics of the return value.
///  - [`WM_CHARTOITEM`]
///  - [`WM_COMPAREITEM`]
///  - [`WM_CTLCOLORBTN`]
///  - [`WM_CTLCOLORDLG`]
///  - [`WM_CTLCOLOREDIT`]
///  - [`WM_CTLCOLORLISTBOX`]
///  - [`WM_CTLCOLORSCROLLBAR`]
///  - [`WM_CTLCOLORSTATIC`]
///  - [`WM_INITDIALOG`]
///  - [`WM_QUERYDRAGICON`]
///  - [`WM_VKEYTOITEM`]
///
/// # Remarks
/// You should use the dialog box procedure only if you use the dialog box class for the dialog
/// box. This is the default class and is used when no explicit class is specified in the dialog
/// box template. Although the dialog box procedure is similar to a window procedure, it must not
/// call the [`DefWindowProc`] function to process unwanted messages. Unwanted messages are
/// processed internally by the dialog box window procedure.
pub type DLGPROC = extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> INT_PTR;

/// A handle to a [`RAWINPUT`].
pub type HRAWINPUT = HANDLE;

/// A pointer to a [`MSG`]
pub type LPMSG = *mut MSG;

/// A pointer to an [`OSVERSIONINFOA`]
pub type LPOSVERSIONINFOA = *mut OSVERSIONINFOA;

/// A pointer to an [`OSVERSIONINFOW`]
pub type LPOSVERSIONINFOW = *mut OSVERSIONINFOW;

/// A pointer to a [`PAINTSTRUCT`]
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;

/// A pointer to a [`WNDCLASSEXW`]
pub type LPWNDCLASSEXW = *mut WNDCLASSEXW;

/// A pointer to a [`WNDCLASSW`]
pub type LPWNDCLASSW = *mut WNDCLASSW;

/// A constant pointer to a [`RAWINPUTDEVICE`]
pub type PCRAWINPUTDEVICE = *const RAWINPUTDEVICE;

/// A pointer to a [`RAWINPUT`]
pub type PRAWINPUT = *mut RAWINPUT;

/// A pointer to a [`RAWINPUTDEVICELIST`]
pub type PRAWINPUTDEVICELIST = *mut RAWINPUTDEVICELIST;

/// A callback function, which you define in your application, that processes messages sent to a
/// window. The [`WNDPROC`] type defines a pointer to this callback function.
pub type WNDPROC =
    unsafe extern "system" fn(wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;
