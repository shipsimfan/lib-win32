use crate::{BOOL, HWND, PAINTSTRUCT};

// rustdoc imports
#[allow(unused_imports)]
use crate::BeginPaint;

#[link(name = "User32")]
unsafe extern "system" {
    /// The [`EndPaint`] function marks the end of painting in the specified window. This function
    /// is required for each call to the [`BeginPaint`] function, but only after painting is
    /// complete.
    ///
    /// # Parameters
    ///  * `wnd` - Handle to the window that has been repainted.
    ///  * `paint` - Pointer to a [`PAINTSTRUCT`] structure that contains the painting information
    ///              retrieved by [`BeginPaint`].
    ///
    /// # Return Value
    /// The return value is always nonzero.
    ///
    /// # Remarks
    /// If the caret was hidden by [`BeginPaint`], [`EndPaint`] restores the caret to the screen.
    ///
    /// [`EndPaint`] releases the display device context that [`BeginPaint`] retrieved.
    pub fn EndPaint(wnd: HWND, paint: *const PAINTSTRUCT) -> BOOL;
}
