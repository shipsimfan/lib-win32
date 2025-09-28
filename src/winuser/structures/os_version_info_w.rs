use crate::{DWORD, WCHAR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetVersionEx, OSVERSIONINFOEX};

/// Contains operating system version information. The information includes major and minor version
/// numbers, a build number, a platform identifier, and descriptive text about the operating
/// system. This structure is used with the [`GetVersionEx`] function.
///
/// To obtain additional version information, use the [`OSVERSIONINFOEX`] structure with
/// [`GetVersionEx`] instead.
///
/// # Remarks
/// Relying on version information is not the best way to test for a feature. Instead, refer to the
/// documentation for the feature of interest.
///
/// If you must require a particular operating system, be sure to use it as a minimum supported
/// version, rather than design the test for the one operating system. This way, your detection
/// code will continue to work on future versions of Windows.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OSVERSIONINFOW {
    /// The size of this data structure, in bytes. Set this member to
    /// `std::mem::size_of::<OSVERSIONINFO>()`.
    pub os_version_info_size: DWORD,

    /// The major version number of the operating system.
    pub major_version: DWORD,

    /// The minor version number of the operating system.
    pub minor_version: DWORD,

    /// The build number of the operating system.
    pub build_number: DWORD,

    /// The operating system platform. This member can be the following value:
    ///  * [`VER_PLATFORM_WIN32_NT`] -  The operating system is Windows 7, Windows Server 2008,
    ///                                 Windows Vista, Windows Server 2003, Windows XP, or Windows
    ///                                 2000.
    pub platform_id: DWORD,

    /// A null-terminated string, such as "Service Pack 3", that indicates the latest Service Pack
    /// installed on the system. If no Service Pack has been installed, the string is empty.
    pub csd_version: [WCHAR; 128],
}

impl Default for OSVERSIONINFOW {
    fn default() -> Self {
        OSVERSIONINFOW {
            os_version_info_size: 0,
            major_version: 0,
            minor_version: 0,
            build_number: 0,
            platform_id: 0,
            csd_version: [0; 128],
        }
    }
}
