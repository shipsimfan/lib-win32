use crate::{UCHAR, ULONG, USHORT, WCHAR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    wdm::RTL_OSVERSIONINFOEXW, VER_NT_DOMAIN_CONTROLLER, VER_NT_SERVER, VER_NT_WORKSTATION,
    VER_PLATFORM_WIN32_NT, VER_SUITE_BACKOFFICE, VER_SUITE_BLADE, VER_SUITE_COMPUTE_SERVER,
    VER_SUITE_DATACENTER, VER_SUITE_EMBEDDEDNT, VER_SUITE_ENTERPRISE, VER_SUITE_PERSONAL,
    VER_SUITE_SINGLEUSERTS, VER_SUITE_SMALLBUSINESS, VER_SUITE_SMALLBUSINESS_RESTRICTED,
    VER_SUITE_STORAGE_SERVER, VER_SUITE_TERMINAL, VER_SUITE_WH_SERVER,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::ntddk::RtlGetVersion;

/// The [`RTL_OSVERSIONINFOEXW`] structure contains operating system version information.
///
/// # Remarks
/// The information in this structure includes the major and minor version numbers, the build
/// number, the platform identifier, the installed product suites, and the latest service pack that
/// is installed on the system. This structure is used with the [`RtlGetVersion`] and
/// [`RtlVerifyVersionInfo`] routines.
///
/// Relying on version information is not always the best way to test whether a feature is
/// available. For guidance, refer to the documentation for the feature you are interested in.
///
/// If possible, design the version detection code in your driver to enable the driver to run on
/// future versions of Windows. If your driver requires a particular operating system version, be
/// sure to treat this version as the minimum supported version, and not as the only version on
/// which the driver can run.
///
/// Only a 64-bit kernel-mode driver can run on Windows XP Professional x64 Edition. Therefore, a
/// 32-bit kernel-mode driver can safely omit checking for this version of Windows.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct OSVERSIONINFOEXW {
    /// The size, in bytes, of an [`RTL_OSVERSIONINFOEXW`] structure. This member must be set
    /// before the structure is used with [`RtlGetVersion`].
    pub os_version_info_size: ULONG,

    /// The major version number of the operating system. For example, for Windows 2000, the major
    /// version number is five.
    pub major_version: ULONG,

    /// The minor version number of the operating system. For example, for Windows 2000, the minor
    /// version number is zero.
    pub minor_version: ULONG,

    /// The build number of the operating system.
    pub build_number: ULONG,

    /// The operating system platform. For Win32 on NT-based operating systems, [`RtlGetVersion`]
    /// returns the value [`VER_PLATFORM_WIN32_NT`].
    pub platform_id: ULONG,

    /// The service-pack version string. This member contains a null-terminated string, such as
    /// "Service Pack 3", which indicates the latest service pack installed on the system. If no
    /// service pack is installed, [`RtlGetVersion`] might not initialize this string. Initialize
    /// `csd_version` to zero (empty string) before the call to [`RtlGetVersion`].
    pub csd_version: [WCHAR; 128],

    /// The major version number of the latest service pack installed on the system. For example,
    /// for Service Pack 3, the major version number is three. If no service pack has been
    /// installed, the value is zero.
    pub service_pack_major: USHORT,

    /// The minor version number of the latest service pack installed on the system. For example,
    /// for Service Pack 3, the minor version number is zero.
    pub service_pack_minor: USHORT,

    /// The product suites available on the system. This member is set to zero or to the bitwise OR
    /// of one or more of the following values:
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
    ///                             not set, the operating system is running in application server
    ///                             mode.
    ///  * [`VER_SUITE_WH_SERVER`] - Windows Home Server is installed.
    ///
    /// You should not rely solely on the [`VER_SUITE_SMALLBUSINESS`] flag to determine whether
    /// Small Business Server is currently installed. Both this flag and the
    /// [`VER_SUITE_SMALLBUSINESS_RESTRICTED`] flag are set when this product suite is installed.
    /// If you upgrade this installation to Windows Server, Standard Edition, the
    /// [`VER_SUITE_SMALLBUSINESS_RESTRICTED`] flag is cleared, but the VER_SUITE_SMALLBUSINESS
    /// flag remains set, which, in this case, indicates that Small Business Server was previously
    /// installed on this system. If this installation is further upgraded to Windows Server,
    /// Enterprise Edition, the [`VER_SUITE_SMALLBUSINESS`] flag remains set.
    pub suite_mask: USHORT,

    /// The product type. This member contains additional information about the system. This member
    /// can be one of the following values:
    ///  * [`VER_NT_WORKSTATION`] - Windows 2000 or later professional version
    ///  * [`VER_NT_DOMAIN_CONTROLLER`] - Windows 2000 or later domain controller
    ///  * [`VER_NT_SERVER`] - Windows 2000 or later server
    pub product_type: UCHAR,

    /// Reserved for future use.
    pub reserved: UCHAR,
}

impl Default for OSVERSIONINFOEXW {
    fn default() -> Self {
        OSVERSIONINFOEXW {
            os_version_info_size: std::mem::size_of::<OSVERSIONINFOEXW>() as _,
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
