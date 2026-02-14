use crate::{BOOL, DPI_AWARENESS_CONTEXT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, DPI_AWARENESS_CONTEXT_UNAWARE, ERROR_ACCESS_DENIED, ERROR_INVALID_PARAMETER,
    FALSE, TRUE,
};

#[link(name = "User32")]
extern "system" {
    /// Sets the current process to a specified dots per inch (dpi) awareness context. The DPI
    /// awareness contexts are from the [`DPI_AWARENESS_CONTEXT`] value.
    ///
    /// # Parameters
    ///  * `value` - A [`DPI_AWARENESS_CONTEXT`] handle to set.
    ///
    /// # Return Value
    /// This function returns [`TRUE`] if the operation was successful, and [`FALSE`] otherwise. To
    /// get extended error information, call [`GetLastError`].
    ///
    /// Possible errors are [`ERROR_INVALID_PARAMETER`] for an invalid input, and
    /// [`ERROR_ACCESS_DENIED`] if the default API awareness mode for the process has already been
    /// set (via a previous API call or within the application manifest).
    ///
    /// # Remarks
    /// This API is a more advanced version of the previously existing [`SetProcessDpiAwareness`]
    /// API, allowing for the process default to be set to the finer-grained
    /// [`DPI_AWARENESS_CONTEXT`] values. Most importantly, this allows you to programmatically set
    /// Per Monitor v2 as the process default value, which is not possible with the previous API.
    ///
    /// This method sets the default [`DPI_AWARENESS_CONTEXT`] for all threads within an
    /// application. Individual threads can have their DPI awareness changed from the default with
    /// the [`SetThreadDpiAwarenessContext`] method.
    ///
    /// You must call this API before you call any APIs that depend on the DPI awareness (including
    /// before creating any UI in your process). Once API awareness is set for an app, any future
    /// calls to this API will fail. This is true regardless of whether you set the DPI awareness
    /// in the manifest or by using this API.
    ///
    /// If the DPI awareness level is not set, the default value is
    /// [`DPI_AWARENESS_CONTEXT_UNAWARE`].
    pub fn SetProcessDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> BOOL;
}
