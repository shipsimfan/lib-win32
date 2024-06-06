use crate::{HWND, LONG_PTR};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, RegisterClassEx, SetLastError, SetWindowLongPtr, GWLP_HINSTANCE, GWLP_ID,
    GWLP_USERDATA, GWLP_WNDPROC, GWL_EXSTYLE, GWL_STYLE, TRUE, WNDCLASSEX, WS_EX_COMPOSITED,
    WS_EX_LAYERED,
};

#[link(name = "User32")]
extern "system" {
    /// Changes an attribute of the specified window. The function also sets a value at the
    /// specified offset in the extra window memory.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window and, indirectly, the class to which the window belongs.
    ///            The [`SetWindowLongPtr`] function fails if the process that owns the window
    ///            specified by the hWnd parameter is at a higher process privilege in the UIPI
    ///            hierarchy than the process the calling thread resides in.
    ///  * `index` - The zero-based offset to the value to be set. Valid values are in the range
    ///              zero through the number of bytes of extra window memory, minus the size of a
    ///              [`LONG_PTR`]. To set any other value, specify one of the following values:
    ///    * [`GWL_EXSTYLE`] - Sets a new extended window style.
    ///    * [`GWLP_HINSTANCE`] - Sets a new application instance handle.
    ///    * [`GWLP_ID`] - Sets a new identifier of the child window. The window cannot be a
    ///                    top-level window.
    ///    * [`GWL_STYLE`] - Sets a new window style.
    ///    * [`GWLP_USERDATA`] - Sets the user data associated with the window. This data is
    ///                          intended for use by the application that created the window. Its
    ///                          value is initially zero.
    ///    * [`GWLP_WNDPROC`] - Sets a new address for the window procedure.
    ///  * `new_long` - The replacement value.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the previous value of the specified offset.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// If the previous value is zero and the function succeeds, the return value is zero, but the
    /// function does not clear the last error information. To determine success or failure, clear
    /// the last error information by calling [`SetLastError`] with `0`, then call
    /// [`SetWindowLongPtr`]. Function failure will be indicated by a return value of zero and a
    /// [`GetLastError`] result that is nonzero.
    ///
    /// # Remarks
    /// Certain window data is cached, so changes you make using [`SetWindowLongPtr`] will not take
    /// effect until you call the [`SetWindowPos`] function.
    ///
    /// If you use [`SetWindowLongPtr`] with the [`GWLP_WNDPROC`] index to replace the window
    /// procedure, the window procedure must conform to the guidelines specified in the description
    /// of the [`WindowProc`] callback function.
    ///
    /// If you use [`SetWindowLongPtr`] with the [`DWLP_MSGRESULT`] index to set the return value
    /// for a message processed by a dialog box procedure, the dialog box procedure should return
    /// [`TRUE`] directly afterward. Otherwise, if you call any function that results in your
    /// dialog box procedure receiving a window message, the nested window message could overwrite
    /// the return value you set by using [`DWLP_MSGRESULT`].
    ///
    /// Calling [`SetWindowLongPtr`] with the [`GWLP_WNDPROC`] index creates a subclass of the
    /// window class used to create the window. An application can subclass a system class, but
    /// should not subclass a window class created by another process. The [`SetWindowLongPtr`]
    /// function creates the window subclass by changing the window procedure associated with a
    /// particular window class, causing the system to call the new window procedure instead of the
    /// previous one. An application must pass any messages not processed by the new window
    /// procedure to the previous window procedure by calling [`CallWindowProc`]. This allows the
    /// application to create a chain of window procedures.
    ///
    /// Reserve extra window memory by specifying a nonzero value in the `wnd_extra` member of the
    /// [`WNDCLASSEX`] structure used with the [`RegisterClassEx`] function.
    ///
    /// Do not call [`SetWindowLongPtr`] with the [`GWLP_HWNDPARENT`] index to change the parent of
    /// a child window. Instead, use the [`SetParent`] function.
    ///
    /// If the window has a class style of [`CS_CLASSDC`] or [`CS_PARENTDC`], do not set the
    /// extended window styles [`WS_EX_COMPOSITED`] or [`WS_EX_LAYERED`].
    ///
    /// Calling [`SetWindowLongPtr`] to set the style on a progressbar will reset its position.
    pub fn SetWindowLongPtrW(wnd: HWND, index: c_int, new_long: LONG_PTR) -> LONG_PTR;
}
