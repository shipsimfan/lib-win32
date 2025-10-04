use crate::{BOOL, HMODULE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, LoadLibrary, LoadLibraryEx};

#[link(name = "Kernel32")]
extern "system" {
    /// Frees the loaded dynamic-link library (DLL) module and, if necessary, decrements its
    /// reference count. When the reference count reaches zero, the module is unloaded from the
    /// address space of the calling process and the handle is no longer valid.
    ///
    /// # Parameters
    ///  * `lib_module` - A handle to the loaded library module. The [`LoadLibrary`],
    ///                   [`LoadLibraryEx`], [`GetModuleHandle`], or [`GetModuleHandleEx`] function
    ///                   returns this handle.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// the [`GetLastError`] function.
    ///
    /// # Remarks
    /// The system maintains a per-process reference count for each loaded module. A module that
    /// was loaded at process initialization due to load-time dynamic linking has a reference count
    /// of one. The reference count for a module is incremented each time the module is loaded by a
    /// call to [`LoadLibrary`]. The reference count is also incremented by a call to
    /// [`LoadLibraryEx`] unless the module is being loaded for the first time and is being loaded
    /// as a data or image file.
    ///
    /// The reference count is decremented each time the [`FreeLibrary`] or
    /// [`FreeLibraryAndExitThread`] function is called for the module. When a module's reference
    /// count reaches zero or the process terminates, the system unloads the module from the
    /// address space of the process. Before unloading a library module, the system enables the
    /// module to detach from the process by calling the module's `DllMain` function, if it has
    /// one, with the [`DLL_PROCESS_DETACH`] value. Doing so gives the library module an
    /// opportunity to clean up resources allocated on behalf of the current process. After the
    /// entry-point function returns, the library module is removed from the address space of the
    /// current process.
    ///
    /// It is not safe to call [`FreeLibrary`] from `DllMain`.
    ///
    /// Calling [`FreeLibrary`] does not affect other processes that are using the same module.
    ///
    /// Use caution when calling [`FreeLibrary`] with a handle returned by [`GetModuleHandle`]. The
    /// [`GetModuleHandle`] function does not increment a module's reference count, so passing this
    /// handle to [`FreeLibrary`] can cause a module to be unloaded prematurely.
    ///
    /// A thread that must unload the DLL in which it is executing and then terminate itself should
    /// call [`FreeLibraryAndExitThread`] instead of calling [`FreeLibrary`] and [`ExitThread`]
    /// separately. Otherwise, a race condition can occur.
    pub fn FreeLibrary(lib_module: HMODULE) -> BOOL;
}
