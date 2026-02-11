use crate::{DISPLAYCONFIG_DEVICE_INFO_HEADER, LONG};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    ERROR_ACCESS_DENIED, ERROR_GEN_FAILURE, ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_PARAMETER,
    ERROR_NOT_SUPPORTED, ERROR_SUCCESS,
};

#[link(name = "User32")]
extern "system" {
    /// The [`DisplayConfigGetDeviceInfo`] function retrieves display configuration information
    /// about the device.
    ///
    /// # Parameters
    ///  * `request_packet` - A pointer to a [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure. This
    ///                       structure contains information about the request, which includes the
    ///                       packet type in the type member. The type and size of additional data
    ///                       that [`DisplayConfigGetDeviceInfo`] returns after the header
    ///                       structure depend on the packet type.
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
    ///                                    enough for the information that the caller requests.
    ///  * [`ERROR_GEN_FAILURE`] - An unspecified error occurred.
    ///
    /// # Remarks
    /// Use the [`DisplayConfigGetDeviceInfo`] function to obtain additional information about a
    /// source or target for an adapter, such as the display name, the preferred display mode, and
    /// source device name.
    ///
    /// The caller can call [`DisplayConfigGetDeviceInfo`] to obtain more friendly names to display
    /// in the user interface. The caller can obtain names for the adapter, the source, and the
    /// target. The caller can also call [`DisplayConfigGetDeviceInfo`] to obtain the best
    /// resolution of the connected display device.
    pub fn DisplayConfigGetDeviceInfo(
        request_packet: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> LONG;
}
