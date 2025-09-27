use crate::{BOOL, HWND, INT_PTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{DialogBox, DialogBoxParam, GetLastError, WM_INITDIALOG};

#[link(name = "User32")]
extern "system" {
    /// Destroys a modal dialog box, causing the system to end any processing for the dialog box.
    ///
    /// # Parameters
    ///  * `dlg` - A handle to the dialog box to be destroyed.
    ///  * `result` - The value to be returned to the application from the function that created
    ///               the dialog box.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// Dialog boxes created by the [`DialogBox`], [`DialogBoxParam`], [`DialogBoxIndirect`], and
    /// [`DialogBoxIndirectParam`] functions must be destroyed using the [`EndDialog`] function. An
    /// application calls [`EndDialog`] from within the dialog box procedure; the function must not
    /// be used for any other purpose.
    ///
    /// A dialog box procedure can call [`EndDialog`] at any time, even during the processing of
    /// the [`WM_INITDIALOG`] message. If your application calls the function while
    /// [`WM_INITDIALOG`] is being processed, the dialog box is destroyed before it is shown and
    /// before the input focus is set.
    ///
    /// [`EndDialog`] does not destroy the dialog box immediately. Instead, it sets a flag and
    /// allows the dialog box procedure to return control to the system. The system checks the flag
    /// before attempting to retrieve the next message from the application queue. If the flag is
    /// set, the system ends the message loop, destroys the dialog box, and uses the value in
    /// `result` as the return value from the function that created the dialog box.
    pub fn EndDialog(dlg: HWND, result: INT_PTR) -> BOOL;
}
