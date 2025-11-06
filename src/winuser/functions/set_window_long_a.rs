use crate::{HWND, LONG};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CallWindowProc, GetLastError, RegisterClassEx, SetLastError, SetParent, SetWindowLong,
    SetWindowPos, CS_CLASSDC, CS_OWNDC, DWL_MSGRESULT, GWL_EXSTYLE, GWL_HINSTANCE, GWL_HWNDPARENT,
    GWL_ID, GWL_STYLE, GWL_USERDATA, GWL_WNDPROC, SWP_FRAMECHANGED, TRUE, WNDCLASSEX,
    WS_EX_COMPOSITED, WS_EX_LAYERED,
};

#[link(name = "User32")]
extern "system" {
    /// Changes an attribute of the specified window. The function also sets the 32-bit (long)
    /// value at the specified offset into the extra window memory.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window and, indirectly, the class to which the window belongs.
    ///  * `index` - The zero-based offset to the value to be set. Valid values are in the range
    ///              zero through the number of bytes of extra window memory, minus the size of an
    ///              integer. To set any other value, specify one of the following values:
    ///    * [`GWL_EXSTYLE`] - Sets a new extended window style.
    ///    * [`GWL_HINSTANCE`] - Sets a new application instance handle.
    ///    * [`GWL_HWNDPARENT`] -  Sets a new owner for a top-level window.
    ///    * [`GWL_ID`] - Sets a new identifier of the child window. The window cannot be a
    ///                   top-level window.
    ///    * [`GWL_STYLE`] - Sets a new window style.
    ///    * [`GWL_USERDATA`] - Sets the user data associated with the window. This data is
    ///                         intended for use by the application that created the window. Its
    ///                         value is initially zero.
    ///    * [`GWL_WNDPROC`] - Sets a new address for the window procedure. You cannot change this
    ///                        attribute if the window does not belong to the same process as the
    ///                        calling thread.
    ///  * `new_long` - The replacement value.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the previous value of the specified 32-bit
    /// integer.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// If the previous value of the specified 32-bit integer is zero, and the function succeeds,
    /// the return value is zero, but the function does not clear the last error information. This
    /// makes it difficult to determine success or failure. To deal with this, you should clear the
    /// last error information by calling [`SetLastError`] with 0 before calling [`SetWindowLong`].
    /// Then, function failure will be indicated by a return value of zero and a [`GetLastError`]
    /// result that is nonzero.
    ///
    /// # Remarks
    /// Certain window data is cached, so changes you make using [`SetWindowLong`] will not take
    /// effect until you call the [`SetWindowPos`] function. Specifically, if you change any of the
    /// frame styles, you must call [`SetWindowPos`] with the [`SWP_FRAMECHANGED`] flag for the
    /// cache to be updated properly.
    ///
    /// If you use [`SetWindowLong`] with the [`GWL_WNDPROC`] index to replace the window
    /// procedure, the window procedure must conform to the guidelines specified in the description
    /// of the `wnd_proc` callback function.
    ///
    /// If you use [`SetWindowLong`] with the [`DWL_MSGRESULT`] index to set the return value for a
    /// message processed by a dialog procedure, you should return [`TRUE`] directly afterward.
    /// Otherwise, if you call any function that results in your dialog procedure receiving a
    /// window message, the nested window message could overwrite the return value you set using
    /// [`DWL_MSGRESULT`].
    ///
    /// Calling [`SetWindowLong`] with the [`GWL_WNDPROC`] index creates a subclass of the window
    /// class used to create the window. An application can subclass a system class, but should not
    /// subclass a window class created by another process. The [`SetWindowLong`] function creates
    /// the window subclass by changing the window procedure associated with a particular window
    /// class, causing the system to call the new window procedure instead of the previous one. An
    /// application must pass any messages not processed by the new window procedure to the
    /// previous window procedure by calling [`CallWindowProc`]. This allows the application to
    /// create a chain of window procedures.
    ///
    /// Reserve extra window memory by specifying a nonzero value in the `wnd_extra` member of the
    /// [`WNDCLASSEX`] structure used with the [`RegisterClassEx`] function.
    ///
    /// Do not call [`SetWindowLong`] with the [`GWL_HWNDPARENT`] index to change the parent of a
    /// child window. Instead, use the [`SetParent`] function.
    ///
    /// [`GWL_HWNDPARENT`] is used to change the owner of a top-level window, not the parent of a
    /// child window.
    ///
    /// A window can have either a parent or an owner, or neither, but never both simultaneously.
    ///
    /// If the window has a class style of [`CS_CLASSDC`] or [`CS_OWNDC`], do not set the extended
    /// window styles [`WS_EX_COMPOSITED`] or [`WS_EX_LAYERED`].
    ///
    /// Calling [`SetWindowLong`] to set the style on a progressbar will reset its position.
    pub fn SetWindowLongA(wnd: HWND, index: c_int, new_long: LONG) -> LONG;
}
