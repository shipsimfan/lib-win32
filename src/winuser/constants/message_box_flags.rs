use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::{MessageBox, SetForegroundWindow, WM_HELP, WS_EX_TOPMOST};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// The message box contains one push button: "OK". This is the default.
pub const MB_OK: UINT = 0x00000000;

/// The message box contains two push buttons: "OK" and "Cancel".
pub const MB_OKCANCEL: UINT = 0x00000001;

/// The message box contains three push buttons: "Abort", "Retry", and "Ignore".
pub const MB_ABORTRETRYIGNORE: UINT = 0x00000002;

/// The message box contains three push buttons: "Yes", "No", and "Cancel".
pub const MB_YESNOCANCEL: UINT = 0x00000003;

/// The message box contains two push buttons: "Yes" and "No".
pub const MB_YESNO: UINT = 0x00000004;

/// The message box contains two push buttons: "Retry" and "Cancel".
pub const MB_RETRYCANCEL: UINT = 0x00000005;

/// The message box contains three push buttons: "Cancel", "Try Again", "Continue". Use this
/// message box type instead of [`MB_ABORTRETRYIGNORE`].
pub const MB_CANCELTRYCONTINUE: UINT = 0x00000006;

/// A stop-sign icon appears in the message box.
pub const MB_ICONHAND: UINT = 0x00000010;

/// A question-mark icon appears in the message box. The question-mark message icon is no longer
/// recommended because it does not clearly represent a specific type of message and because the
/// phrasing of a message as a question could apply to any message type. In addition, users can
/// confuse the message symbol question mark with Help information. Therefore, do not use this
/// question mark message symbol in your message boxes. The system continues to support its
/// inclusion only for backward compatibility.
pub const MB_ICONQUESTION: UINT = 0x00000020;

/// An exclamation-point icon appears in the message box.
pub const MB_ICONEXCLAMATION: UINT = 0x00000030;

/// An icon consisting of a lowercase letter 'i' in a circle appears in the message box.
pub const MB_ICONASTERISK: UINT = 0x00000040;

/// An exclamation-point icon appears in the message box.
pub const MB_ICONWARNING: UINT = MB_ICONEXCLAMATION;

/// A stop-sign icon appears in the message box.
pub const MB_ICONERROR: UINT = MB_ICONHAND;

/// An icon consisting of a lowercase letter 'i' in a circle appears in the message box.
pub const MB_ICONINFORMATION: UINT = MB_ICONASTERISK;

/// A stop-sign icon appears in the message box.
pub const MB_ICONSTOP: UINT = MB_ICONHAND;

/// The first button is the default button. [`MB_DEFBUTTON1`] is the default unless
/// [`MB_DEFBUTTON2`], [`MB_DEFBUTTON3`], or [`MB_DEFBUTTON4`] is specified.
pub const MB_DEFBUTTON1: UINT = 0x00000000;

/// The second button is the default button.
pub const MB_DEFBUTTON2: UINT = 0x00000100;

/// The third button is the default button.
pub const MB_DEFBUTTON3: UINT = 0x00000200;

/// The fourth button is the default button.
pub const MB_DEFBUTTON4: UINT = 0x00000300;

/// The user must respond to the message box before continuing work in the window identified by the
/// `wnd` parameter. However, the user can move to the windows of other threads and work in those
/// windows.
///
/// Depending on the hierarchy of windows in the application, the user may be able to move to other
/// windows within the thread. All child windows of the parent of the message box are automatically
/// disabled, but pop-up windows are not.
///
/// [`MB_APPLMODAL`] is the default if neither [`MB_SYSTEMMODAL`] nor [`MB_TASKMODAL`] is
/// specified.
pub const MB_APPLMODAL: UINT = 0x00000000;

/// Same as [`MB_APPLMODAL`] except that the message box has the [`WS_EX_TOPMOST`] style. Use
/// system-modal message boxes to notify the user of serious, potentially damaging errors that
/// require immediate attention (for example, running out of memory). This flag has no effect on
/// the user's ability to interact with windows other than those associated with `wnd`.
pub const MB_SYSTEMMODAL: UINT = 0x00001000;

/// Same as [`MB_APPLMODAL`] except that all the top-level windows belonging to the current thread
/// are disabled if the `wnd` parameter is [`null_mut`]. Use this flag when the calling application
/// or library does not have a window handle available but still needs to prevent input to other
/// windows in the calling thread without suspending other threads.
pub const MB_TASKMODAL: UINT = 0x00002000;

/// Adds a Help button to the message box. When the user clicks the Help button or presses F1, the
/// system sends a [`WM_HELP`] message to the owner.
pub const MB_HELP: UINT = 0x00004000;

/// The message box becomes the foreground window. Internally, the system calls the
/// [`SetForegroundWindow`] function for the message box.
pub const MB_SETFOREGROUND: UINT = 0x00010000;

/// Same as desktop of the interactive window station.
///
/// If the current input desktop is not the default desktop, [`MessageBox`] does not return until
/// the user switches to the default desktop.
pub const MB_DEFAULT_DESKTOP_ONLY: UINT = 0x00020000;

/// The message box is created with the [`WS_EX_TOPMOST`] window style.
pub const MB_TOPMOST: UINT = 0x00040000;

/// The text is right-justified.
pub const MB_RIGHT: UINT = 0x00080000;

/// Displays message and caption text using right-to-left reading order on Hebrew and Arabic
/// systems.
pub const MB_RTLREADING: UINT = 0x00100000;

/// The caller is a service notifying the user of an event. The function displays a message box on
/// the current active desktop, even if there is no user logged on to the computer.
///
/// Terminal Services: If the calling thread has an impersonation token, the function directs the
/// message box to the session specified in the impersonation token.
///
/// If this flag is set, the `wnd` parameter must be [`null_mut`]. This is so that the message box
/// can appear on a desktop other than the desktop corresponding to the `wnd`.
pub const MB_SERVICE_NOTIFICATION: UINT = 0x00200000;
