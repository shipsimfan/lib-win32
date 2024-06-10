use crate::{HMODULE, LPCWSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{FreeLibrary, GetLastError, GetModuleHandle};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Kernel32")]
extern "system" {
    /// Retrieves a module handle for the specified module. The module must have been loaded by the
    /// calling process.
    ///
    /// To avoid the race conditions described in the Remarks section, use the
    /// [`GetModuleHandleEx`] function.
    ///
    /// # Parameters
    ///  * `module_name` - The name of the loaded module (either a .dll or .exe file). If the file
    ///    name extension is omitted, the default library extension .dll is appended. The file name
    ///    string can include a trailing point character (.) to indicate that the module name has
    ///    no extension. The string does not have to specify a path. When specifying a path, be
    ///    sure to use backslashes (\), not forward slashes (/). The name is compared (case
    ///    independently) to the names of modules currently mapped into the address space of the
    ///    calling process. If this parameter is [`null`], GetModuleHandle returns a handle to the
    ///    file used to create the calling process (.exe file). The [`GetModuleHandle`] function
    ///    does not retrieve handles for modules that were loaded using the
    ///    [`LOAD_LIBRARY_AS_DATAFILE`] flag. For more information, see [`LoadLibraryEx`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the specified module.
    ///
    /// If the function fails, the return value is [`null`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// The returned handle is not global or inheritable. It cannot be duplicated or used by
    /// another process.
    ///
    /// If `module_name` does not include a path and there is more than one loaded module with the
    /// same base name and extension, you cannot predict which module handle will be returned. To
    /// work around this problem, you could specify a path, use side-by-side assemblies, or use
    /// [`GetModuleHandleEx`] to specify a memory location rather than a DLL name.
    ///
    /// The [`GetModuleHandle`] function returns a handle to a mapped module without incrementing
    /// its reference count. However, if this handle is passed to the [`FreeLibrary`] function, the
    /// reference count of the mapped module will be decremented. Therefore, do not pass a handle
    /// returned by [`GetModuleHandle`] to the [`FreeLibrary`] function. Doing so can cause a DLL
    /// module to be unmapped prematurely.
    ///
    /// This function must be used carefully in a multithreaded application. There is no guarantee
    /// that the module handle remains valid between the time this function returns the handle and
    /// the time it is used. For example, suppose that a thread retrieves a module handle, but
    /// before it uses the handle, a second thread frees the module. If the system loads another
    /// module, it could reuse the module handle that was recently freed. Therefore, the first
    /// thread would have a handle to a different module than the one intended.
    pub fn GetModuleHandleW(module_name: LPCWSTR) -> HMODULE;
}
