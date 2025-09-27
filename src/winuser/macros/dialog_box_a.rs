// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateWindowEx, DialogBox, DialogBoxParam, EndDialog, DLGPROC, DS_SETFONT, DS_SHELLFONT,
    MAKEINTRESOURCE, WM_INITDIALOG, WM_SETFONT, WS_VISIBLE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Creates a modal dialog box from a dialog box template resource. [`DialogBox`] does not return
/// control until the specified callback function terminates the modal dialog box by calling the
/// [`EndDialog`] function.
///
/// [`DialogBox`] is implemented as a call to the [`DialogBoxParam`] function.
///
/// # Parameters
///  * `instance` - A handle to the module which contains the dialog box template. If this
///                 parameter is [`null_mut`], then the current executable is used.
///  * `template` - The dialog box template. This parameter is either the pointer to a
///                 null-terminated character string that specifies the name of the dialog box
///                 template or an integer value that specifies the resource identifier of the
///                 dialog box template. If the parameter specifies a resource identifier, its
///                 high-order word must be zero and its low-order word must contain the
///                 identifier. You can use the [`MAKEINTRESOURCE`] macro to create this value.
///  * `wnd_parent` - A handle to the window that owns the dialog box.
///  * `dialog_func` - A pointer to the dialog box procedure. For more information about the dialog
///                    box procedure, see [`DLGPROC`].
///
/// # Remarks
/// The [`DialogBox`] macro uses the [`CreateWindowEx`] function to create the dialog box.
/// [`DialogBox`] then sends a [`WM_INITDIALOG`] message (and a [`WM_SETFONT`] message if the
/// template specifies the [`DS_SETFONT`] or [`DS_SHELLFONT`] style) to the dialog box procedure.
/// The function displays the dialog box (regardless of whether the template specifies the
/// [`WS_VISIBLE`] style), disables the owner window, and starts its own message loop to retrieve
/// and dispatch messages for the dialog box.
///
/// When the dialog box procedure calls the [`EndDialog`] function, [`DialogBox`] destroys the
/// dialog box, ends the message loop, enables the owner window (if previously enabled), and
/// returns the `result` parameter specified by the dialog box procedure when it called
/// [`EndDialog`].
#[macro_export]
macro_rules! DialogBoxA {
    (
        $instance: expr,
        $template: expr,
        $wnd_parent: expr,
        $dialog_func: expr,
    ) => {
        $crate::DialogBoxParamA($instance, $template, $wnd_parent, $dialog_func, 0)
    };
}
