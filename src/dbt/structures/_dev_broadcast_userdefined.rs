use crate::dbt::_DEV_BROADCAST_HDR;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    WM_DEVICECHANGE,
    dbt::{DBT_USERDEFINED, DEV_BROADCAST_HDR},
};

/// Contains the user-defined event and optional data associated with the [`DBT_USERDEFINED`]
/// device event.
///
/// # Remarks
/// Because this structure contains variable length fields, use it as a template for creating a
/// pointer to a user-defined structure. Note that the structure must not contain pointers. The
/// following example shows such a user-defined structure:
/// ```no_run
/// const NAME_LENGTH: usize = 32;
/// const USER_LENGTH: usize = 50;
///  
/// #[repr(C)]
/// struct WIDGET_WARE_DEV_BROADCAST_USERDEFINED {
///     db_header: _DEV_BROADCAST_HDR,
///     sz_name: [c_char; NAME_LENGTH],
///     user_defined: [win32::BYTE; USER_LENGTH],
/// }
/// ```
#[repr(C)]
#[derive(Debug, Clone)]
pub struct _DEV_BROADCAST_USERDEFINED {
    /// Information about the device affected by a [`WM_DEVICECHANGE`] message as specified by the
    /// [`DEV_BROADCAST_HDR`] structure. Because [`_DEV_BROADCAST_USERDEFINED`] is variable length,
    /// the `size` member of the `dbh` structure must be the size in bytes of the entire structure,
    /// including the variable length portion.
    pub dbh: _DEV_BROADCAST_HDR,

    /// A pointer to a case-sensitive, null-terminated string that names the message. The string
    /// must consist of the vendor name, a backslash, followed by arbitrary user-defined
    /// null-terminated text.
    pub name: [c_char; 1],
}

impl Default for _DEV_BROADCAST_USERDEFINED {
    fn default() -> Self {
        _DEV_BROADCAST_USERDEFINED {
            dbh: DEV_BROADCAST_HDR::default(),
            name: [0],
        }
    }
}
