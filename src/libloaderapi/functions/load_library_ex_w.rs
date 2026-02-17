use crate::{DWORD, HANDLE, HMODULE, LPCWSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, GetProcAddress, LoadLibrary, LoadLibraryEx};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Loads the specified module into the address space of the calling process. The specified
    /// module may cause other modules to be loaded.
    ///
    /// # Parameters
    ///  * `lib_file_name` - A string that specifies the file name of the module to load. This name
    ///                      is not related to the name stored in a library module itself, as
    ///                      specified by the `LIBRARY` keyword in the module-definition (.def)
    ///                      file. The module can be a library module (a .dll file) or an
    ///                      executable module (an .exe file). If the specified module is an
    ///                      executable module, static imports are not loaded; instead, the module
    ///                      is loaded as if [`DONT_RESOLVE_DLL_REFERENCES`] was specified. See the
    ///                      `flags` parameter for more information. If the string specifies a
    ///                      module name without a path and the file name extension is omitted, and
    ///                      the module name does not contain any point character (.), then the
    ///                      function appends the default library extension ".DLL" to the module
    ///                      name. To prevent the function from appending ".DLL" to the module
    ///                      name, include a trailing point character (.) in the module name
    ///                      string. If the string specifies a fully qualified path, the function
    ///                      searches only that path for the module. When specifying a path, be
    ///                      sure to use backslashes (\), not forward slashes (/). If the string
    ///                      specifies a module name without a path and more than one loaded module
    ///                      has the same base name and extension, the function returns a handle to
    ///                      the module that was loaded first. If the string specifies a module
    ///                      name without a path and a module of the same name is not already
    ///                      loaded, or if the string specifies a module name with a relative path,
    ///                      the function searches for the specified module. The function also
    ///                      searches for modules if loading the specified module causes the system
    ///                      to load other associated modules (that is, if the module has
    ///                      dependencies). The directories that are searched and the order in
    ///                      which they are searched depend on the specified path and the `flags`
    ///                      parameter. If the function cannot find the module or one of its
    ///                      dependencies, the function fails.
    ///  * `file` - This parameter is reserved for future use. It must be [`null_mut`].
    ///  * `flags` -
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the loaded module.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// The [`LoadLibraryEx`] function is very similar to the [`LoadLibrary`] function. The
    /// differences consist of a set of optional behaviors that [`LoadLibraryEx`] provides:
    ///  - [`LoadLibraryEx`] can load a DLL module without calling the DllMain function of the DLL.
    ///  - [`LoadLibraryEx`] can load a module in a way that is optimized for the case where the
    ///    module will never be executed, loading the module as if it were a data file.
    ///  - [`LoadLibraryEx`] can find modules and their associated modules by using either of two
    ///    search strategies or it can search a process-specific set of directories.
    ///
    /// You select these optional behaviors by setting the `flags` parameter; if `flags` is zero,
    /// [`LoadLibraryEx`] behaves identically to [`LoadLibrary`].
    ///
    /// The calling process can use the handle returned by [`LoadLibraryEx`] to identify the module
    /// in calls to the [`GetProcAddress`], [`FindResource`], and [`LoadResource`] functions.
    ///
    /// To enable or disable error messages displayed by the loader during DLL loads, use the
    /// [`SetErrorMode`] function.
    ///
    /// It is not safe to call [`LoadLibraryEx`] from `DllMain`.
    pub fn LoadLibraryExW(lib_file_name: LPCWSTR, file: HANDLE, flags: DWORD) -> HMODULE;
}
