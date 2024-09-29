use crate::{HDC, HWND, LPPAINTSTRUCT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{PAINTSTRUCT, WM_ERASEBKGND, WM_PAINT};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// The [`BeginPaint`] function prepares the specified window for painting and fills a
    /// [`PAINTSTRUCT`] structure with information about the painting.
    ///
    /// # Parameters
    ///  * `wnd` - Handle to the window to be repainted.
    ///  * `paint` - Pointer to the [`PAINTSTRUCT`] structure that will receive painting
    ///              information.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the handle to a display device context for
    /// the specified window.
    ///
    /// If the function fails, the return value is [`null_mut`], indicating that no display device
    /// context is available.
    ///
    /// # Remarks
    /// The [`BeginPaint`] function automatically sets the clipping region of the device context to
    /// exclude any area outside the update region. The update region is set by the
    /// [`InvalidateRect`] or [`InvalidateRgn`] function and by the system after sizing, moving,
    /// creating, scrolling, or any other operation that affects the client area. If the update
    /// region is marked for erasing, [`BeginPaint`] sends a [`WM_ERASEBKGND`] message to the
    /// window.
    ///
    /// An application should not call [`BeginPaint`] except in response to a [`WM_PAINT`] message.
    /// Each call to [`BeginPaint`] must have a corresponding call to the [`EndPaint`] function.
    ///
    /// If the caret is in the area to be painted, [`BeginPaint`] automatically hides the caret to
    /// prevent it from being erased.
    ///
    /// If the window's class has a background brush, [`BeginPaint`] uses that brush to erase the
    /// background of the update region before returning.
    pub fn BeginPaint(wnd: HWND, paint: LPPAINTSTRUCT) -> HDC;
}
