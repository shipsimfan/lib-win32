use crate::{ATOM, WNDCLASSEX};

// rustdoc imports
#[allow(unused_imports)]
use crate::GetLastError;

#[link(name = "User32")]
extern "system" {
    /// Registers a window class for subsequent use in calls to the [`CreateWindow`] or
    /// [`CreateWindowEx`] function.
    ///
    /// # Parameters
    ///  * `unnamed` - A pointer to a [`WNDCLASSEX`] structure. You must fill the structure with
    ///                the appropriate class attributes before passing it to the function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a class atom that uniquely identifies the
    /// class being registered. This atom can only be used by the [`CreateWindow`],
    /// [`CreateWindowEx`], [`GetClassInfo`], [`GetClassInfoEx`], [`FindWindow`], [`FindWindowEx`],
    /// and [`UnregisterClass`] functions and the [`IActiveIMMap::FilterClientWindows`] method.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// If you register the window class by using [`RegisterClassExA`], the application tells the
    /// system that the windows of the created class expect messages with text or character
    /// parameters to use the ANSI character set; if you register it by using [`RegisterClassExW`],
    /// the application requests that the system pass text parameters of messages as Unicode. The
    /// [`IsWindowUnicode`] function enables applications to query the nature of each window.
    ///
    /// All window classes that an application registers are unregistered when it terminates.
    ///
    /// No window classes registered by a DLL are unregistered when the DLL is unloaded. A DLL must
    /// explicitly unregister its classes when it is unloaded.
    pub fn RegisterClassExW(unnamed: *const WNDCLASSEX) -> ATOM;
}
