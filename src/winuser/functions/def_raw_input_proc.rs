use crate::{INT, LRESULT, PRAWINPUT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{DefWindowProcA, DefWindowProcW, RAWINPUTHEADER};

#[link(name = "User32")]
unsafe extern "system" {
    /// Unlike [`DefWindowProcA`] and [`DefWindowProcW`], this function doesn't do any processing.
    ///
    /// [`DefRawInputProc`] only checks whether `size_header`'s value corresponds to the expected
    /// size of [`RAWINPUTHEADER`].
    ///
    /// # Parameters
    ///  * `raw_input` - Ignored.
    ///  * `input` - Ignored.
    ///  * `size_header` - The size, in bytes, of the [`RAWINPUTHEADER`] structure.
    ///
    /// # Return Value
    /// If successful, the function returns 0. Otherwise it returns -1.
    pub fn DefRawInputProc(raw_input: *mut PRAWINPUT, input: INT, size_header: UINT) -> LRESULT;
}
