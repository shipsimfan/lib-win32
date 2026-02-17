use crate::{
    BOOL, HANDLE, HDC, HMONITOR, HWND, INT_PTR, LPARAM, LPRECT, LRESULT, MONITORINFO, MSG,
    OSVERSIONINFOA, OSVERSIONINFOW, PAINTSTRUCT, RAWINPUTDEVICE, RAWINPUTDEVICELIST, UINT,
    WNDCLASSEXW, WNDCLASSW, WPARAM,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateDialog, DWL_MSGRESULT, DefWindowProc, DialogBox, EnumDisplayMonitors, FALSE, RAWINPUT,
    RECT, SetWindowLong, TRUE, WM_CHARTOITEM, WM_COMPAREITEM, WM_CTLCOLORBTN, WM_CTLCOLORDLG,
    WM_CTLCOLOREDIT, WM_CTLCOLORLISTBOX, WM_CTLCOLORSCROLLBAR, WM_CTLCOLORSTATIC, WM_INITDIALOG,
    WM_QUERYDRAGICON, WM_VKEYTOITEM,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

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
pub type DLGPROC = unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> INT_PTR;

/// A handle to a [`RAWINPUT`].
pub type HRAWINPUT = HANDLE;

/// A pointer to a [`MONITORINFO`]
pub type LPMONITORINFO = *mut MONITORINFO;

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

/// An application-defined callback function used with the [`EnumDisplayMonitors`] function. The
/// [`MONITORENUMPROC`] type defines a pointer to this callback function.
///
/// # Parameters
///  * `unnamed_param1` - A handle to the display monitor. This value will always not be
///                       [`null_mut`]. This parameter is typically named `monitor`.
///  * `unnamed_param2` - A handle to a device context. This parameter is typically named
///                       `hdc_monitor`. The device context has color attributes that are
///                       appropriate for the display monitor identified by `monitor`. The clipping
///                       area of the device context is set to the intersection of the visible
///                       region of the device context identified by the `hdc` parameter of
///                       [`EnumDisplayMonitors`], the rectangle pointed to by the `clip` parameter
///                       of [`EnumDisplayMonitors`], and the display monitor rectangle. This value
///                       is [`null_mut`] if the `hdc` parameter of [`EnumDisplayMonitors`] was
///                       [`null_mut`].
///  * `unnamed_param3` - A pointer to a [`RECT`] structure. This parameter is typically named
///                       `rect_monitor`. If `hdc_monitor` is not [`null_mut`], this rectangle is
///                       the intersection of the clipping area of the device context identified by
///                       `hdc_monitor` and the display monitor rectangle. The rectangle
///                       coordinates are device-context coordinates. If `hdc_monitor` is
///                       [`null_mut`], this rectangle is the display monitor rectangle. The
///                       rectangle coordinates are virtual-screen coordinates.
///  * `unnamed_param4` - Application-defined data that [`EnumDisplayMonitors`] passes directly to
///                       the enumeration function. This parameter is typically named `data`.
///
/// # Return Value
/// To continue the enumeration, return [`TRUE`].
///
/// To stop the enumeration, return [`FALSE`].
///
/// # Remarks
/// You can use the [`EnumDisplayMonitors`] function to enumerate the set of display monitors that
/// intersect the visible region of a specified device context and, optionally, a clipping
/// rectangle. To do this, set the `hdc` parameter to a not [`null_mut`] value, and set the `clip`
/// parameter as needed.
///
/// You can also use the [`EnumDisplayMonitors`] function to enumerate one or more of the display
/// monitors on the desktop, without supplying a device context. To do this, set the `hdc`
/// parameter of [`EnumDisplayMonitors`] to [`null_mut`] and set the `clip` parameter as needed.
///
/// In all cases, [`EnumDisplayMonitors`] calls a specified [`MONITORENUMPROC`] function once for
/// each display monitor in the calculated enumeration set. The [`MONITORENUMPROC`] function always
/// receives a handle to the display monitor.
///
/// If the `hdc` parameter of [`EnumDisplayMonitors`] is not [`null_mut`], the [`MONITORENUMPROC`]
/// function also receives a handle to a device context whose color format is appropriate for the
/// display monitor. You can then paint into the device context in a manner that is optimal for the
/// display monitor.
pub type MONITORENUMPROC = unsafe extern "system" fn(
    unnamed_param1: HMONITOR,
    unnamed_param2: HDC,
    unnamed_param3: LPRECT,
    unnamed_param4: LPARAM,
) -> BOOL;

/// A constant pointer to a [`RAWINPUTDEVICE`]
pub type PCRAWINPUTDEVICE = *const RAWINPUTDEVICE;

/// A pointer to a [`RAWINPUT`]
pub type PRAWINPUT = *mut RAWINPUT;

/// A pointer to a [`RAWINPUTDEVICE`]
pub type PRAWINPUTDEVICE = *mut RAWINPUTDEVICE;

/// A pointer to a [`RAWINPUTDEVICELIST`]
pub type PRAWINPUTDEVICELIST = *mut RAWINPUTDEVICELIST;

/// A callback function, which you define in your application, that processes messages sent to a
/// window. The [`WNDPROC`] type defines a pointer to this callback function.
pub type WNDPROC =
    unsafe extern "system" fn(wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;
