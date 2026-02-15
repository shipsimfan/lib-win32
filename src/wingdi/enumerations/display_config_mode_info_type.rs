// rustdoc imports
#[allow(unused_imports)]
use crate::{DISPLAYCONFIG_DESKTOP_IMAGE_INFO, DISPLAYCONFIG_MODE_INFO};

/// The [`DISPLAYCONFIG_MODE_INFO_TYPE`] enumeration specifies that the information that is
/// contained within the [`DISPLAYCONFIG_MODE_INFO`] structure is either source or target mode.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DISPLAYCONFIG_MODE_INFO_TYPE {
    /// Indicates that the [`DISPLAYCONFIG_MODE_INFO`] structure contains source mode information.
    Source = 1,

    /// Indicates that the [`DISPLAYCONFIG_MODE_INFO`] structure contains target mode information.
    Target = 2,

    /// Indicates that the [`DISPLAYCONFIG_MODE_INFO`] structure contains a valid
    /// [`DISPLAYCONFIG_DESKTOP_IMAGE_INFO`] structure. Supported starting in Windows 10.
    DesktopImage = 3,

    /// Forces this enumeration to compile to 32 bits in size. Without this value, some compilers
    /// would allow this enumeration to compile to a size other than 32 bits. You should not use
    /// this value.
    ForceUint32 = 0xFFFFFFFF,
}
