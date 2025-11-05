use std::ffi::c_void;

/// The _HIDP_PREPARSED_DATA structure contains a top-level collection's preparsed data.
///
/// A user-mode application calls [`HidD_GetPreparsedData`] to obtain a top-level collection's
/// preparsed data in a variable length [`_HIDP_PREPARSED_DATA`] structure.
///
/// A kernel-mode driver uses an [`IOCTL_HID_GET_COLLECTION_DESCRIPTOR`] request to obtain a
/// pointer to a top-level collection's preparsed data.
///
/// The internal structure of a [`_HIDP_PREPARSED_DATA`] structure is reserved for internal system
/// use.
#[allow(non_camel_case_types)]
pub type _HIDP_PREPARSED_DATA = c_void;

/// A pointer to a [`_HIDP_PREPARSED_DATA`]
#[allow(non_camel_case_types)]
pub type PHIDP_PREPARSED_DATA = *mut _HIDP_PREPARSED_DATA;
