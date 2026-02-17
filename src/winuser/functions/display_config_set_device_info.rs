use crate::{DISPLAYCONFIG_DEVICE_INFO_HEADER, LONG};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    DISPLAYCONFIG_SET_TARGET_PERSISTENCE, ERROR_ACCESS_DENIED, ERROR_GEN_FAILURE,
    ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_PARAMETER, ERROR_NOT_SUPPORTED, ERROR_SUCCESS,
};

#[link(name = "User32")]
unsafe extern "system" {
    /// The [`DisplayConfigSetDeviceInfo`] function sets the properties of a target.
    ///
    /// # Parameters
    ///  * `set_packet` - A pointer to a [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure that
    ///                   contains information to set for the device. The type and size of
    ///                   additional data that [`DisplayConfigSetDeviceInfo`] uses for the
    ///                   configuration comes after the header structure. This additional data
    ///                   depends on the packet type, as specified by the type member of
    ///                   [`DISPLAYCONFIG_DEVICE_INFO_HEADER`]. For example, if the caller wants to
    ///                   change the boot persistence, that caller allocates and fills a
    ///                   [`DISPLAYCONFIG_SET_TARGET_PERSISTENCE`] structure and passes a pointer
    ///                   to this structure in `set_packet`. Note that the first member of the
    ///                   [`DISPLAYCONFIG_SET_TARGET_PERSISTENCE`] structure is the
    ///                   [`DISPLAYCONFIG_DEVICE_INFO_HEADER`].
    ///
    /// # Return Value
    /// The function returns one of the following return codes:
    ///  * [`ERROR_SUCCESS`] - The function succeeded.
    ///  * [`ERROR_INVALID_PARAMETER`] - The combination of parameters and flags specified are
    ///                                  invalid.
    ///  * [`ERROR_NOT_SUPPORTED`] - The system is not running a graphics driver that was written
    ///                              according to the Windows Display Driver Model (WDDM). The
    ///                              function is only supported on a system with a WDDM driver
    ///                              running.
    ///  * [`ERROR_ACCESS_DENIED`] - The caller does not have access to the console session. This
    ///                              error occurs if the calling process does not have access to
    ///                              the current desktop or is running on a remote session.
    ///  * [`ERROR_INSUFFICIENT_BUFFER`] - The size of the packet that the caller passes is not big
    ///                                    enough.
    ///  * [`ERROR_GEN_FAILURE`] - An unspecified error occurred.
    ///
    /// Remarks
    /// [`DisplayConfigSetDeviceInfo`] can currently only be used to start and stop boot persisted
    /// force projection on an analog target.
    ///
    /// [`DisplayConfigSetDeviceInfo`] can only be used to set
    /// `DISPLAYCONFIG_DEVICE_INFO_TYPE::SetXXX` type of information.
    /// [`DisplayConfigSetDeviceInfo`] fails if the type member of
    /// [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] is set to one of the
    /// `DISPLAYCONFIG_DEVICE_INFO_TYPE::GetXXX` values.
    pub fn DisplayConfigSetDeviceInfo(set_packet: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER) -> LONG;
}
