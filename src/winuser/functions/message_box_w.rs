use crate::{HWND, LPCWSTR, UINT};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, MessageBox, IDABORT, IDCANCEL, IDCONTINUE, IDIGNORE, IDNO, IDOK, IDRETRY,
    IDTRYAGAIN, IDYES, MB_ABORTRETRYIGNORE, MB_APPLMODAL, MB_CANCELTRYCONTINUE,
    MB_DEFAULT_DESKTOP_ONLY, MB_DEFBUTTON1, MB_DEFBUTTON2, MB_DEFBUTTON3, MB_DEFBUTTON4, MB_HELP,
    MB_ICONASTERISK, MB_ICONERROR, MB_ICONEXCLAMATION, MB_ICONHAND, MB_ICONINFORMATION,
    MB_ICONQUESTION, MB_ICONSTOP, MB_ICONWARNING, MB_OK, MB_OKCANCEL, MB_RETRYCANCEL, MB_RIGHT,
    MB_RTLREADING, MB_SERVICE_NOTIFICATION, MB_SETFOREGROUND, MB_SYSTEMMODAL, MB_TASKMODAL,
    MB_TOPMOST, MB_YESNO, MB_YESNOCANCEL, WM_HELP, WS_EX_TOPMOST,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// Displays a modal dialog box that contains a system icon, a set of buttons, and a brief
    /// application-specific message, such as status or error information. The message box returns
    /// an integer value that indicates which button the user clicked.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the owner window of the message box to be created. If this parameter
    ///            is [`null_mut`], the message box has no owner window.
    ///  * `text` - The message to be displayed. If the string consists of more than one line, you
    ///             can separate the lines using a carriage return and/or linefeed character
    ///             between each line.
    ///  * `caption` - The dialog box title. If this parameter is [`null_mut`], the default title
    ///                is "Error".
    ///  * `r#type` - The contents and behavior of the dialog box. This parameter can be a
    ///               combination of flags from the following:
    ///    * [`MB_ABORTRETRYIGNORE`] - The message box contains three push buttons: "Abort",
    ///                                "Retry", and "Ignore".
    ///    * [`MB_CANCELTRYCONTINUE`] - The message box contains three push buttons: "Cancel",
    ///                                 "Try Again", "Continue". Use this message box type instead
    ///                                 of [`MB_ABORTRETRYIGNORE`].
    ///    * [`MB_HELP`] - Adds a "Help" button to the message box. When the user clicks the "Help"
    ///                    button or presses F1, the system sends a [`WM_HELP`] message to the
    ///                    owner.
    ///    * [`MB_OK`] - The message box contains one push button: "OK". This is the default.
    ///    * [`MB_OKCANCEL`] - The message box contains two push buttons: "OK" and "Cancel".
    ///    * [`MB_RETRYCANCEL`] - The message box contains two push buttons: "Retry" and "Cancel".
    ///    * [`MB_YESNO`] - The message box contains two push buttons: "Yes" and "No".
    ///    * [`MB_YESNOCANCEL`] - The message box contains three push buttons: "Yes", "No", and
    ///                           "Cancel".
    ///    * [`MB_ICONEXCLAMATION`] - An exclamation-point icon appears in the message box.
    ///    * [`MB_ICONWARNING`] - An exclamation-point icon appears in the message box.
    ///    * [`MB_ICONINFORMATION`] - An icon consisting of a lowercase letter 'i' in a circle
    ///                               appears in the message box.
    ///    * [`MB_ICONASTERISK`] - An icon consisting of a lowercase letter 'i' in a circle appears
    ///                            in the message box.
    ///    * [`MB_ICONQUESTION`] - A question-mark icon appears in the message box. The
    ///                            question-mark message icon is no longer recommended because it
    ///                            does not clearly represent a specific type of message and
    ///                            because the phrasing of a message as a question could apply to
    ///                            any message type. In addition, users can confuse the message
    ///                            symbol question mark with "Help" information. Therefore, do not
    ///                            use this question mark message symbol in your message boxes. The
    ///                            system continues to support its inclusion only for backward
    ///                            compatibility.
    ///    * [`MB_ICONSTOP`] - A stop-sign icon appears in the message box.
    ///    * [`MB_ICONERROR`] - A stop-sign icon appears in the message box.
    ///    * [`MB_ICONHAND`] - A stop-sign icon appears in the message box.
    ///    * [`MB_DEFBUTTON1`] - The first button is the default button. [`MB_DEFBUTTON1`] is the
    ///                          default unless [`MB_DEFBUTTON2`], [`MB_DEFBUTTON3`], or
    ///                          [`MB_DEFBUTTON4`] is specified.
    ///    * [`MB_DEFBUTTON2`] - The second button is the default button.
    ///    * [`MB_DEFBUTTON3`] - The third button is the default button.
    ///    * [`MB_DEFBUTTON4`] - The fourth button is the default button.
    ///    * [`MB_APPLMODAL`] - The user must respond to the message box before continuing work in
    ///                         the window identified by the `wnd` parameter. However, the user can
    ///                         move to the windows of other threads and work in those windows.
    ///                         Depending on the hierarchy of windows in the application, the user
    ///                         may be able to move to other windows within the thread. All child
    ///                         windows of the parent of the message box are automatically
    ///                         disabled, but pop-up windows are not. [`MB_APPLMODAL`] is the
    ///                         default if neither [`MB_SYSTEMMODAL`] nor [`MB_TASKMODAL`] is
    ///                         specified.
    ///    * [`MB_SYSTEMMODAL`] - Same as [`MB_APPLMODAL`] except that the message box has the
    ///                           [`WS_EX_TOPMOST`] style. Use system-modal message boxes to notify
    ///                           the user of serious, potentially damaging errors that require
    ///                           immediate attention (for example, running out of memory). This
    ///                           flag has no effect on the user's ability to interact with windows
    ///                           other than those associated with `wnd`.
    ///    * [`MB_TASKMODAL`] - Same as [`MB_APPLMODAL`] except that all the top-level windows
    ///                         belonging to the current thread are disabled if the `wnd` parameter
    ///                         is [`null_mut`]. Use this flag when the calling application or
    ///                         library does not have a window handle available but still needs to
    ///                         prevent input to other windows in the calling thread without
    ///                         suspending other threads.
    ///    * [`MB_DEFAULT_DESKTOP_ONLY`] - Same as desktop of the interactive window station. If
    ///                                    the current input desktop is not the default desktop,
    ///                                    [`MessageBox`] does not return until the user switches
    ///                                    to the default desktop.
    ///    * [`MB_RIGHT`] - The text is right-justified.
    ///    * [`MB_RTLREADING`] - Displays message and caption text using right-to-left reading
    ///                          order on Hebrew and Arabic systems.
    ///    * [`MB_SETFOREGROUND`] - The message box becomes the foreground window. Internally, the
    ///                             system calls the [`SetForegroundWindow`] function for the
    ///                             message box.
    ///    * [`MB_TOPMOST`] - The message box is created with the [`WS_EX_TOPMOST`] window style.
    ///    * [`MB_SERVICE_NOTIFICATION`] - The caller is a service notifying the user of an event.
    ///                                    The function displays a message box on the current
    ///                                    active desktop, even if there is no user logged on to
    ///                                    the computer. Terminal Services: If the calling thread
    ///                                    has an impersonation token, the function directs the
    ///                                    message box to the session specified in the
    ///                                    impersonation token. If this flag is set, the `wnd`
    ///                                    parameter must be [`null_mut`]. This is so that the
    ///                                    message box can appear on a desktop other than the
    ///                                    desktop corresponding to the `wnd`.
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
    /// Adding two right-to-left marks (RLMs), represented by Unicode formatting character
    /// `U+200F`, in the beginning of a [`MessageBox`] display string is interpreted by the
    /// [`MessageBox`] rendering engine so as to cause the reading order of the [`MessageBox`] to
    /// be rendered as right-to-left (RTL).
    ///
    /// When you use a system-modal message box to indicate that the system is low on memory, the
    /// strings pointed to by the `text` and `caption` parameters should not be taken from a
    /// resource file because an attempt to load the resource may fail.
    ///
    /// If you create a message box while a dialog box is present, use a handle to the dialog box
    /// as the `wnd` parameter. The `wnd` parameter should not identify a child window, such as a
    /// control in a dialog box.
    pub fn MessageBoxW(wnd: HWND, text: LPCWSTR, caption: LPCWSTR, r#type: UINT) -> c_int;
}
