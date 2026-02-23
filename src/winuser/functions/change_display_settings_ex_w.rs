use crate::{DEVMODEW, DWORD, HWND, LONG, LPCWSTR, LPVOID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CDS_DISABLE_UNSAFE_MODES, CDS_ENABLE_UNSAFE_MODES, CDS_FULLSCREEN, CDS_GLOBAL, CDS_NORESET,
    CDS_RESET, CDS_SET_PRIMARY, CDS_TEST, CDS_UPDATEREGISTRY, CDS_VIDEOPARAMETERS,
    ChangeDisplaySettingsEx, DEVMODE, DISP_CHANGE_BADDUALVIEW, DISP_CHANGE_BADFLAGS,
    DISP_CHANGE_BADMODE, DISP_CHANGE_BADPARAM, DISP_CHANGE_FAILED, DISP_CHANGE_NOTUPDATED,
    DISP_CHANGE_RESTART, DISP_CHANGE_SUCCESSFUL, DISPLAY_DEVICE_PRIMARY_DEVICE, DM_POSITION,
    EnumDisplayDevices, WM_DISPLAYCHANGE,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "User32")]
unsafe extern "system" {
    /// The [`ChangeDisplaySettingsEx`] function changes the settings of the specified display
    /// device to the specified graphics mode.
    ///
    /// # Parameters
    ///  * `device_name` - A pointer to a null-terminated string that specifies the display device
    ///                    whose graphics mode will change. Only display device names as returned
    ///                    by [`EnumDisplayDevices`] are valid. See [`EnumDisplayDevices`] for
    ///                    further information on the names associated with these display devices.
    ///                    The `device_name` parameter can be [`null`]. A [`null`] value specifies
    ///                    the default display device. The default device can be determined by
    ///                    calling [`EnumDisplayDevices`] and checking for the
    ///                    [`DISPLAY_DEVICE_PRIMARY_DEVICE`] flag.
    ///  * `dev_mode` - A pointer to a [`DEVMODE`] structure that describes the new graphics mode.
    ///                 If `dev_mode` is [`null_mut`], all the values currently in the registry
    ///                 will be used for the display setting. Passing [`null_mut`] for the
    ///                 `dev_mode` parameter and 0 for the `flags` parameter is the easiest way to
    ///                 return to the default mode after a dynamic mode change. The `size` member
    ///                 must be initialized to the size, in bytes, of the [`DEVMODE`] structure.
    ///                 The `driver_extra` member must be initialized to indicate the number of
    ///                 bytes of private driver data following the [`DEVMODE`] structure. In
    ///                 addition, you can use any of the following members of the [`DEVMODE`]
    ///                 structure:
    ///    * `bits_per_pel` - Bits per pixel
    ///    * `pels_width` - Pixel width
    ///    * `pels_height` - Pixel height
    ///    * `display_flags` - Mode flags
    ///    * `display_frequency` - Mode frequency
    ///    * `position` - Position of the device in a multi-monitor configuration.
    ///  * `wnd` - Reserved; must be [`null_mut`].
    ///  * `flags` - Indicates how the graphics mode should be changed. This parameter can be one
    ///              of the following values:
    ///    * 0 - The graphics mode for the current screen will be changed dynamically.
    ///    * [`CDS_FULLSCREEN`] - The mode is temporary in nature. If you change to and from
    ///                           another desktop, this mode will not be reset.
    ///    * [`CDS_GLOBAL`] - The settings will be saved in the global settings area so that they
    ///                       will affect all users on the machine. Otherwise, only the settings
    ///                       for the user are modified. This flag is only valid when specified
    ///                       with the [`CDS_UPDATEREGISTRY`] flag.
    ///    * [`CDS_NORESET`] - The settings will be saved in the registry, but will not take
    ///                        effect. This flag is only valid when specified with the
    ///                        [`CDS_UPDATEREGISTRY`] flag.
    ///    * [`CDS_RESET`] - The settings should be changed, even if the requested settings are the
    ///                      same as the current settings.
    ///    * [`CDS_SET_PRIMARY`] - This device will become the primary device.
    ///    * [`CDS_TEST`] - The system tests if the requested graphics mode could be set.
    ///    * [`CDS_UPDATEREGISTRY`] - The graphics mode for the current screen will be changed
    ///                               dynamically and the graphics mode will be updated in the
    ///                               registry. The mode information is stored in the `USER`
    ///                               profile.
    ///    * [`CDS_VIDEOPARAMETERS`] - When set, the `l_param` parameter is a pointer to a
    ///                                [`VIDEOPARAMETERS`] structure.
    ///    * [`CDS_ENABLE_UNSAFE_MODES`] - Enables settings changes to unsafe graphics modes.
    ///    * [`CDS_DISABLE_UNSAFE_MODES`] - Disables settings changes to unsafe graphics modes.
    ///  * `param` - If `flags` is [`CDS_VIDEOPARAMETERS`], `l_param` is a pointer to a
    ///              [`VIDEOPARAMETERS`] structure. Otherwise `l_param` must be [`null_mut`].
    ///
    /// # Return Value
    /// The [`ChangeDisplaySettingsEx`] function returns one of the following values:
    ///  * [`DISP_CHANGE_SUCCESSFUL`] - The settings change was successful.
    ///  * [`DISP_CHANGE_BADDUALVIEW`] - The settings change was unsuccessful because the system is
    ///                                  DualView capable.
    ///  * [`DISP_CHANGE_BADFLAGS`] - An invalid set of flags was passed in.
    ///  * [`DISP_CHANGE_BADMODE`] - The graphics mode is not supported.
    ///  * [`DISP_CHANGE_BADPARAM`] - An invalid parameter was passed in. This can include an
    ///                               invalid flag or combination of flags.
    ///  * [`DISP_CHANGE_FAILED`] - The display driver failed the specified graphics mode.
    ///  * [`DISP_CHANGE_NOTUPDATED`] - Unable to write settings to the registry.
    ///  * [`DISP_CHANGE_RESTART`] - The computer must be restarted for the graphics mode to work.
    ///
    /// # Remarks
    /// To ensure that the [`DEVMODE`] structure passed to [`ChangeDisplaySettingsEx`] is valid and
    /// contains only values supported by the display driver, use the [`DEVMODE`] returned by the
    /// [`EnumDisplaySettings`] function.
    ///
    /// When adding a display monitor to a multiple-monitor system programmatically, set
    /// [`DEVMODE::fields`] to [`DM_POSITION`] and specify a position (in `DEVMODE::position`) for
    /// the monitor you are adding that is adjacent to at least one pixel of the display area of an
    /// existing monitor. To detach the monitor, set [`DEVMODE::fields`] to [`DM_POSITION`] but set
    /// [`DEVMODE::pels_width`] and [`DEVMODE::pels_height`] to zero.
    ///
    /// When the display mode is changed dynamically, the [`WM_DISPLAYCHANGE`] message is sent to
    /// all running applications with the following message parameters.
    ///  * `w_param` - New bits per pixel
    ///  * `LOWORD!(l_param)` - New pixel width
    ///  * `HIWORD!(l_param)` - New pixel height
    ///
    /// To change the settings for more than one display at the same time, first call
    /// [`ChangeDisplaySettingsEx`] for each device individually to update the registry without
    /// applying the changes. Then call [`ChangeDisplaySettingsEx`] once more, with a [`null_mut`]
    /// device, to apply the changes. For example, to change the settings for two displays, do the
    /// following:
    /// ```no_run
    /// ChangeDisplaySettingsEx(device_name1, dev_mode1, null_mut(), (CDS_UPDATEREGISTRY | CDS_NORESET), null_mut());
    /// ChangeDisplaySettingsEx(device_name2, dev_mode2, null_mut(), (CDS_UPDATEREGISTRY | CDS_NORESET), null_mut());
    /// ChangeDisplaySettingsEx(null(), null_mut(), null_mut(), 0, null_mut());
    /// ```
    pub fn ChangeDisplaySettingsExW(
        device_name: LPCWSTR,
        dev_mode: *mut DEVMODEW,
        wnd: HWND,
        flags: DWORD,
        l_param: LPVOID,
    ) -> LONG;
}
