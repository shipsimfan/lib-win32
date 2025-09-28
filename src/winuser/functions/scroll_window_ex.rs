use crate::{HRGN, HWND, LPRECT, RECT, UINT};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, CS_CLASSDC, CS_OWNDC, WM_ERASEBKGND, WM_MOVE, WM_PAINT, WS_CLIPCHILDREN,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "User32")]
extern "system" {
    /// The [`ScrollWindowEx`] function scrolls the contents of the specified window's client area.
    ///
    /// # Parameters
    ///  * `wnd` - Handle to the window where the client area is to be scrolled.
    ///  * `dx` - Specifies the amount, in device units, of horizontal scrolling. This parameter
    ///           must be a negative value to scroll to the left.
    ///  * `dy` - Specifies the amount, in device units, of vertical scrolling. This parameter must
    ///           be a negative value to scroll up.
    ///  * `scroll` - Pointer to a [`RECT`] structure that specifies the portion of the client area
    ///               to be scrolled. If this parameter is [`null`], the entire client area is
    ///               scrolled.
    ///  * `clip` - Pointer to a [`RECT`] structure that contains the coordinates of the clipping
    ///             rectangle. Only device bits within the clipping rectangle are affected. Bits
    ///             scrolled from the outside of the rectangle to the inside are painted; bits
    ///             scrolled from the inside of the rectangle to the outside are not painted. This
    ///             parameter may be [`null`].
    ///  * `rgn_update` - Handle to the region that is modified to hold the region invalidated by
    ///                   scrolling. This parameter may be [`null_mut`].
    ///  * `rc_update` - Pointer to a [`RECT`] structure that receives the boundaries of the
    ///                  rectangle invalidated by scrolling. This parameter may be [`null_mut`].
    ///  * `flags` - Specifies flags that control scrolling. This parameter can be a combination of
    ///              the following values:
    ///    * [`SW_ERASE`] - Erases the newly invalidated region by sending a [`WM_ERASEBKGND`]
    ///                     message to the window when specified with the [`SW_INVALIDATE`] flag.
    ///    * [`SW_INVALIDATE`] - Invalidates the region identified by the `rgn_update` parameter
    ///                          after scrolling.
    ///    * [`SW_SCROLLCHILDREN`] - Scrolls all child windows that intersect the rectangle pointed
    ///                              to by the `scroll` parameter. The child windows are scrolled
    ///                              by the number of pixels specified by the `dx` and `dy`
    ///                              parameters. The system sends a [`WM_MOVE`] message to all
    ///                              child windows that intersect the `scroll` rectangle, even if
    ///                              they do not move.
    ///    * [`SW_SMOOTHSCROLL`] - Scrolls using smooth scrolling. Use the [`HIWORD`] portion of
    ///                            the flags parameter to indicate how much time, in milliseconds,
    ///                            the smooth-scrolling operation should take.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`SIMPLEREGION`]
    /// (rectangular invalidated region), [`COMPLEXREGION`] (nonrectangular invalidated region;
    /// overlapping rectangles), or [`NULLREGION`] (no invalidated region).
    ///
    /// If the function fails, the return value is [`ERROR`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// If the [`SW_INVALIDATE`] and [`SW_ERASE`] flags are not specified, [`ScrollWindowEx`] does
    /// not invalidate the area that is scrolled from. If either of these flags is set,
    /// [`ScrollWindowEx`] invalidates this area. The area is not updated until the application
    /// calls the [`UpdateWindow`] function, calls the [`RedrawWindow`] function (specifying the
    /// [`RDW_UPDATENOW`] or [`RDW_ERASENOW`] flag), or retrieves the [`WM_PAINT`] message from the
    /// application queue.
    ///
    /// If the window has the [`WS_CLIPCHILDREN`] style, the returned areas specified by
    /// `rgn_update` and `rc_update` represent the total area of the scrolled window that must be
    /// updated, including any areas in child windows that need updating.
    ///
    /// If the [`SW_SCROLLCHILDREN`] flag is specified, the system does not properly update the
    /// screen if part of a child window is scrolled. The part of the scrolled child window that
    /// lies outside the source rectangle is not erased and is not properly redrawn in its new
    /// destination. To move child windows that do not lie completely within the rectangle
    /// specified by `scroll`, use the [`DeferWindowPos`] function. The cursor is repositioned
    /// if the [`SW_SCROLLCHILDREN`] flag is set and the caret rectangle intersects the scroll
    /// rectangle.
    ///
    /// All input and output coordinates (for `scroll`, `clip`, `rc_update`, and `rgn_update`) are
    /// determined as client coordinates, regardless of whether the window has the [`CS_OWNDC`] or
    /// [`CS_CLASSDC`] class style. Use the [`LPtoDP`] and [`DPtoLP`] functions to convert to and
    /// from logical coordinates, if necessary.
    pub fn ScrollWindowEx(
        wnd: HWND,
        dx: c_int,
        dy: c_int,
        scroll: *const RECT,
        clip: *const RECT,
        rgn_update: HRGN,
        rc_update: LPRECT,
        flags: UINT,
    ) -> c_int;
}
