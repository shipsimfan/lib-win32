use crate::{BOOL, MSG};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetMessage, IsDialogMessage, PeekMessage, WM_CHAR, WM_DEADCHAR, WM_KEYDOWN, WM_KEYUP,
    WM_SYSCHAR, WM_SYSDEADCHAR, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

#[link(name = "User32")]
unsafe extern "system" {
    /// Translates virtual-key messages into character messages. The character messages are posted
    /// to the calling thread's message queue, to be read the next time the thread calls the
    /// [`GetMessage`] or [`PeekMessage`] function.
    ///
    /// # Parameters
    ///  * `msg` - A pointer to an [`MSG`] structure that contains message information retrieved
    ///            from the calling thread's message queue by using the [`GetMessage`] or
    ///            [`PeekMessage`] function.
    ///
    /// # Return Value
    /// If the message is translated (that is, a character message is posted to the thread's
    /// message queue), the return value is nonzero.
    ///
    /// If the message is [`WM_KEYDOWN`], [`WM_KEYUP`], [`WM_SYSKEYDOWN`], or [`WM_SYSKEYUP`], the
    /// return value is nonzero, regardless of the translation.
    ///
    /// If the message is not translated (that is, a character message is not posted to the
    /// thread's message queue), the return value is zero.
    ///
    /// # Remarks
    /// The [`TranslateMessage`] function does not modify the message pointed to by the `msg`
    /// parameter.
    ///
    /// [`WM_KEYDOWN`] and [`WM_KEYUP`] combinations produce a [`WM_CHAR`] or [`WM_DEADCHAR`]
    /// message. [`WM_SYSKEYDOWN`] and [`WM_SYSKEYUP`] combinations produce a [`WM_SYSCHAR`] or
    /// [`WM_SYSDEADCHAR`] message.
    ///
    /// [`TranslateMessage`] produces [`WM_CHAR`] messages only for keys that are mapped to ASCII
    /// characters by the keyboard driver.
    ///
    /// If applications process virtual-key messages for some other purpose, they should not call
    /// [`TranslateMessage`]. For instance, an application should not call [`TranslateMessage`] if
    /// the [`TranslateAccelerator`] function returns a nonzero value. Note that the application is
    /// responsible for retrieving and dispatching input messages to the dialog box. Most
    /// applications use the main message loop for this. However, to permit the user to move to and
    /// to select controls by using the keyboard, the application must call [`IsDialogMessage`].
    pub fn TranslateMessage(msg: *const MSG) -> BOOL;
}
