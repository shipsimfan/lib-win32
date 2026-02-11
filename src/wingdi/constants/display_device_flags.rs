use crate::DWORD;

#[allow(missing_docs)]
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: DWORD = 0x00000001;

#[allow(missing_docs)]
pub const DISPLAY_DEVICE_MULTI_DRIVER: DWORD = 0x00000002;

/// The primary desktop is on the device. For a system with a single display card, this is always
/// set. For a system with multiple display cards, only one device can have this set.
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: DWORD = 0x00000004;

/// Represents a pseudo device used to mirror application drawing for remoting or other purposes.
/// An invisible pseudo monitor is associated with this device. For example, NetMeeting uses it.
/// Note that [`GetSystemMetrics`] ([`SM_MONITORS`]) only accounts for visible display monitors.
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: DWORD = 0x00000008;

/// The device is VGA compatible.
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: DWORD = 0x00000010;

/// The device is removable; it cannot be the primary display.
pub const DISPLAY_DEVICE_REMOVABLE: DWORD = 0x00000020;

#[allow(missing_docs)]
pub const DISPLAY_DEVICE_ACC_DRIVER: DWORD = 0x00000040;

/// The device has more display modes than its output devices support.
pub const DISPLAY_DEVICE_MODESPRUNED: DWORD = 0x08000000;

#[allow(missing_docs)]
pub const DISPLAY_DEVICE_RDPUDD: DWORD = 0x01000000;

#[allow(missing_docs)]
pub const DISPLAY_DEVICE_REMOTE: DWORD = 0x04000000;

#[allow(missing_docs)]
pub const DISPLAY_DEVICE_DISCONNECT: DWORD = 0x02000000;

#[allow(missing_docs)]
pub const DISPLAY_DEVICE_TS_COMPATIBLE: DWORD = 0x00200000;

#[allow(missing_docs)]
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: DWORD = 0x00080000;

/// [`DISPLAY_DEVICE_ACTIVE`] specifies whether a monitor is presented as being "on" by the
/// respective GDI view. Windows Vista: EnumDisplayDevices will only enumerate monitors that can be
/// presented as being "on."
pub const DISPLAY_DEVICE_ACTIVE: DWORD = 0x00000001;

#[allow(missing_docs)]
pub const DISPLAY_DEVICE_ATTACHED: DWORD = 0x00000002;
