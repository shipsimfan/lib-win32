use crate::{DLGPROC, HINSTANCE, HWND, LPARAM, LPCWSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateDialogParam, CreateWindowEx, DestroyWindow, GetLastError, IsDialogMessage, ShowWindow,
    DS_SETFONT, DS_SHELLFONT, MAKEINTRESOURCE, WM_INITDIALOG, WM_SETFONT, WS_VISIBLE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// Creates a modeless dialog box from a dialog box template resource. Before displaying the
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
    ///                      resource identifier, its high-order word must be zero and low-order
    ///                      word must contain the identifier. You can use the [`MAKEINTRESOURCE`]
    ///                      macro to create this value.
    ///  * `wnd_parent` - A handle to the window that owns the dialog box.
    ///  * `dialog_func` - A pointer to the dialog box procedure.
    ///  * `init_param` - The value to be passed to the dialog box procedure in the `l_param`
    ///                   parameter in the [`WM_INITDIALOG`] message.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the window handle to the dialog box.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// The [`CreateDialogParam`] function uses the [`CreateWindowEx`] function to create the
    /// dialog box. [`CreateDialogParam`] then sends a [`WM_INITDIALOG`] message (and a
    /// [`WM_SETFONT`] message if the template specifies the [`DS_SETFONT`] or [`DS_SHELLFONT`]
    /// style) to the dialog box procedure. The function displays the dialog box if the template
    /// specifies the [`WS_VISIBLE`] style. Finally, [`CreateDialogParam`] returns the window
    /// handle of the dialog box.
    ///
    /// After [`CreateDialogParam`] returns, the application displays the dialog box (if it is not
    /// already displayed) using the [`ShowWindow`] function. The application destroys the dialog
    /// box by using the [`DestroyWindow`] function. To support keyboard navigation and other
    /// dialog box functionality, the message loop for the dialog box must call the
    /// [`IsDialogMessage`] function.
    pub fn CreateDialogParamW(
        instance: HINSTANCE,
        template_name: LPCWSTR,
        wnd_parent: HWND,
        dialog_func: DLGPROC,
        init_param: LPARAM,
    ) -> HWND;
}
