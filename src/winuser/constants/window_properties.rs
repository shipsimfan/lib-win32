use crate::{DLGPROC, LRESULT};
use std::ffi::c_int;

/// Sets a new extended window style.
pub const GWL_EXSTYLE: c_int = -20;

/// Sets a new application instance handle.
pub const GWL_HINSTANCE: c_int = -6;

/// Sets a new application instance handle.
pub const GWLP_HINSTANCE: c_int = -6;

/// Sets a new identifier of the child window. The window cannot be a top-level window.
pub const GWL_ID: c_int = -12;

/// Sets a new identifier of the child window. The window cannot be a top-level window.
pub const GWLP_ID: c_int = -12;

/// Sets a new owner for a top-level window.
pub const GWL_HWNDPARENT: c_int = -8;

/// Sets a new owner for a top-level window.
pub const GWLP_HWNDPARENT: c_int = -8;

/// Sets a new window style.
pub const GWL_STYLE: c_int = -16;

/// Sets the user data associated with the window. This data is intended for use by the application
/// that created the window. Its value is initially zero.
pub const GWL_USERDATA: c_int = -21;

/// Sets the user data associated with the window. This data is intended for use by the application
/// that created the window. Its value is initially zero.
pub const GWLP_USERDATA: c_int = -21;

/// Sets a new address for the window procedure.
pub const GWL_WNDPROC: c_int = -4;

/// Sets a new address for the window procedure.
pub const GWLP_WNDPROC: c_int = -4;

///  Sets the return value of a message processed in the dialog box procedure.
pub const DWL_MSGRESULT: c_int = 0;

///  Sets the return value of a message processed in the dialog box procedure.
pub const DWLP_MSGRESULT: c_int = 0;

///  Sets the new address of the dialog box procedure.
pub const DWL_DLGPROC: c_int = DWLP_MSGRESULT + std::mem::size_of::<LRESULT>() as c_int;

///  Sets the new pointer to the dialog box procedure.
pub const DWLP_DLGPROC: c_int = DWLP_MSGRESULT + std::mem::size_of::<LRESULT>() as c_int;

///  Sets new extra information that is private to the application, such as handles or pointers.
pub const DWLP_USER: c_int = DWLP_DLGPROC + std::mem::size_of::<DLGPROC>() as c_int;

///  Sets new extra information that is private to the application, such as handles or pointers.
pub const DWL_USER: c_int = DWLP_DLGPROC + std::mem::size_of::<DLGPROC>() as c_int;
