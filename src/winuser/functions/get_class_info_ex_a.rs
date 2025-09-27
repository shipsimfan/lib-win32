use crate::{BOOL, HINSTANCE, LPCSTR, LPWNDCLASSEXW};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetClassInfo, GetLastError, RegisterClass, RegisterClassEx, WNDCLASSEX};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// Retrieves information about a window class, including a handle to the small icon associated
    /// with the window class. The [`GetClassInfo`] function does not retrieve a handle to the
    /// small icon.
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
    ///  * `wcx` - A pointer to a [`WNDCLASSEX`] structure that receives the information about the
    ///            class.
    ///
    /// # Return Value
    /// If the function finds a matching class and successfully copies the data, the return value
    /// is nonzero.
    ///
    /// If the function does not find a matching class and successfully copy the data, the return
    /// value is zero. To get extended error information, call [`GetLastError`].
    ///
    /// # Remarks
    /// Class atoms are created using the [`RegisterClass`] or [`RegisterClassEx`] function, not
    /// the [`GlobalAddAtom`] function.
    pub fn GetClassInfoExA(instance: HINSTANCE, class: LPCSTR, wcx: LPWNDCLASSEXW) -> BOOL;
}
