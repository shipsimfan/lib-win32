use crate::DWORD;

/// Disables the RTS line when the device is opened and leaves it disabled.
pub const RTS_CONTROL_DISABLE: DWORD = 0x00 << 13;

/// Enables the RTS line when the device is opened and leaves it on.
pub const RTS_CONTROL_ENABLE: DWORD = 0x01 << 13;

/// Enables RTS handshaking. The driver raises the RTS line when the "type-ahead" (input) buffer is
/// less than one-half full and lowers the RTS line when the buffer is more than three-quarters
/// full. If handshaking is enabled, it is an error for the application to adjust the line by using
/// the [`EscapeCommFunction`] function.
pub const RTS_CONTROL_HANDSHAKE: DWORD = 0x02 << 13;

/// Specifies that the RTS line will be high if bytes are available for transmission. After all
/// buffered bytes have been sent, the RTS line will be low.
pub const RTS_CONTROL_TOGGLE: DWORD = 0x03 << 13;
