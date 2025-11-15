use crate::{ULONG, WCHAR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    ntddk::RtlGetVersion,
    wdm::{RTL_OSVERSIONINFOEXW, RTL_OSVERSIONINFOW},
    VER_PLATFORM_WIN32_NT,
};

/// The [`RTL_OSVERSIONINFOW`] structure contains operating system version information. The
/// information includes major and minor version numbers, a build number, a platform identifier,
/// and descriptive text about the operating system. The [`RTL_OSVERSIONINFOW`] structure is used
/// with [`RtlGetVersion`].
///
/// # Remarks
/// For a list of the major and minor version numbers for the various versions of Windows, see
/// [`RTL_OSVERSIONINFOEXW`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct OSVERSIONINFOW {
    /// The size in bytes of an [`RTL_OSVERSIONINFOW`] structure. This member must be set before
    /// the structure is used with [`RtlGetVersion`].
    pub os_version_info_size: ULONG,

    /// The major version number of the operating system. For example, for Windows 2000, the major
    /// version number is five. For more information, see [`RTL_OSVERSIONINFOEXW`].
    pub major_version: ULONG,

    /// The major version number of the operating system. For example, for Windows 2000, the major
    /// version number is five. For more information, see [`RTL_OSVERSIONINFOEXW`].
    pub minor_version: ULONG,

    /// The build number of the operating system.
    pub build_number: ULONG,

    /// The operating system platform. For Microsoft Win32 on NT-based operating systems,
    /// [`RtlGetVersion`] returns the value [`VER_PLATFORM_WIN32_NT`].
    pub platform_id: ULONG,

    /// The service-pack version string. This member contains a null-terminated string, such as
    /// "Service Pack 3", which indicates the latest service pack installed on the system. If no
    /// service pack is installed, [`RtlGetVersion`] might not initialize this string. Initialize
    /// `csd_version` to zero (empty string) before the call to [`RtlGetVersion`].
    pub csd_version: [WCHAR; 128],
}

impl Default for OSVERSIONINFOW {
    fn default() -> Self {
        OSVERSIONINFOW {
            os_version_info_size: std::mem::size_of::<OSVERSIONINFOW>() as _,
            major_version: 0,
            minor_version: 0,
            build_number: 0,
            platform_id: 0,
            csd_version: [0; 128],
        }
    }
}
