use crate::{DLGPROC, HINSTANCE, HWND, INT_PTR, LPARAM, LPCSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateWindowEx, DialogBoxParam, EndDialog, GetLastError, DS_SETFONT, DS_SHELLFONT,
    MAKEINTRESOURCE, WM_INITDIALOG, WM_SETFONT, WS_VISIBLE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Creates a modal dialog box from a dialog box template resource. Before displaying the
    /// dialog box, the function passes an application-defined value to the dialog box procedure as
    /// the `l_param` parameter of the [`WM_INITDIALOG`] message. An application can use this value
    /// to initialize dialog box controls.
    ///
    /// # Parameters
    ///  * `instance` - A handle to the module which contains the dialog box template. If this
    ///                 parameter is [`null_mut`], then the current executable is used.
    ///  * `template_name` - The dialog box template. This parameter is either the pointer to a
    ///                      null-terminated character string that specifies the name of the dialog
    ///                      box template or an integer value that specifies the resource
    ///                      identifier of the dialog box template. If the parameter specifies a
    ///                      resource identifier, its high-order word must be zero and its
    ///                      low-order word must contain the identifier. You can use the
    ///                      [`MAKEINTRESOURCE`] macro to create this value.
    ///  * `wnd_parent` - A handle to the window that owns the dialog box.
    ///  * `dialog_func` - A pointer to the dialog box procedure.
    ///  * `init_param` - The value to pass to the dialog box in the `l_param` parameter of the
    ///                   [`WM_INITDIALOG`] message.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the value of the `result` parameter specified
    /// in the call to the [`EndDialog`] function used to terminate the dialog box.
    ///
    /// If the function fails because the `wnd_parent` parameter is invalid, the return value is
    /// zero. The function returns zero in this case for compatibility with previous versions of
    /// Windows. If the function fails for any other reason, the return value is –1. To get
    /// extended error information, call [`GetLastError`].
    ///
    /// # Remarks
    /// The [`DialogBoxParam`] function uses the [`CreateWindowEx`] function to create the dialog
    /// box. [`DialogBoxParam`] then sends a [`WM_INITDIALOG`] message (and a [`WM_SETFONT`]
    /// message if the template specifies the [`DS_SETFONT`] or [`DS_SHELLFONT`] style) to the
    /// dialog box procedure. The function displays the dialog box (regardless of whether the
    /// template specifies the [`WS_VISIBLE`] style), disables the owner window, and starts its own
    /// message loop to retrieve and dispatch messages for the dialog box.
    ///
    /// When the dialog box procedure calls the [`EndDialog`] function, [`DialogBoxParam`] destroys
    /// the dialog box, ends the message loop, enables the owner window (if previously enabled),
    /// and returns the `result` parameter specified by the dialog box procedure when it called
    /// [`EndDialog`].
    pub fn DialogBoxParamA(
        instance: HINSTANCE,
        template_name: LPCSTR,
        wnd_parent: HWND,
        dialog_func: DLGPROC,
        init_param: LPARAM,
    ) -> INT_PTR;
}
