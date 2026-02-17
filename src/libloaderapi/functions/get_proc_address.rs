use crate::{HMODULE, LPCSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, LoadLibrary, LoadLibraryEx};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves the address of an exported function (also known as a procedure) or variable from
    /// the specified dynamic-link library (DLL).
    ///
    /// # Parameters
    ///  * `module` - A handle to the DLL module that contains the function or variable. The
    ///               [`LoadLibrary`], [`LoadLibraryEx`], [`LoadPackagedLibrary`], or
    ///               [`GetModuleHandle`] function returns this handle. The [`GetProcAddress`]
    ///               function does not retrieve addresses from modules that were loaded using the
    ///               [`LOAD_LIBRARY_AS_DATAFILE`] flag.
    ///  * `proc_name` - The function or variable name, or the function's ordinal value. If this
    ///                  parameter is an ordinal value, it must be in the low-order word; the
    ///                  high-order word must be zero.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the address of the exported function or
    /// variable.
    ///
    /// If the function fails, the return value is [`None`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// The spelling and case of a function name pointed to by `proc_name` must be identical to
    /// that in the `EXPORTS` statement of the source DLL's module-definition (.def) file. The
    /// exported names of functions may differ from the names you use when calling these functions
    /// in your code. This difference is hidden by macros used in the SDK header files.
    ///
    /// The `proc_name` parameter can identify the DLL function by specifying an ordinal value
    /// associated with the function in the `EXPORTS` statement. [`GetProcAddress`] verifies that
    /// the specified ordinal is in the range 1 through the highest ordinal value exported in the
    /// .def file. The function then uses the ordinal as an index to read the function's address
    /// from a function table.
    ///
    /// If the .def file does not number the functions consecutively from 1 to N (where N is the
    /// number of exported functions), an error can occur where GetProcAddress returns an invalid,
    /// non-NULL address, even though there is no function with the specified ordinal.
    pub fn GetProcAddress(module: HMODULE, proc_name: LPCSTR) -> Option<unsafe extern "system" fn()>;
}
