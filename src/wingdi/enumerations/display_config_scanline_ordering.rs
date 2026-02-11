/// The [`DISPLAYCONFIG_SCANLINE_ORDERING`] enumeration specifies the method that the display uses
/// to create an image on a screen.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DISPLAYCONFIG_SCANLINE_ORDERING {
    /// Indicates that scan-line ordering of the output is unspecified. The caller can only set the
    /// `scanline_ordering` member of the [`DISPLAYCONFIG_PATH_TARGET_INFO`] structure in a call to
    /// the [`SetDisplayConfig`] function to [`DISPLAYCONFIG_SCANLINE_ORDERING::Unspecified`] if
    /// the caller also set the refresh rate denominator and numerator of the `refresh_rate` member
    /// both to zero. In this case, [`SetDisplayConfig`] uses the best refresh rate it can find.
    Unspecified = 0,

    /// Indicates that the output is a progressive image.
    Progressive = 1,

    /// Indicates that the output is an interlaced image that is created beginning with the upper
    /// field.
    Interlaced = 2,

    /// Indicates that the output is an interlaced image that is created beginning with the lower
    /// field.
    InterlacedLowerFieldFirst = 3,

    /// Forces this enumeration to compile to 32 bits in size. Without this value, some compilers
    /// would allow this enumeration to compile to a size other than 32 bits. You should not use
    /// this value.
    ForceUint32 = 0xFFFFFFFF,
}
