// rustdoc imports
#[allow(unused_imports)]
use crate::{CoInitializeEx, S_FALSE};

#[link(name = "Ole32")]
unsafe extern "system" {
    /// Closes the COM library on the current thread, unloads all DLLs loaded by the thread, frees
    /// any other resources that the thread maintains, and forces all RPC connections on the thread
    /// to close.
    ///
    /// # Remarks
    /// A thread must call [`CoUninitialize`] once for each successful call it has made to the
    /// [`CoInitialize`] or [`CoInitializeEx`] function, including any call that returns
    /// [`S_FALSE`]. Only the [`CoUninitialize`] call corresponding to the [`CoInitialize`] or
    /// [`CoInitializeEx`] call that initialized the library can close it.
    ///
    /// Calls to [`OleInitialize`] must be balanced by calls to [`OleUninitialize`]. The
    /// [`OleUninitialize`] function calls [`CoUninitialize`] internally, so applications that call
    /// [`OleUninitialize`] do not also need to call [`CoUninitialize`].
    ///
    /// [`CoUninitialize`] should be called on application shutdown, as the last call made to the
    /// COM library after the application hides its main windows and falls through its main message
    /// loop. If there are open conversations remaining, [`CoUninitialize`] starts a modal message
    /// loop and dispatches any pending messages from the containers or server for this COM
    /// application. By dispatching the messages, [`CoUninitialize`] ensures that the application
    /// does not quit before receiving all of its pending messages. Non-COM messages are discarded.
    ///
    /// Because there is no way to control the order in which in-process servers are loaded or
    /// unloaded, do not call [`CoInitialize`], [`CoInitializeEx`], or [`CoUninitialize`] from the
    /// `DllMain` function.
    pub fn CoUninitialize();
}
