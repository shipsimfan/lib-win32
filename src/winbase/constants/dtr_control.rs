use crate::DWORD;

/// Disables the DTR line when the device is opened and leaves it disabled.
pub const DTR_CONTROL_DISABLE: DWORD = 0x00 << 5;

/// Enables the DTR line when the device is opened and leaves it on.
pub const DTR_CONTROL_ENABLE: DWORD = 0x01 << 5;

/// Enables DTR handshaking. If handshaking is enabled, it is an error for the application to
/// adjust the line by using the [`EscapeCommFunction`] function.
pub const DTR_CONTROL_HANDSHAKE: DWORD = 0x02 << 5;
