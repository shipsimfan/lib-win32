use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::INPUT_RECORD;

/// Describes a focus event in a console [`INPUT_RECORD`] structure. These events are used internally and should be ignored.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct FOCUS_EVENT_RECORD {
    /// Reserved
    pub set_focus: BOOL,
}
