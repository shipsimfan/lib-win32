use crate::{BOOL, LPOSVERSIONINFOA};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, GetVersionEx, OSVERSIONINFOA, OSVERSIONINFOEXA};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// With the release of Windows 8.1, the behavior of the [`GetVersionEx`] API has changed in
    /// the value it will return for the operating system version. The value returned by the
    /// [`GetVersionEx`] function now depends on how the application is manifested.
    ///
    /// Applications not manifested for Windows 8.1 or Windows 10 will return the Windows 8 OS
    /// version value (6.2). Once an application is manifested for a given operating system
    /// version, [`GetVersionEx`] will always return the version that the application is manifested
    /// for in future releases.
    ///
    /// # Parameters
    ///  * `version_information` - An [`OSVERSIONINFOA`] or [`OSVERSIONINFOEXA`] structure that
    ///                            receives the operating system information. Before calling the
    ///                            [`GetVersionEx`] function, set the `os_version_info_size` member
    ///                            of the structure as appropriate to indicate which data structure
    ///                            is being passed to this function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a nonzero value.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`]. The function fails if you specify an invalid value for the
    /// `os_version_info_size` member of the [`OSVERSIONINFOA`] or [`OSVERSIONINFOEXA`] structure.
    ///
    /// # Remarks
    /// Identifying the current operating system is usually not the best way to determine whether a
    /// particular operating system feature is present. This is because the operating system may
    /// have had new features added in a redistributable DLL. Rather than using [`GetVersionEx`] to
    /// determine the operating system platform or version number, test for the presence of the
    /// feature itself.
    ///
    /// The [`GetSystemMetrics`] function provides additional information about the current
    /// operating system.
    ///
    /// To check for specific operating systems or operating system features, use the [`IsOS`]
    /// function. The [`GetProductInfo`] function retrieves the product type.
    ///
    /// To retrieve information for the operating system on a remote computer, use the
    /// [`NetWkstaGetInfo`] function, or the OperatingSystem property of the [`IADsComputer`]
    /// interface.
    ///
    /// To compare the current system version to a required version, use the [`VerifyVersionInfo`]
    /// function instead of using [`GetVersionEx`] to perform the comparison yourself.
    ///
    /// If compatibility mode is in effect, the [`GetVersionEx`] function reports the operating
    /// system as it identifies itself, which may not be the operating system that is installed.
    /// For example, if compatibility mode is in effect, [`GetVersionEx`] reports the operating
    /// system that is selected for application compatibility.
    pub fn GetVersionExA(version_information: LPOSVERSIONINFOA) -> BOOL;
}
