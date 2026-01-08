use crate::USHORT;

/// Microsoft Small Business Server was once installed on the system, but may have been upgraded to
/// another version of Windows.
pub const VER_SUITE_SMALLBUSINESS: USHORT = 0x00000001;

/// Windows Server 2008 Enterprise, Windows Server 2003, Enterprise Edition, or Windows 2000
/// Advanced Server is installed.
pub const VER_SUITE_ENTERPRISE: USHORT = 0x00000002;

/// Microsoft BackOffice components are installed.
pub const VER_SUITE_BACKOFFICE: USHORT = 0x00000004;

/// Terminal Services is installed. This value is always set. If [`VER_SUITE_TERMINAL`] is set but
/// [`VER_SUITE_SINGLEUSERTS`] is not set, the operating system is running in application server
/// mode.
pub const VER_SUITE_TERMINAL: USHORT = 0x00000010;

/// Microsoft Small Business Server is installed with the restrictive client license in force.
pub const VER_SUITE_SMALLBUSINESS_RESTRICTED: USHORT = 0x00000020;

/// Windows XP Embedded is installed.
pub const VER_SUITE_EMBEDDEDNT: USHORT = 0x00000040;

/// Windows Server 2008 Datacenter, Windows Server 2003, Datacenter Edition, or Windows 2000
/// Datacenter Server is installed.
pub const VER_SUITE_DATACENTER: USHORT = 0x00000080;

/// Remote Desktop is supported, but only one interactive session is supported. This value is set
/// unless the system is running in application server mode.
pub const VER_SUITE_SINGLEUSERTS: USHORT = 0x00000100;

/// Windows Vista Home Premium, Windows Vista Home Basic, or Windows XP Home Edition is installed.
pub const VER_SUITE_PERSONAL: USHORT = 0x00000200;

/// Windows Server 2003, Web Edition is installed.
pub const VER_SUITE_BLADE: USHORT = 0x00000400;

/// Windows Storage Server 2003 R2 or Windows Storage Server 2003 is installed.
pub const VER_SUITE_STORAGE_SERVER: USHORT = 0x00002000;

/// Windows Server 2003, Compute Cluster Edition is installed.
pub const VER_SUITE_COMPUTE_SERVER: USHORT = 0x00004000;

/// Windows Home Server is installed.
pub const VER_SUITE_WH_SERVER: USHORT = 0x00008000;
