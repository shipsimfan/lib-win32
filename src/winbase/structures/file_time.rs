use crate::DWORD;

/// Contains a 64-bit value representing the number of 100-nanosecond intervals since January 1,
/// 1601 (UTC).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FILETIME {
    /// The low-order part of the file time.
    pub low_date_time: DWORD,

    /// The high-order part of the file time.
    pub high_date_time: DWORD,
}
