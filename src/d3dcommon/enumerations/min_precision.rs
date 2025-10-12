/// Values that indicate the minimum desired interpolation precision.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D_MIN_PRECISION {
    /// Default minimum precision, which is 32-bit precision.
    Default = 0,

    /// Minimum precision is min16float, which is 16-bit floating point.
    Float16 = 1,

    /// Minimum precision is min10float, which is 10-bit floating point.
    Float2_8 = 2,

    /// Reserved
    Reserved = 3,

    /// Minimum precision is min16int, which is 16-bit signed integer.
    SInt16 = 4,

    /// Minimum precision is min16uint, which is 16-bit unsigned integer.
    UInt16 = 5,

    /// Minimum precision is any 16-bit value.
    Any16 = 0xF0,

    /// Minimum precision is any 10-bit value.
    Any10 = 0xF1,
}
