use crate::{wdm::PRTL_OSVERSIONINFOW, NTSTATUS};

#[link(name = "Ntdll")]
unsafe extern "system" {
    /// Gets version information about the currently running operating system.
    ///
    /// # Parameters
    ///  * `version_information` - Pointer to either a [`RTL_OSVERSIONINFOW`] structure or a
    ///                            [`RTL_OSVERSIONINFOEXW`] structure that contains the version
    ///                            information about the currently running operating system. A
    ///                            caller specifies which input structure is used by setting the
    ///                            `os_version_info_size` member of the structure to the size in
    ///                            bytes of the structure that is used.
    ///
    /// # Return Value
    /// Returns [`STATUS_SUCCESS`].
    ///
    /// # Remarks
    /// [`RtlGetVersion`] is the equivalent of the [`GetVersionEx`] function in the Windows SDK.
    /// See the example in the Windows SDK that shows how to get the system version.
    ///
    /// When using [`RtlGetVersion`] to determine whether a particular version of the operating
    /// system is running, a caller should check for version numbers that are greater than or equal
    /// to the required version number. This ensures that a version test succeeds for later
    /// versions of Windows.
    ///
    /// Because operating system features can be added in a redistributable DLL, checking only the
    /// major and minor version numbers is not the most reliable way to verify the presence of a
    /// specific system feature. A driver should use [`RtlVerifyVersionInfo`] to test for the
    /// presence of a specific system feature.
    pub fn RtlGetVersion(version_information: PRTL_OSVERSIONINFOW) -> NTSTATUS;
}
