use crate::{DWORD, HWND, USHORT};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    RegisterRawInputDevices, RIDEV_APPKEYS, RIDEV_CAPTUREMOUSE, RIDEV_DEVNOTIFY, RIDEV_EXCLUDE,
    RIDEV_EXINPUTSINK, RIDEV_INPUTSINK, RIDEV_NOHOTKEYS, RIDEV_NOLEGACY, RIDEV_PAGEONLY,
    RIDEV_REMOVE, WM_INPUT_DEVICE_CHANGE, WM_KEYDOWN, WM_LBUTTONDOWN,
};

/// Defines information for the raw input devices.
///
/// # Remarks
/// If [`RIDEV_NOLEGACY`] is set for a mouse or a keyboard, the system does not generate any legacy
/// message for that device for the application. For example, if the mouse TLC is set with
/// [`RIDEV_NOLEGACY`], [`WM_LBUTTONDOWN`] and related legacy mouse messages are not generated.
/// Likewise, if the keyboard TLC is set with [`RIDEV_NOLEGACY`], [`WM_KEYDOWN`] and related legacy
/// keyboard messages are not generated.
///
/// If [`RIDEV_REMOVE`] is set and the `wnd_target` member is not set to [`null_mut`], then
/// [`RegisterRawInputDevices`] function will fail.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RAWINPUTDEVICE {
    /// Top level collection Usage page for the raw input device.
    pub usage_page: USHORT,

    /// Top level collection Usage ID for the raw input device.
    pub usage: USHORT,

    /// Mode flag that specifies how to interpret the information provided by `usage_pages` and
    /// `usage`. It can be zero (the default) or one of the following values. By default, the
    /// operating system sends raw input from devices with the specified top level collection (TLC)
    /// to the registered application as long as it has the window focus.
    /// * [`RIDEV_REMOVE`] - If set, this removes the top level collection from the inclusion list.
    ///                      This tells the operating system to stop reading from a device which
    ///                      matches the top level collection.
    /// * [`RIDEV_EXCLUDE`] - If set, this specifies the top level collections to exclude when
    ///                       reading a complete usage page. This flag only affects a TLC whose
    ///                       usage page is already specified with [`RIDEV_PAGEONLY`].
    /// * [`RIDEV_PAGEONLY`] - If set, this specifies all devices whose top level collection is
    ///                        from the specified `usage_page`. Note that `usage` must be zero. To
    ///                        exclude a particular top level collection, use [`RIDEV_EXCLUDE`].
    /// * [`RIDEV_NOLEGACY`] - If set, this prevents any devices specified by `usage_page` or
    ///                        `usage` from generating legacy messages. This is only for the mouse
    ///                        and keyboard.
    /// * [`RIDEV_INPUTSINK`] - If set, this enables the caller to receive the input even when the
    ///                         caller is not in the foreground. Note that `wnd_target` must be
    ///                         specified.
    /// * [`RIDEV_CAPTUREMOUSE`] - If set, the mouse button click does not activate the other
    ///                            window. [`RIDEV_CAPTUREMOUSE`] can be specified only if
    ///                            [`RIDEV_NOLEGACY`] is specified for a mouse device.
    /// * [`RIDEV_NOHOTKEYS`] - If set, the application-defined keyboard device hotkeys are not
    ///                         handled. However, the system hotkeys; for example, `ALT+TAB` and
    ///                         `CTRL+ALT+DEL`, are still handled. By default, all keyboard hotkeys
    ///                         are handled. [`RIDEV_NOHOTKEYS`] can be specified even if
    ///                         [`RIDEV_NOLEGACY`] is not specified and `wnd_target` is
    ///                         [`null_mut`].
    /// * [`RIDEV_APPKEYS`] - If set, the application command keys are handled. [`RIDEV_APPKEYS`]
    ///                       can be specified only if [`RIDEV_NOLEGACY`] is specified for a
    ///                       keyboard device.
    /// * [`RIDEV_EXINPUTSINK`] - If set, this enables the caller to receive input in the
    ///                           background only if the foreground application does not process
    ///                           it. In other words, if the foreground application is not
    ///                           registered for raw input, then the background application that is
    ///                           registered will receive the input.
    /// * [`RIDEV_DEVNOTIFY`] - If set, this enables the caller to receive
    ///                         [`WM_INPUT_DEVICE_CHANGE`] notifications for device arrival and
    ///                         device removal.
    pub flags: DWORD,

    /// A handle to the target window. If [`null_mut`], raw input events follow the keyboard focus
    /// to ensure only the focused application window receives the events.
    pub wnd_target: HWND,
}

impl Default for RAWINPUTDEVICE {
    fn default() -> Self {
        RAWINPUTDEVICE {
            usage_page: 0,
            usage: 0,
            flags: 0,
            wnd_target: null_mut(),
        }
    }
}
