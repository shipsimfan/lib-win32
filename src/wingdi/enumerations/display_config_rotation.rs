/// The [`DISPLAYCONFIG_ROTATION`] enumeration specifies the clockwise rotation of the display.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DISPLAYCONFIG_ROTATION {
    /// Indicates that rotation is 0 degrees—landscape mode.
    Identity = 1,

    /// Indicates that rotation is 90 degrees clockwise—portrait mode.
    Rotate90 = 2,

    /// Indicates that rotation is 180 degrees clockwise—inverted landscape mode.
    Rotate180 = 3,

    /// Indicates that rotation is 270 degrees clockwise—inverted portrait mode.
    Rotate270 = 4,

    /// Forces this enumeration to compile to 32 bits in size. Without this value, some compilers
    /// would allow this enumeration to compile to a size other than 32 bits. You should not use
    /// this value.
    ForceUint32 = 0xFFFFFFFF,
}
