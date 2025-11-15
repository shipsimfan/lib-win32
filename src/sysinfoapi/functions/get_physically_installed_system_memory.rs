use crate::{BOOL, PULONGLONG};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, ERROR_INVALID_DATA, ERROR_INVALID_PARAMETER, FALSE, TRUE};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "C" {
    /// Retrieves the amount of RAM that is physically installed on the computer.
    ///
    /// # Parameters
    ///  * `total_memory_in_kilobytes` - A pointer to a variable that receives the amount of
    ///                                  physically installed RAM, in kilobytes.
    ///
    /// # Return Value
    /// If the function succeeds, it returns [`TRUE`] and sets the `total_memory_in_kilobytes`
    /// parameter to a nonzero value.
    ///
    /// If the function fails, it returns FALSE and does not modify the `total_memory_in_kilobytes`
    /// parameter. To get extended error information, use the [`GetLastError`] function. Common
    /// errors are the following:
    ///  * [`ERROR_INVALID_PARAMETER`] -  The `total_memory_in_kilobytes` parameter is
    ///                                   [`null_mut`].
    ///  * [`ERROR_INVALID_DATA`] -  The System Management BIOS (SMBIOS) data is malformed.
    ///
    /// # Remarks
    /// The [`GetPhysicallyInstalledSystemMemory`] function retrieves the amount of physically
    /// installed RAM from the computer's SMBIOS firmware tables. This can differ from the amount
    /// reported by the [`GlobalMemoryStatusEx`] function, which sets the `total_phys` member of
    /// the [`MEMORYSTATUSEX`] structure to the amount of physical memory that is available for the
    /// operating system to use. The amount of memory available to the operating system can be less
    /// than the amount of memory physically installed in the computer because the BIOS and some
    /// drivers may reserve memory as I/O regions for memory-mapped devices, making the memory
    /// unavailable to the operating system and applications.
    ///
    /// The amount of physical memory retrieved by the [`GetPhysicallyInstalledSystemMemory`]
    /// function must be equal to or greater than the amount reported by the
    /// [`GlobalMemoryStatusEx`] function; if it is less, the SMBIOS data is malformed and the
    /// function fails with [`ERROR_INVALID_DATA`]. Malformed SMBIOS data may indicate a problem
    /// with the user's computer.
    pub fn GetPhysicallyInstalledSystemMemory(total_memory_in_kilobytes: PULONGLONG) -> BOOL;
}
