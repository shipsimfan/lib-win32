use crate::{DWORD, HANDLE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CloseHandle, CreateFile, GetLastError, ReadFile, WriteFile, GENERIC_READ, GENERIC_WRITE,
    INVALID_HANDLE_VALUE, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "system" {
    /// Retrieves a handle to the specified standard device (standard input, standard output, or
    /// standard error).
    ///
    /// # Parameters
    ///  * `std_handle` - The standard device. This parameter can be one of the following values:
    ///    * [`STD_INPUT_HANDLE`] - The standard input device. Initially, this is the console input
    ///                             buffer, "CONIN$".
    ///    * [`STD_OUTPUT_HANDLE`] - The standard input device. Initially, this is the console
    ///                              input buffer, "CONOUT$".
    ///    * [`STD_ERROR_HANDLE`] - The standard input device. Initially, this is the console input
    ///                             buffer, "CONOUT$".
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the specified device, or a
    /// redirected handle set by a previous call to [`SetStdHandle`]. The handle has
    /// [`GENERIC_READ`] and [`GENERIC_WRITE`] access rights, unless the application has used
    /// [`SetStdHandle`] to set a standard handle with lesser access.
    ///
    /// If the function fails, the return value is [`INVALID_HANDLE_VALUE`]. To get extended error
    /// information, call [`GetLastError`].
    ///
    /// If an application does not have associated standard handles, such as a service running on
    /// an interactive desktop, and has not redirected them, the return value is [`null_mut`].
    ///
    /// # Remarks
    /// Handles returned by [`GetStdHandle`] can be used by applications that need to read from or
    /// write to the console. When a console is created, the standard input handle is a handle to
    /// the console's input buffer, and the standard output and standard error handles are handles
    /// of the console's active screen buffer. These handles can be used by the [`ReadFile`] and
    /// [`WriteFile`] functions, or by any of the console functions that access the console input
    /// buffer or a screen buffer (for example, the [`ReadConsoleInput`], [`WriteConsole`], or
    /// [`GetConsoleScreenBufferInfo`] functions).
    ///
    /// The standard handles of a process may be redirected by a call to [`SetStdHandle`], in which
    /// case [`GetStdHandle`] returns the redirected handle. If the standard handles have been
    /// redirected, you can specify the "CONIN$" value in a call to the [`CreateFile`] function to
    /// get a handle to a console's input buffer. Similarly, you can specify the CONOUT$ value to
    /// get a handle to a console's active screen buffer.
    ///
    /// The standard handles of a process on entry of the main method are dictated by the
    /// configuration of the "/SUBSYSTEM" flag passed to the linker when the application was built.
    /// Specifying "/SUBSYSTEM:CONSOLE" requests that the operating system fill the handles with a
    /// console session on startup, if the parent didn't already fill the standard handle table by
    /// inheritance. On the contrary, "/SUBSYSTEM:WINDOWS" implies that the application does not
    /// need a console and will likely not be making use of the standard handles. More information
    /// on handle inheritance can be found in the documentation for [`STARTF_USESTDHANDLES`].
    ///
    /// Some applications operate outside the boundaries of their declared subsystem; for instance,
    /// a "/SUBSYSTEM:WINDOWS" application might check/use standard handles for logging or
    /// debugging purposes but operate normally with a graphical user interface. These applications
    /// will need to carefully probe the state of standard handles on startup and make use of
    /// [`AttachConsole`], [`AllocConsole`], and [`FreeConsole`] to add/remove a console if
    /// desired.
    ///
    /// Some applications may also vary their behavior on the type of inherited handle.
    /// Disambiguating the type between console, pipe, file, and others can be performed with
    /// [`GetFileType`].
    ///
    /// ## Handle disposal
    /// It is not required to [`CloseHandle`] when done with the handle retrieved from
    /// [`GetStdHandle`]. The returned value is simply a copy of the value stored in the process
    /// table. The process itself is generally considered the owner of these handles and their
    /// lifetime. Each handle is placed in the table on creation depending on the inheritance and
    /// launch specifics of the [`CreateProcess`] call and will be freed when the process is
    /// destroyed.
    ///
    /// Manual manipulation of the lifetime of these handles may be desirable for an application
    /// intentionally trying to replace them or block other parts of the process from using them.
    /// As a [`HANDLE`] can be cached by running code, that code will not necessarily pick up
    /// changes made via SetStdHandle. Closing the handle explicitly via [`CloseHandle`] will close
    /// it process-wide and the next usage of any cached reference will encounter an error.
    ///
    /// Guidance for replacing a standard handle in the process table would be to get the existing
    /// [`HANDLE`] from the table with [`GetStdHandle`], use [`SetStdHandle`] to place a new
    /// [`HANDLE`] in that is opened with [`CreateFile`] (or a similar function), then to close the
    /// retrieved handle.
    ///
    /// There is no validation of the values stored as handles in the process table by either the
    /// [`GetStdHandle`] or [`SetStdHandle`] functions. Validation is performed at the time of the
    /// actual read/write operation such as [`ReadFile`] or [`WriteFile`].
    ///
    /// ## Attach/detach behavior
    /// When attaching to a new console, standard handles are always replaced with console handles
    /// unless [`STARTF_USESTDHANDLES`] was specified during process creation.
    ///
    /// If the existing value of the standard handle is [`null_mut`], or the existing value of the
    /// standard handle looks like a console pseudohandle, the handle is replaced with a console
    /// handle.
    ///
    /// When a parent uses both [`CREATE_NEW_CONSOLE`] and [`STARTF_USESTDHANDLES`] to create a
    /// console process, standard handles will not be replaced unless the existing value of the
    /// standard handle is [`null_mut`] or a console pseudohandle.
    pub fn GetStdHandle(std_handle: DWORD) -> HANDLE;
}
