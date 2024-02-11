use crate::HKEY;

// rustdoc imports
#[allow(unused_imports)]
use crate::RegOpenCurrentUser;

/// Registry entries subordinate to this key define types (or classes) of documents and the
/// properties associated with those types. Shell and COM applications use the information stored
/// under this key.
///
/// This key also provides backward compatibility with the Windows 3.1 registration database by
/// storing information for DDE and OLE support. File viewers and user interface extensions store
/// their OLE class identifiers in [`HKEY_CLASSES_ROOT`], and in-process servers are registered in
/// this key.
///
/// This handle should not be used in a service or an application that impersonates different
/// users.
pub const HKEY_CLASSES_ROOT: HKEY = 0x80000000 as HKEY;

/// Registry entries subordinate to this key define the preferences of the current user. These
/// preferences include the settings of environment variables, data about program groups, colors,
/// printers, network connections, and application preferences. This key makes it easier to
/// establish the current user's settings; the key maps to the current user's branch in
/// [`HKEY_USERS`]. In [`HKEY_CURRENT_USER`], software vendors store the current user-specific
/// preferences to be used within their applications. Microsoft, for example, creates the
/// `HKEY_CURRENT_USER\Software\Microsoft` key for its applications to use, with each application
/// creating its own subkey under the Microsoft key.
///
/// The mapping between [`HKEY_CURRENT_USER`] and [`HKEY_USERS`] is per process and is established
/// the first time the process references [`HKEY_CURRENT_USER`]. The mapping is based on the
/// security context of the first thread to reference [`HKEY_CURRENT_USER`]. If this security
/// context does not have a registry hive loaded in [`HKEY_USERS`], the mapping is established with
/// `HKEY_USERS\.Default`. After this mapping is established it persists, even if the security
/// context of the thread changes.
///
/// All registry entries in [`HKEY_CURRENT_USER`] except those under
/// `HKEY_CURRENT_USER\Software\Classes` are included in the per-user registry portion of a roaming
/// user profile. To exclude other entries from a roaming user profile, store them in
/// [`HKEY_CURRENT_USER_LOCAL_SETTINGS`].
///
/// This handle should not be used in a service or an application that impersonates different
/// users. Instead, call the [`RegOpenCurrentUser`] function.
pub const HKEY_CURRENT_USER: HKEY = 0x80000001 as HKEY;

/// Registry entries subordinate to this key define the physical state of the computer, including
/// data about the bus type, system memory, and installed hardware and software. It contains
/// subkeys that hold current configuration data, including Plug and Play information (the `Enum`
/// branch, which includes a complete list of all hardware that has ever been on the system),
/// network logon preferences, network security information, software-related information (such as
/// server names and the location of the server), and other system information.
pub const HKEY_LOCAL_MACHINE: HKEY = 0x80000002 as HKEY;

/// Registry entries subordinate to this key define the default user configuration for new users on
/// the local computer and the user configuration for the current user.
pub const HKEY_USERS: HKEY = 0x80000003 as HKEY;

/// Registry entries subordinate to this key allow you to access performance data. The data is not
/// actually stored in the registry; the registry functions cause the system to collect the data
/// from its source.
pub const HKEY_PERFORMANCE_DATA: HKEY = 0x80000004 as HKEY;

/// Registry entries subordinate to this key reference the text strings that describe counters in
/// US English. These entries are not available to Regedit.exe and Regedt32.exe. Windows 2000: This
/// key is not supported.
pub const HKEY_PERFORMANCE_TEXT: HKEY = 0x80000050 as HKEY;

/// Registry entries subordinate to this key reference the text strings that describe counters in
/// the local language of the area in which the computer system is running. These entries are not
/// available to Regedit.exe and Regedt32.exe. Windows 2000: This key is not supported.
pub const HKEY_PERFORMANCE_NLSTEXT: HKEY = 0x80000060 as HKEY;

/// Contains information about the current hardware profile of the local computer system. The
/// information under [`HKEY_CURRENT_CONFIG`] describes only the differences between the current
/// hardware configuration and the standard configuration. Information about the standard hardware
/// configuration is stored under the Software and System keys of [`HKEY_LOCAL_MACHINE`].
///
/// [`HKEY_CURRENT_CONFIG`] is an alias for
/// `HKEY_LOCAL_MACHINE\System\CurrentControlSet\Hardware Profiles\Current`.
pub const HKEY_CURRENT_CONFIG: HKEY = 0x80000005 as HKEY;

/// Registry entries subordinate to this key define preferences of the current user that are local
/// to the machine. These entries are not included in the per-user registry portion of a roaming
/// user profile. Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This
/// key is supported starting with Windows 7 and Windows Server 2008 R2.
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: HKEY = 0x80000007 as HKEY;
