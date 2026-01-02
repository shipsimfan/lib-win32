use crate::WPARAM;

/// The window has been resized, but neither the [`SIZE_MINIMIZED`] nor [`SIZE_MAXIMIZED`] value
/// applies.
pub const SIZE_RESTORED: WPARAM = 0;

/// The window has been minimized.
pub const SIZE_MINIMIZED: WPARAM = 1;

/// The window has been maximized.
pub const SIZE_MAXIMIZED: WPARAM = 2;

/// Message is sent to all pop-up windows when some other window has been restored to its former
/// size.
pub const SIZE_MAXSHOW: WPARAM = 3;

/// Message is sent to all pop-up windows when some other window is maximized.
pub const SIZE_MAXHIDE: WPARAM = 4;
