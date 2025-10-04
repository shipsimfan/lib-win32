use crate::{HMODULE, LPCSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{FreeLibrary, GetLastError, GetProcAddress, LoadLibrary, LoadLibraryEx, FALSE, TRUE};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Loads the specified module into the address space of the calling process. The specified
    /// module may cause other modules to be loaded.
    ///
    /// For additional load options, use the [`LoadLibraryEx`] function.
    ///
    /// # Parameters
    ///  * `lib_file_name` - The name of the module. This can be either a library module (a .dll
    ///                      file) or an executable module (an .exe file). If the specified module
    ///                      is an executable module, static imports are not loaded; instead, the
    ///                      module is loaded as if by [`LoadLibraryEx`] with the
    ///                      [`DONT_RESOLVE_DLL_REFERENCES`] flag. The name specified is the file
    ///                      name of the module and is not related to the name stored in the
    ///                      library module itself, as specified by the LIBRARY keyword in the
    ///                      module-definition (.def) file. If the string specifies a full path,
    ///                      the function searches only that path for the module. If the string
    ///                      specifies a relative path or a module name without a path, the
    ///                      function uses a standard search strategy to find the module; for more
    ///                      information, see the Remarks. If the function cannot find the module,
    ///                      the function fails. When specifying a path, be sure to use backslashes
    ///                      (\), not forward slashes (/). If the string specifies a module name
    ///                      without a path and the file name extension is omitted, the function
    ///                      appends the default library extension ".DLL" to the module name. To
    ///                      prevent the function from appending ".DLL" to the module name, include
    ///                      a trailing point character (.) in the module name string.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the module.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// To enable or disable error messages displayed by the loader during DLL loads, use the
    /// [`SetErrorMode`] function.
    ///
    /// [`LoadLibrary`] can be used to load a library module into the address space of the process
    /// and return a handle that can be used in [`GetProcAddress`] to get the address of a DLL
    /// function. [`LoadLibrary`] can also be used to load other executable modules. For example,
    /// the function can specify an .exe file to get a handle that can be used in [`FindResource`]
    /// or [`LoadResource`]. However, do not use [`LoadLibrary`] to run an .exe file. Instead, use
    /// the [`CreateProcess`] function.
    ///
    /// If the specified module is a DLL that is not already loaded for the calling process, the
    /// system calls the DLL's `DllMain` function with the [`DLL_PROCESS_ATTACH`] value. If
    /// `DllMain` returns [`TRUE`], LoadLibrary returns a handle to the module. If `DllMain`
    /// returns [`FALSE`], the system unloads the DLL from the process address space and
    /// [`LoadLibrary`] returns [`null_mut`]. It is not safe to call [`LoadLibrary`] from
    /// `DllMain`.
    ///
    /// Module handles are not global or inheritable. A call to [`LoadLibrary`] by one process does
    /// not produce a handle that another process can use — for example, in calling
    /// [`GetProcAddress`]. The other process must make its own call to [`LoadLibrary`] for the
    /// module before calling [`GetProcAddress`].
    ///
    /// If `lib_file_name` does not include a path and there is more than one loaded module with
    /// the same base name and extension, the function returns a handle to the module that was
    /// loaded first.
    ///
    /// If no file name extension is specified in the `lib_file_name` parameter, the default
    /// library extension .dll is appended. However, the file name string can include a trailing
    /// point character (.) to indicate that the module name has no extension. When no path is
    /// specified, the function searches for loaded modules whose base name matches the base name
    /// of the module to be loaded. If the name matches, the load succeeds. Otherwise, the function
    /// searches for the file.
    ///
    /// The first directory searched is the directory containing the image file used to create the
    /// calling process (for more information, see the CreateProcess function). Doing this allows
    /// private dynamic-link library (DLL) files associated with a process to be found without
    /// adding the process's installed directory to the `PATH` environment variable. If a relative
    /// path is specified, the entire relative path is appended to every token in the DLL search
    /// path list. To load a module from a relative path without searching any other path, use
    /// [`GetFullPathName`] to get a nonrelative path and call [`LoadLibrary`] with the nonrelative
    /// path.
    ///
    /// The search path can be altered using the [`SetDllDirectory`] function. This solution is
    /// recommended instead of using [`SetCurrentDirectory`] or hard-coding the full path to the
    /// DLL.
    ///
    /// If a path is specified and there is a redirection file for the application, the function
    /// searches for the module in the application's directory. If the module exists in the
    /// application's directory, [`LoadLibrary`] ignores the specified path and loads the module
    /// from the application's directory. If the module does not exist in the application's
    /// directory, [`LoadLibrary`] loads the module from the specified directory.
    ///
    /// If you call [`LoadLibrary`] with the name of an assembly without a path specification and
    /// the assembly is listed in the system compatible manifest, the call is automatically
    /// redirected to the side-by-side assembly.
    ///
    /// The system maintains a per-process reference count on all loaded modules. Calling
    /// [`LoadLibrary`] increments the reference count. Calling the [`FreeLibrary`] or
    /// [`FreeLibraryAndExitThread`] function decrements the reference count. The system unloads a
    /// module when its reference count reaches zero or when the process terminates (regardless of
    /// the reference count).
    pub fn LoadLibraryA(lib_file_name: LPCSTR) -> HMODULE;
}
