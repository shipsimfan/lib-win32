// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateDialog, CreateDialogParam, CreateWindowEx, DestroyWindow, IsDialogMessage, ShowWindow,
    DS_SETFONT, DS_SHELLFONT, MAKEINTRESOURCE, WM_INITDIALOG, WM_SETFONT, WS_VISIBLE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Creates a modeless dialog box from a dialog box template resource. The [`CreateDialog`] macro
/// uses the [`CreateDialogParam`] function.
///
/// # Parameters
///  * `instance` - A handle to the module which contains the dialog box template. If this
///                 parameter is [`null_mut`], then the current executable is used.
///  * `name` - The dialog box template. This parameter is either the pointer to a null-terminated
///             character string that specifies the name of the dialog box template or an integer
///             value that specifies the resource identifier of the dialog box template. If the
///             parameter specifies a resource identifier, its high-order word must be zero and its
///             low-order word must contain the identifier. You can use the [`MAKEINTRESOURCE`]
///             macro to create this value.
///  * `wnd_parent` - A handle to the window that owns the dialog box.
///  * `dialog_func` - A pointer to the dialog box procedure.
///
/// # Remarks
/// The [`CreateDialog`] function uses the [`CreateWindowEx`] function to create the dialog box.
/// [`CreateDialog`] then sends a [`WM_INITDIALOG`] message (and a [`WM_SETFONT`] message if the
/// template specifies the [`DS_SETFONT`] or [`DS_SHELLFONT`] style) to the dialog box procedure.
/// The function displays the dialog box if the template specifies the [`WS_VISIBLE`] style.
/// Finally, [`CreateDialog`] returns the window handle to the dialog box.
///
/// After [`CreateDialog`] returns, the application displays the dialog box (if it is not already
/// displayed) by using the [`ShowWindow`] function. The application destroys the dialog box by
/// using the [`DestroyWindow`] function. To support keyboard navigation and other dialog box
/// functionality, the message loop for the dialog box must call the [`IsDialogMessage`] function.
#[macro_export]
macro_rules! CreateDialogA {
    (
        $instance: expr,
        $name: expr,
        $wnd_parent: expr,
        $dialog_func: expr
    ) => {
        $crate::CreateDialogParamA($instance, $name, $wnd_parent, $dialog_func, 0);
    };
}
