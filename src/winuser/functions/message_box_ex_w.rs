use crate::{HWND, LPCWSTR, UINT, WORD};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, MessageBox, MessageBoxEx, IDABORT, IDCANCEL, IDCLOSE, IDCONTINUE, IDHELP,
    IDIGNORE, IDNO, IDOK, IDRETRY, IDTRYAGAIN, IDYES, MAKELANGID,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// Creates, displays, and operates a message box. The message box contains an
    /// application-defined message and title, plus any combination of predefined icons and push
    /// buttons. The buttons are in the language of the system user interface.
    ///
    /// Currently [`MessageBoxEx`] and [`MessageBox`] work the same way.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the owner window of the message box to be created. If this parameter
    ///            is [`null_mut`], the message box has no owner window.
    ///  * `text` - The message to be displayed.
    ///  * `caption` - The dialog box title. If this parameter is [`null_mut`], the default title
    ///                "Error" is used.
    ///  * `r#type` - The contents and behavior of the dialog box. For information on the supported
    ///               flags, see [`MessageBox`].
    ///  * `language_id` - The language for the text displayed in the message box button(s).
    ///                    Specifying a value of zero (0) indicates to display the button text in
    ///                    the default system language. If this parameter is
    ///                    `MAKELANGID(LANG_NEUTRAL, SUBLANG_NEUTRAL)`, the current language
    ///                    associated with the calling thread is used. To specify a language other
    ///                    than the current language, use the [`MAKELANGID`] macro to create this
    ///                    parameter. For more information, see [`MAKELANGID`].
    ///
    /// # Return Value
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// If the function succeeds, the return value is one of the following menu-item values:
    ///  * [`IDABORT`] - The "Abort" button was selected.
    ///  * [`IDCANCEL`] - The "Cancel" button was selected.
    ///  * [`IDCONTINUE`] - The "Continue" button was selected.
    ///  * [`IDIGNORE`] - The "Ignore" button was selected.
    ///  * [`IDNO`] - The "No" button was selected.
    ///  * [`IDOK`] - The "OK" button was selected.
    ///  * [`IDRETRY`] - The "Retry" button was selected.
    ///  * [`IDTRYAGAIN`] - The "Try Again" button was selected.
    ///  * [`IDYES`] - The "Yes" button was selected.
    ///
    /// # Remarks
    /// When you use a system-modal message box to indicate that the system is low on memory, the
    /// strings pointed to by the `text` and `caption` parameters should not be taken from a
    /// resource file because an attempt to load the resource may fail.
    ///
    /// If you create a message box while a dialog box is present, use a handle to the dialog box
    /// as the `wnd` parameter. The `wnd` parameter should not identify a child window, such as a
    /// control in a dialog box.
    pub fn MessageBoxExW(
        wnd: HWND,
        text: LPCWSTR,
        caption: LPCWSTR,
        r#type: UINT,
        language_id: WORD,
    ) -> c_int;
}
