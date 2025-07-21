use std::ffi::{c_char, c_ushort};

/// A socket address
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct sockaddr {
    /// The family of the address
    pub family: c_ushort,

    /// The data defining the address
    pub data: [c_char; 14],
}

impl Default for sockaddr {
    fn default() -> Self {
        sockaddr {
            family: 0,
            data: [0; 14],
        }
    }
}
