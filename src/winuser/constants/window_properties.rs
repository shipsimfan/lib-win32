use std::ffi::c_int;

/// Sets a new extended window style.
pub const GWL_EXSTYLE: c_int = -20;

/// Sets a new application instance handle.
pub const GWLP_HINSTANCE: c_int = -6;

/// Sets a new identifier of the child window. The window cannot be a top-level window.
pub const GWLP_ID: c_int = -12;

/// Sets a new window style.
pub const GWL_STYLE: c_int = -16;

/// Sets the user data associated with the window. This data is intended for use by the application that created the window. Its value is initially zero.
pub const GWLP_USERDATA: c_int = -21;

/// Sets a new address for the window procedure.
pub const GWLP_WNDPROC: c_int = -4;
