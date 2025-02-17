use crate::{DWORD, LONG, LONGLONG};
use std::fmt::{Debug, Display};

/// Represents a 64-bit signed integer value.
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union LARGE_INTEGER {
    #[allow(missing_docs)]
    pub u: LARGE_INTEGER_DUMMY,

    /// A signed 64-bit integer.
    pub quad_part: LONGLONG,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub struct LARGE_INTEGER_DUMMY {
    #[allow(missing_docs)]
    pub low_part: DWORD,

    #[allow(missing_docs)]
    pub high_part: LONG,
}

impl Default for LARGE_INTEGER {
    fn default() -> Self {
        LARGE_INTEGER { quad_part: 0 }
    }
}

impl Display for LARGE_INTEGER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(unsafe { &self.quad_part }, f)
    }
}

impl Debug for LARGE_INTEGER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(unsafe { &self.quad_part }, f)
    }
}

impl PartialEq for LARGE_INTEGER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.quad_part.eq(&other.quad_part) }
    }
}

impl Eq for LARGE_INTEGER {}

impl PartialOrd for LARGE_INTEGER {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unsafe { self.quad_part.partial_cmp(&other.quad_part) }
    }
}

impl Ord for LARGE_INTEGER {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.quad_part.cmp(&other.quad_part) }
    }
}
