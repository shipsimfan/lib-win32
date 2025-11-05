use crate::DWORD;

/// Defines the raw input data coming from the specified keyboard.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct RID_DEVICE_INFO_KEYBOARD {
    /// The type of keyboard.
    pub r#type: DWORD,

    /// The vendor-specific subtype of the keyboard.
    pub sub_type: DWORD,

    /// The scan code mode. Usually 1, which means that Scan Code Set 1 is used.
    pub keyboard_mode: DWORD,

    /// The number of function keys on the keyboard.
    pub number_of_function_keys: DWORD,

    /// The number of LED indicators on the keyboard.
    pub number_of_function_indicators: DWORD,

    /// The total number of keys on the keyboard.
    pub number_of_keys_total: DWORD,
}

impl Default for RID_DEVICE_INFO_KEYBOARD {
    fn default() -> Self {
        RID_DEVICE_INFO_KEYBOARD {
            r#type: 0,
            sub_type: 0,
            keyboard_mode: 0,
            number_of_function_keys: 0,
            number_of_function_indicators: 0,
            number_of_keys_total: 0,
        }
    }
}
