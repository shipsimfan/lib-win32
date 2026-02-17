use crate::{BOOL, HINSTANCE, LPCSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, RegisterClass, RegisterClassEx};

#[link(name = "User32")]
unsafe extern "system" {
    /// Unregisters a window class, freeing the memory required for the class.
    ///
    /// # Parameters
    ///  * `class_name` - A null-terminated string or a class atom. If `class_name` is a string, it
    ///                   specifies the window class name. This class name must have been
    ///                   registered by a previous call to the [`RegisterClass`] or
    ///                   [`RegisterClassEx`] function. System classes, such as dialog box
    ///                   controls, cannot be unregistered. If this parameter is an atom, it must
    ///                   be a class atom created by a previous call to the [`RegisterClass`] or
    ///                   [`RegisterClassEx`] function. The atom must be in the low-order word of
    ///                   `class_name`; the high-order word must be zero.
    ///  * `instance` - A handle to the instance of the module that created the class.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the class could not be found or if a window still exists that was created with the
    /// class, the return value is zero. To get extended error information, call [`GetLastError`].
    ///
    /// # Remarks
    /// Before calling this function, an application must destroy all windows created with the
    /// specified class.
    ///
    /// All window classes that an application registers are unregistered when it terminates.
    ///
    /// Class atoms are special atoms returned only by [`RegisterClass`] and [`RegisterClassEx`].
    ///
    /// No window classes registered by a DLL are unregistered when the .dll is unloaded.
    pub fn UnregisterClassA(class_name: LPCSTR, instance: HINSTANCE) -> BOOL;
}
