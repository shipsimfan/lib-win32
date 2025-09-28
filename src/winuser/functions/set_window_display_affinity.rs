use crate::{BOOL, DWORD, HWND};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, FALSE, TRUE, WDA_EXCLUDEFROMCAPTURE, WDA_MONITOR, WDA_NONE};

#[link(name = "User32")]
extern "system" {
    /// Specifies where the content of the window can be displayed.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the top-level window. The window must belong to the current process.
    ///  * `affinity` - The display affinity setting that specifies where the content of the window
    ///                 can be displayed. This parameter can be one of the following values:
    ///    * [`WDA_NONE`] - Imposes no restrictions on where the window can be displayed.
    ///    * [`WDA_MONITOR`] - The window content is displayed only on a monitor. Everywhere else,
    ///                        the window appears with no content.
    ///    * [`WDA_EXCLUDEFROMCAPTURE`] - The window is displayed only on a monitor. Everywhere
    ///                                   else, the window does not appear at all. One use for this
    ///                                   affinity is for windows that show video recording
    ///                                   controls, so that the controls are not included in the
    ///                                   capture.
    ///
    /// # Return Value
    /// If the function succeeds, it returns [`TRUE`]; otherwise, it returns [`FALSE`] when, for
    /// example, the function call is made on a non top-level window. To get extended error
    /// information, call [`GetLastError`].
    ///
    /// # Remarks
    /// This function and [`GetWindowDisplayAffinity`] are designed to support the window content
    /// protection feature that is new to Windows 7. This feature enables applications to protect
    /// their own onscreen window content from being captured or copied through a specific set of
    /// public operating system features and APIs. However, it works only when the Desktop Window
    /// Manager(DWM) is composing the desktop.
    ///
    /// It is important to note that unlike a security feature or an implementation of Digital
    /// Rights Management (DRM), there is no guarantee that using [`SetWindowDisplayAffinity`] and
    /// [`GetWindowDisplayAffinity`], and other necessary functions such as
    /// [`DwmIsCompositionEnabled`], will strictly protect windowed content, for example where
    /// someone takes a photograph of the screen.
    ///
    /// Starting in Windows 10 Version 2004, [`WDA_EXCLUDEFROMCAPTURE`] is a supported value.
    /// Setting the display affinity to [`WDA_EXCLUDEFROMCAPTURE`] on previous version of Windows
    /// will behave as if [`WDA_MONITOR`] is applied.
    pub fn SetWindowDisplayAffinity(wnd: HWND, affinity: DWORD) -> BOOL;
}
