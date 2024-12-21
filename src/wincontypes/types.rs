use crate::{INPUT_RECORD, SMALL_RECT};

/// A pointer to an [`INPUT_RECORD`]
#[allow(non_camel_case_types)]
pub type PINPUT_RECORD = *mut INPUT_RECORD;

/// A pointer to a [`SMALL_RECT`]
#[allow(non_camel_case_types)]
pub type PSMALL_RECT = *mut SMALL_RECT;
