use crate::{DISPLAY_DEVICEA, DISPLAY_DEVICEW};

/// A pointer to a [`DISPLAY_DEVICEA`]
#[allow(non_camel_case_types)]
pub type PDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;

/// A pointer to a [`DISPLAY_DEVICEW`]
#[allow(non_camel_case_types)]
pub type PDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
