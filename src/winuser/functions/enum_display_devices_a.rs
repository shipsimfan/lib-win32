use crate::{BOOL, DWORD, LPCSTR, PDISPLAY_DEVICEA};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    EnumDisplayDevices, DISPLAY_DEVICE, DISPLAY_DEVICE_ATTACHED_TO_DESKTOP,
    EDD_GET_DEVICE_INTERFACE_NAME,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "User32")]
unsafe extern "system" {
    /// The [`EnumDisplayDevices`] function lets you obtain information about the display devices
    /// in the current session.
    ///
    /// # Parameters
    ///  * `device` - A pointer to the device name. If [`null`], function returns information for
    ///               the display adapter(s) on the machine, based on `dev_num`.
    ///  * `dev_num` - An index value that specifies the display device of interest. The operating
    ///                system identifies each display device in the current session with an index
    ///                value. The index values are consecutive integers, starting at 0. If the
    ///                current session has three display devices, for example, they are specified
    ///                by the index values 0, 1, and 2.
    ///  * `display_device` - A pointer to a [`DISPLAY_DEVICE`] structure that receives information
    ///                       about the display device specified by `dev_num`. Before calling
    ///                       [`EnumDisplayDevices`], you must initialize the `cb` member of
    ///                       [`DISPLAY_DEVICE`] to the size, in bytes, of [`DISPLAY_DEVICE`].
    ///  * `flags` - Set this flag to [`EDD_GET_DEVICE_INTERFACE_NAME`] to retrieve the device
    ///              interface name for [`GUID_DEVINTERFACE_MONITOR`], which is registered by the
    ///              operating system on a per monitor basis. The value is placed in the
    ///              `device_id` member of the [`DISPLAY_DEVICE`] structure returned in
    ///              `display_device`. The resulting device interface name can be used with
    ///              SetupAPI functions and serves as a link between GDI monitor devices and
    ///              SetupAPI monitor devices.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. The function fails if `dev_num` is greater
    /// than the largest device index.
    ///
    /// # Remarks
    /// To query all display devices in the current session, call this function in a loop, starting
    /// with `dev_num` set to 0, and incrementing `dev_num` until the function fails. To select all
    /// display devices in the desktop, use only the display devices that have the
    /// [`DISPLAY_DEVICE_ATTACHED_TO_DESKTOP`] flag in the [`DISPLAY_DEVICE`] structure.
    ///
    /// To get information on the display adapter, call [`EnumDisplayDevices`] with `device` set to
    /// [`null`]. For example, [`DISPLAY_DEVICE::device_string`] contains the adapter name.
    ///
    /// To obtain information on a display monitor, first call [`EnumDisplayDevices`] with `device`
    /// set to [`null`]. Then call [`EnumDisplayDevices`] with `device` set to
    /// [`DISPLAY_DEVICE::device_name`] from the first call to [`EnumDisplayDevices`] and with
    /// `dev_num` set to zero. Then [`DISPLAY_DEVICE::device_string`] is the monitor name.
    ///
    /// To query all monitor devices associated with an adapter, call [`EnumDisplayDevices`] in a
    /// loop with `device` set to the adapter name, `dev_num` set to start at 0, and `dev_num` set
    /// to increment until the function fails. Note that [`DISPLAY_DEVICE::device_name`] changes
    /// with each call for monitor information, so you must save the adapter name. The function
    /// fails when there are no more monitors for the adapter.
    pub fn EnumDisplayDevicesA(
        device: LPCSTR,
        dev_num: DWORD,
        display_device: PDISPLAY_DEVICEA,
        flags: DWORD,
    ) -> BOOL;
}
