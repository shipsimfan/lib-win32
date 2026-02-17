use crate::{HWND, LONG_PTR};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, GetWindowLongPtr, RegisterClassEx, SetWindowLong, SetWindowLongPtr,
    GWLP_HINSTANCE, GWLP_ID, GWLP_USERDATA, GWLP_WNDPROC, GWL_EXSTYLE, GWL_STYLE, WNDCLASSEX,
};

#[link(name = "User32")]
unsafe extern "system" {
    /// Retrieves information about the specified window. The function also retrieves the value at
    /// a specified offset into the extra window memory.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window and, indirectly, the class to which the window belongs.
    ///  * `index` - The zero-based offset to the value to be retrieved. Valid values are in the
    ///              range zero through the number of bytes of extra window memory, minus the size
    ///              of a [`LONG_PTR`]. To retrieve any other value, specify one of the following
    ///              values:
    ///    * [`GWL_EXSTYLE`] - Sets a new extended window style.
    ///    * [`GWLP_HINSTANCE`] - Sets a new application instance handle.
    ///    * [`GWLP_ID`] - Sets a new identifier of the child window. The window cannot be a
    ///                    top-level window.
    ///    * [`GWL_STYLE`] - Sets a new window style.
    ///    * [`GWLP_USERDATA`] - Sets the user data associated with the window. This data is
    ///                          intended for use by the application that created the window. Its
    ///                          value is initially zero.
    ///    * [`GWLP_WNDPROC`] - Sets a new address for the window procedure.
    ///
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
    pub fn GetWindowLongPtrA(wnd: HWND, index: c_int) -> LONG_PTR;
}
