use crate::{BOOL, DEVMODEW, DWORD, LPCWSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{EnumDisplaySettingsEx, DEVMODE, ENUM_CURRENT_SETTINGS, ENUM_REGISTRY_SETTINGS};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "User32")]
extern "system" {
    /// The [`EnumDisplaySettingsEx`] function retrieves information about one of the graphics
    /// modes for a display device. To retrieve information for all the graphics modes for a
    /// display device, make a series of calls to this function.
    ///
    /// This function differs from [`EnumDisplaySettings`] in that there is a `flags` parameter.
    ///
    /// # Parameters
    ///  * `device_name` - A pointer to a null-terminated string that specifies the display device
    ///                    about which graphics mode the function will obtain information. This
    ///                    parameter is either [`null`] or a `DISPLAY_DEVICE`. `device_name`
    ///                    returned from [`EnumDisplayDevices`]. A [`null`] value specifies the
    ///                    current display device on the computer that the calling thread is
    ///                    running on.
    ///  * `mode_num` - Indicates the type of information to be retrieved. Graphics mode indexes
    ///                 start at zero. To obtain information for all of a display device's graphics
    ///                 modes, make a series of calls to [`EnumDisplaySettingsEx`], as follows: Set
    ///                 `mode_num` to zero for the first call, and increment `mode_num` by one for
    ///                 each subsequent call. Continue calling the function until the return value
    ///                 is zero. When you call [`EnumDisplaySettingsEx`] with `mode_num` set to
    ///                 zero, the operating system initializes and caches information about the
    ///                 display device. When you call [`EnumDisplaySettingsEx`] with `mode_num` set
    ///                 to a nonzero value, the function returns the information that was cached
    ///                 the last time the function was called with `mode_num` set to zero. This
    ///                 value can be a graphics mode index or one of the following values:
    ///    * [`ENUM_CURRENT_SETTINGS`] - Retrieve the current settings for the display device.
    ///    * [`ENUM_REGISTRY_SETTINGS`] - Retrieve the settings for the display device that are
    ///                                   currently stored in the registry.
    ///  * `dev_mode` - A pointer to a [`DEVMODE`] structure into which the function stores
    ///                 information about the specified graphics mode. Before calling
    ///                 [`EnumDisplaySettingsEx`], set the `size` member to
    ///                 `std::mem::size_of::<DEVMODE>()s`, and set the `driver_extra` member to
    ///                 indicate the size, in bytes, of the additional space available to receive
    ///                 private driver data. The [`EnumDisplaySettingsEx`] function will populate
    ///                 the `fields` member of the `dev_mode` and one or more other members of the
    ///                 [`DEVMODE`] structure. To determine which members were set by the call to
    ///                 [`EnumDisplaySettingsEx`], inspect the `fields` bitmask. Some of the fields
    ///                 typically populated by this function include:
    ///    * `bits_per_pel`
    ///    * `pels_width`
    ///    * `pels_height`
    ///    * `display_flags`
    ///    * `display_frequency`
    ///    * `position`
    ///    * `display_orientation`
    ///  * `flags` - This parameter can be the following value:
    ///    * [`EDS_RAWMODE`] - If set, the function will return all graphics modes reported by the
    ///                        adapter driver, regardless of monitor capabilities. Otherwise, it
    ///                        will only return modes that are compatible with current monitors.
    ///    * [`EDS_ROTATEDMODE`] - If set, the function will return graphics modes in all
    ///                            orientations. Otherwise, it will only return modes that have the
    ///                            same orientation as the one currently set for the requested
    ///                            display.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero.
    ///
    /// # Remarks
    /// The function fails if `mode_num` is greater than the index of the display device's last
    /// graphics mode. As noted in the description of the `mode_num` parameter, you can use this
    /// behavior to enumerate all of a display device's graphics modes.
    pub fn EnumDisplaySettingsExW(
        device_name: LPCWSTR,
        mode_num: DWORD,
        dev_mode: *mut DEVMODEW,
        flags: DWORD,
    ) -> BOOL;
}
