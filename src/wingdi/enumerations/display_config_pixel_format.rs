// rustdoc imports
#[allow(unused_imports)]
use crate::QueryDisplayConfig;

/// The [`DISPLAYCONFIG_PIXELFORMAT`] enumeration specifies pixel format in various bits per pixel
/// (BPP) values.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DISPLAYCONFIG_PIXELFORMAT {
    /// Indicates 8 BPP format.
    _8Bpp = 1,

    /// Indicates 16 BPP format.
    _16Bpp = 2,

    /// Indicates 24 BPP format.
    _24Bpp = 3,

    /// Indicates 32 BPP format.
    _32Bpp = 4,

    /// Indicates that the current display is not an 8, 16, 24, or 32 BPP GDI desktop mode. For
    /// example, a call to the [`QueryDisplayConfig`] function returns
    /// [`DISPLAYCONFIG_PIXELFORMAT::NonGdi`] if a DirectX application previously set the desktop
    /// to A2R10G10B10 format. A call to the [`SetDisplayConfig`] function fails if any pixel
    /// formats for active paths are set to [`DISPLAYCONFIG_PIXELFORMAT::NonGdi`].
    NonGdi = 5,

    /// Forces this enumeration to compile to 32 bits in size. Without this value, some compilers
    /// would allow this enumeration to compile to a size other than 32 bits. You should not use
    /// this value.
    ForceUint32 = 0xFFFFFFFF,
}
