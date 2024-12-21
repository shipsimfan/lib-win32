use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::INPUT_RECORD;

/// Describes a menu event in a console [`INPUT_RECORD`] structure. These events are used
/// internally and should be ignored.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct MENU_EVENT_RECORD {
    /// Reserved.
    pub command_id: UINT,
}
