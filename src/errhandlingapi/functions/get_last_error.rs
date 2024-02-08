use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{FormatMessage, SetLastError, HRESULT, HRESULT_FROM_WIN32};

#[link(name = "Kernel32")]
extern "system" {
    /// Retrieves the calling thread's last-error code value. The last-error code is maintained on
    /// a per-thread basis. Multiple threads do not overwrite each other's last-error code.
    ///
    /// # Return Value
    /// The return value is the calling thread's last-error code.
    ///
    /// # Remarks
    /// Functions executed by the calling thread set this value by calling the [`SetLastError`]
    /// function. You should call the [`GetLastError`] function immediately when a function's
    /// return value indicates that such a call will return useful data. That is because some
    /// functions call [`SetLastError`] with a zero when they succeed, wiping out the error code
    /// set by the most recently failed function.
    ///
    /// To obtain an error string for system error codes, use the [`FormatMessage`] function.
    ///
    /// The error codes returned by a function are not part of the Windows API specification and
    /// can vary by operating system or device driver. For this reason, we cannot provide the
    /// complete list of error codes that can be returned by each function. There are also many
    /// functions whose documentation does not include even a partial list of error codes that can
    /// be returned.
    ///
    /// Error codes are 32-bit values (bit 31 is the most significant bit). Bit 29 is reserved for
    /// application-defined error codes; no system error code has this bit set. If you are defining
    /// an error code for your application, set this bit to one. That indicates that the error code
    /// has been defined by an application, and ensures that your error code does not conflict with
    /// any error codes defined by the system.
    ///
    /// To convert a system error into an [`HRESULT`] value, use the [`HRESULT_FROM_WIN32`] macro.
    pub fn GetLastError() -> DWORD;
}
