use crate::{HWND, LPPOINT, UINT};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SetLastError, POINT, RECT, WS_EX_LAYOUTRTL};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// The [`MapWindowPoints`] function converts (maps) a set of points from a coordinate space
    /// relative to one window to a coordinate space relative to another window.
    ///
    /// # Parameters
    ///  * `wnd_from` - A handle to the window from which points are converted. If this parameter
    ///                 is [`null_mut`] or [`HWND_DESKTOP`], the points are presumed to be in
    ///                 screen coordinates.
    ///  * `wnd_to` - A handle to the window to which points are converted. If this parameter is
    ///               [`null_mut`] or [`HWND_DESKTOP`], the points are converted to screen
    ///               coordinates.
    ///  * `points` - A pointer to an array of [`POINT`] structures that contain the set of points
    ///               to be converted. The points are in device units. This parameter can also
    ///               point to a [`RECT`] structure, in which case the `num_points` parameter
    ///               should be set to 2.
    ///  * `num_points` - The number of [`POINT`] structures in the array pointed to by the
    ///                   `points` parameter.
    ///
    /// # Return Value
    /// If the function succeeds, the low-order word of the return value is the number of pixels
    /// added to the horizontal coordinate of each source point in order to compute the horizontal
    /// coordinate of each destination point. (In addition to that, if precisely one of `wnd_from`
    /// and `wnd_to` is mirrored, then each resulting horizontal coordinate is multiplied by -1.)
    /// The high-order word is the number of pixels added to the vertical coordinate of each source
    /// point in order to compute the vertical coordinate of each destination point.
    ///
    /// If the function fails, the return value is zero. Call [`SetLastError`] prior to calling
    /// this method to differentiate an error return value from a legitimate "0" return value.
    ///
    /// # Remarks
    /// If `wnd_from` or `wnd_to` (or both) are mirrored windows (that is, have [`WS_EX_LAYOUTRTL`]
    /// extended style) and precisely two points are passed in `points`, [`MapWindowPoints`] will
    /// interpret those two points as a [`RECT`] and possibly automatically swap the left and right
    /// fields of that rectangle to ensure that left is not greater than right. If any number of
    /// points other than 2 is passed in `points`, then [`MapWindowPoints`] will correctly map the
    /// coordinates of each of those points separately, so if you pass in a pointer to an array of
    /// more than one rectangle in `points`, the new rectangles may get their left field greater
    /// than right. Thus, to guarantee the correct transformation of rectangle coordinates, you
    /// must call [`MapWindowPoints`] with one [`RECT`] pointer at a time.
    ///
    /// Also, if you need to map precisely two independent points and don't want the [`RECT`] logic
    /// applied to them by [`MapWindowPoints`], to guarantee the correct result you must call
    /// [`MapWindowPoints`] with one [`POINT`] pointer at a time.
    pub fn MapWindowPoints(
        wnd_from: HWND,
        wnd_to: HWND,
        points: LPPOINT,
        num_points: UINT,
    ) -> c_int;
}
