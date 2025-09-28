use std::ffi::c_int;

/// Start at the lower-left corner of the work area.
pub const ARW_BOTTOMLEFT: c_int = 0x0000;

/// Start at the lower-right corner of the work area.
pub const ARW_BOTTOMRIGHT: c_int = 0x0001;

/// Start at the upper-left corner of the work area.
pub const ARW_TOPLEFT: c_int = 0x0002;

/// Start at the upper-right corner of the work area.
pub const ARW_TOPRIGHT: c_int = 0x0003;

/// Arrange left (valid with [`ARW_BOTTOMRIGHT`] and [`ARW_TOPRIGHT`] only).
pub const ARW_LEFT: c_int = 0x0000;

/// Arrange right (valid with [`ARW_BOTTOMLEFT`] and [`ARW_TOPLEFT`] only).
pub const ARW_RIGHT: c_int = 0x0000;

/// Arrange up (valid with [`ARW_BOTTOMLEFT`] and [`ARW_BOTTOMRIGHT`] only).
pub const ARW_UP: c_int = 0x0004;

/// Arrange down (valid with [`ARW_TOPLEFT`] and [`ARW_TOPRIGHT`] only).
pub const ARW_DOWN: c_int = 0x0004;

/// Hide minimized windows by moving them off the visible area of the screen.
pub const ARW_HIDE: c_int = 0x0008;
