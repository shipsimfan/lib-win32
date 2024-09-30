use crate::{BOOL, HINSTANCE, LPCWSTR, LPWNDCLASSW};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, RegisterClass, RegisterClassEx, WNDCLASS};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// Retrieves information about a window class.
    ///
    /// # Parameters
    ///  * `instance` - A handle to the instance of the application that created the class. To
    ///                 retrieve information about classes defined by the system (such as buttons
    ///                 or list boxes), set this parameter to [`null_mut`].
    ///  * `class` - The class name. The name must be that of a preregistered class or a class
    ///              registered by a previous call to the [`RegisterClass`] or [`RegisterClassEx`]
    ///              function. Alternatively, this parameter can be a class atom created by a
    ///              previous call to [`RegisterClass`] or [`RegisterClassEx`]. The atom must be in
    ///              the low-order word of `class`; the high-order word must be zero.
    ///  * `wnd_class` - A pointer to a [`WNDCLASS`] structure that receives the information about
    ///                  the class.
    ///
    /// # Return Value
    /// If the function finds a matching class and successfully copies the data, the return value
    /// is nonzero.
    ///
    /// If the function does not find a matching class and successfully copy the data, the return
    /// value is zero. To get extended error information, call [`GetLastError`].
    pub fn GetClassInfoW(instance: HINSTANCE, class: LPCWSTR, wnd_class: LPWNDCLASSW) -> BOOL;
}
