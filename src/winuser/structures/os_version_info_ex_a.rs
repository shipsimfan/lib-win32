use crate::{BYTE, CHAR, DWORD, WORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::GetVersionEx;

/// Contains operating system version information. The information includes major and minor version
/// numbers, a build number, a platform identifier, and information about product suites and the
/// latest Service Pack installed on the system. This structure is used with the [`GetVersionEx`]
/// and [`VerifyVersionInfo`] functions.
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
pub struct OSVERSIONINFOEXA {
    /// The size of this data structure, in bytes. Set this member to
    /// `std::mem::size_of::<OSVERSIONINFOEX>()`.
    pub os_version_info_size: DWORD,

    /// The major version number of the operating system.
    pub major_version: DWORD,

    /// The minor version number of the operating system.
    pub minor_version: DWORD,

    /// The build number of the operating system.
    pub build_number: DWORD,

    /// The operating system platform. This member can be [`VER_PLATFORM_WIN32_NT`] (2).
    pub platform_id: DWORD,

    /// A null-terminated string, such as "Service Pack 3", that indicates the latest Service Pack
    /// installed on the system. If no Service Pack has been installed, the string is empty.
    pub csd_version: [CHAR; 128],

    /// The major version number of the latest Service Pack installed on the system. For example,
    /// for Service Pack 3, the major version number is 3. If no Service Pack has been installed,
    /// the value is zero.
    pub service_pack_major: WORD,

    /// The minor version number of the latest Service Pack installed on the system. For example,
    /// for Service Pack 3, the minor version number is 0.
    pub service_pack_minor: WORD,

    /// A bit mask that identifies the product suites available on the system. This member can be a
    /// combination of the following values:
    ///  * [`VER_SUITE_BACKOFFICE`] - Microsoft BackOffice components are installed.
    ///  * [`VER_SUITE_BLADE`] - Windows Server 2003, Web Edition is installed.
    ///  * [`VER_SUITE_COMPUTE_SERVER`] - Windows Server 2003, Compute Cluster Edition is installed.
    ///  * [`VER_SUITE_DATACENTER`] - Windows Server 2008 Datacenter, Windows Server 2003,
    ///                               Datacenter Edition, or Windows 2000 Datacenter Server is
    ///                               installed.
    ///  * [`VER_SUITE_ENTERPRISE`] - Windows Server 2008 Enterprise, Windows Server 2003,
    ///                               Enterprise Edition, or Windows 2000 Advanced Server is
    ///                               installed.
    ///  * [`VER_SUITE_EMBEDDEDNT`] - Windows XP Embedded is installed.
    ///  * [`VER_SUITE_PERSONAL`] - Windows Vista Home Premium, Windows Vista Home Basic, or
    ///                             Windows XP Home Edition is installed.
    ///  * [`VER_SUITE_SINGLEUSERTS`] - Remote Desktop is supported, but only one interactive
    ///                                 session is supported. This value is set unless the system
    ///                                 is running in application server mode.
    ///  * [`VER_SUITE_SMALLBUSINESS`] - Microsoft Small Business Server was once installed on the
    ///                                  system, but may have been upgraded to another version of
    ///                                  Windows.
    ///  * [`VER_SUITE_SMALLBUSINESS_RESTRICTED`] - Microsoft Small Business Server is installed
    ///                                             with the restrictive client license in force.
    ///  * [`VER_SUITE_STORAGE_SERVER`] - Windows Storage Server 2003 R2 or Windows Storage Server
    ///                                   2003 is installed.
    ///  * [`VER_SUITE_TERMINAL`] - Terminal Services is installed. This value is always set. If
    ///                             [`VER_SUITE_TERMINAL`] is set but [`VER_SUITE_SINGLEUSERTS`] is
    ///                             not set, the system is running in application server mode.
    ///  * [`VER_SUITE_WH_SERVER`] - Windows Home Server is installed.
    ///  * [`VER_SUITE_MULTIUSERTS`] - AppServer mode is enabled.
    pub suite_mask: WORD,

    /// Any additional information about the system. This member can be one of the following
    /// values:
    ///  * [`VER_NT_DOMAIN_CONTROLLER`] - The system is a domain controller and the operating
    ///                                   system is Windows Server 2012 , Windows Server 2008 R2,
    ///                                   Windows Server 2008, Windows Server 2003, or Windows 2000
    ///                                   Server.
    ///  * [`VER_NT_SERVER`] - The operating system is Windows Server 2012, Windows Server 2008 R2,
    ///                        Windows Server 2008, Windows Server 2003, or Windows 2000 Server.
    ///                        Note that a server that is also a domain controller is reported as
    ///                        [`VER_NT_DOMAIN_CONTROLLER`], not [`VER_NT_SERVER`].
    ///  * [`VER_NT_WORKSTATION`] - The operating system is Windows 8, Windows 7, Windows Vista,
    ///                             Windows XP Professional, Windows XP Home Edition, or Windows
    ///                             2000 Professional.
    pub product_type: BYTE,

    /// Reserved for future use.
    pub reserved: BYTE,
}

impl Default for OSVERSIONINFOEXA {
    fn default() -> Self {
        OSVERSIONINFOEXA {
            os_version_info_size: 0,
            major_version: 0,
            minor_version: 0,
            build_number: 0,
            platform_id: 0,
            csd_version: [0; 128],
            service_pack_major: 0,
            service_pack_minor: 0,
            suite_mask: 0,
            product_type: 0,
            reserved: 0,
        }
    }
}
