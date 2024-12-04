use crate::{BOOL, DWORD, HANDLE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateFile, GetLastError, CLRBREAK, CLRDTR, CLRRTS, SETBREAK, SETDTR, SETRTS, SETXOFF, SETXON,
};

#[link(name = "Kernel32")]
extern "system" {
    /// Directs the specified communications device to perform an extended function.
    ///
    /// # Parameters
    ///  * `file` - A handle to the communications device. The [`CreateFile`] function returns this
    ///             handle.
    ///  * `func` - The extended function to be performed. This parameter can be one of the
    ///             following values:
    ///    * [`CLRBREAK`] - Restores character transmission and places the transmission line in a
    ///                     nonbreak state. The [`CLRBREAK`] extended function code is identical to
    ///                     the [`ClearCommBreak`] function.
    ///    * [`CLRDTR`] - Clears the DTR (data-terminal-ready) signal.
    ///    * [`CLRRTS`] - Clears the RTS (request-to-send) signal.
    ///    * [`SETBREAK`] - Suspends character transmission and places the transmission line in a
    ///                     break state until the [`ClearCommBreak`] function is called (or
    ///                     [`EscapeCommFunction`] is called with the [`CLRBREAK`] extended
    ///                     function code). The [`SETBREAK`] extended function code is identical to
    ///                     the [`SetCommBreak`] function. Note that this extended function does
    ///                     not flush data that has not been transmitted.
    ///    * [`SETDTR`] - Sends the DTR (data-terminal-ready) signal.
    ///    * [`SETRTS`] - Sends the RTS (request-to-send) signal.
    ///    * [`SETXOFF`] - Causes transmission to act as if an XOFF character has been received.
    ///    * [`SETXON`] - Causes transmission to act as if an XON character has been received.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    pub fn EscapeCommFunction(file: HANDLE, func: DWORD) -> BOOL;
}
