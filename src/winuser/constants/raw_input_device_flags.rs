use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::WM_INPUT_DEVICE_CHANGE;
#[allow(unused_imports)]
use std::ptr::null_mut;

/// If set, this removes the top level collection from the inclusion list. This tells the operating
/// system to stop reading from a device which matches the top level collection.
pub const RIDEV_REMOVE: DWORD = 0x00000001;

/// If set, this specifies the top level collections to exclude when reading a complete usage page.
/// This flag only affects a TLC whose usage page is already specified with [`RIDEV_PAGEONLY`].
pub const RIDEV_EXCLUDE: DWORD = 0x00000010;

/// If set, this specifies all devices whose top level collection is from the specified
/// `usage_page`. Note that usUsage must be zero. To exclude a particular top level collection, use
/// [`RIDEV_EXCLUDE`].
pub const RIDEV_PAGEONLY: DWORD = 0x00000020;

/// If set, this prevents any devices specified by `usage_page` or `usage` from generating legacy
/// messages. This is only for the mouse and keyboard.
pub const RIDEV_NOLEGACY: DWORD = 0x00000030;

/// If set, this enables the caller to receive the input even when the caller is not in the
/// foreground. Note that `wnd_target` must be specified.
pub const RIDEV_INPUTSINK: DWORD = 0x00000100;

/// If set, the mouse button click does not activate the other window. [`RIDEV_CAPTUREMOUSE`] can
/// be specified only if [`RIDEV_NOLEGACY`] is specified for a mouse device.
pub const RIDEV_CAPTUREMOUSE: DWORD = 0x00000200;

/// If set, the application-defined keyboard device hotkeys are not handled. However, the system
/// hotkeys; for example, `ALT+TAB` and `CTRL+ALT+DEL`, are still handled. By default, all keyboard
/// hotkeys are handled. [`RIDEV_NOHOTKEYS`] can be specified even if [`RIDEV_NOLEGACY`] is not
/// specified and `wnd_target` is [`null_mut`].
pub const RIDEV_NOHOTKEYS: DWORD = 0x00000200;

/// If set, the application command keys are handled. [`RIDEV_APPKEYS`] can be specified only if
/// [`RIDEV_NOLEGACY`] is specified for a keyboard device.
pub const RIDEV_APPKEYS: DWORD = 0x00000400;

/// If set, this enables the caller to receive input in the background only if the foreground
/// application does not process it. In other words, if the foreground application is not
/// registered for raw input, then the background application that is registered will receive the
/// input.
pub const RIDEV_EXINPUTSINK: DWORD = 0x00001000;

/// If set, this enables the caller to receive [`WM_INPUT_DEVICE_CHANGE`] notifications for device
/// arrival and device removal.
pub const RIDEV_DEVNOTIFY: DWORD = 0x00002000;
