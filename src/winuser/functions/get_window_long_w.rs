use crate::{HWND, LONG};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CallWindowProc, GetLastError, GetWindowLongPtr, RegisterClassEx, SetWindowLong,
    SetWindowLongPtr, DWORD, GWLP_HINSTANCE, GWLP_HWNDPARENT, GWLP_ID, GWLP_USERDATA, GWLP_WNDPROC,
    GWL_EXSTYLE, GWL_STYLE, WNDCLASSEX,
};

#[link(name = "User32")]
extern "system" {
    /// Retrieves information about the specified window. The function also retrieves the 32-bit
    /// ([`DWORD`]) value at the specified offset into the extra window memory.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window and, indirectly, the class to which the window belongs.
    ///  * `index` - The zero-based offset to the value to be retrieved. Valid values are in the
    ///              range zero through the number of bytes of extra window memory, minus four; for
    ///              example, if you specified 12 or more bytes of extra memory, a value of 8 would
    ///              be an index to the third 32-bit integer. To retrieve any other value, specify
    ///              one of the following values:
    ///    * [`GWL_EXSTYLE`] - Retrieves the extended window styles.
    ///    * [`GWLP_HINSTANCE`] - Retrieves a handle to the application instance.
    ///    * [`GWLP_HWNDPARENT`] - Retrieves a handle to the parent window, if there is one.
    ///    * [`GWLP_ID`] - Retrieves the identifier of the window.
    ///    * [`GWL_STYLE`] - Retrieves the window styles.
    ///    * [`GWLP_USERDATA`] - Retrieves the user data associated with the window. This data is
    ///                          intended for use by the application that created the window. Its
    ///                          value is initially zero.
    ///    * [`GWLP_WNDPROC`] -  Retrieves the pointer to the window procedure, or a handle
    ///                          representing the pointer to the window procedure. You must use the
    ///                          [`CallWindowProc`] function to call the window procedure.
    /// # Return Value
    /// If the function succeeds, the return value is the requested value.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// If [`SetWindowLong`] or [`SetWindowLongPtr`] has not been called previously,
    /// [`GetWindowLongPtr`] returns zero for values in the extra window or class memory.
    ///
    /// # Remarks
    /// Reserve extra window memory by specifying a nonzero value in the `wnd_extra` member of the
    /// [`WNDCLASSEX`] structure used with the [`RegisterClassEx`] function.
    pub fn GetWindowLongW(wnd: HWND, index: c_int) -> LONG;
}
