use crate::{BOOL, HDC};

#[link(name = "Gdi32")]
unsafe extern "system" {
    /// The [`DeleteDC`] function deletes the specified device context (DC).
    ///
    /// # Parameters
    ///  * `dc` - A handle to the device context.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero.
    ///
    /// # Remarks
    /// An application must not delete a DC whose handle was obtained by calling the [`GetDC`]
    /// function. Instead, it must call the [`ReleaseDC`] function to free the DC.
    pub fn DeleteDC(dc: HDC) -> BOOL;
}
