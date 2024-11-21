use crate::{BYTE, DWORD, WORD};
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CBR_110, CBR_115200, CBR_1200, CBR_128000, CBR_14400, CBR_19200, CBR_2400, CBR_256000, CBR_300,
    CBR_38400, CBR_4800, CBR_57600, CBR_600, CBR_9600, DTR_CONTROL_DISABLE, DTR_CONTROL_ENABLE,
    DTR_CONTROL_HANDSHAKE, EVENPARITY, FALSE, MARKPARITY, NOPARITY, ODDPARITY, ONE5STOPBITS,
    ONESTOPBIT, RTS_CONTROL_DISABLE, RTS_CONTROL_ENABLE, RTS_CONTROL_HANDSHAKE, RTS_CONTROL_TOGGLE,
    SPACEPARITY, TRUE, TWOSTOPBITS,
};

/// Defines the control setting for a serial communications device.
///
/// When a DCB structure is used to configure the 8250, the following restrictions apply to the
/// values specified for the `byte_size` and `stop_bits` members:
///  - The number of data bits must be 5 to 8 bits.
///  - The use of 5 data bits with 2 stop bits is an invalid combination, as is 6, 7, or 8 data
///    bits with 1.5 stop bits.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DCB {
    /// The length of the structure, in bytes. The caller must set this member to
    /// `std::mem::size_of::<DCB>()`
    pub length: DWORD,

    /// The baud rate at which the communications device operates. This member can be an actual
    /// baud rate value, or one of the following indexes:
    ///  - [`CBR_110`] - 110 bps
    ///  - [`CBR_300`] - 300 bps
    ///  - [`CBR_600`] - 600 bps
    ///  - [`CBR_1200`] - 1200 bps
    ///  - [`CBR_2400`] - 2400 bps
    ///  - [`CBR_4800`] - 4800 bps
    ///  - [`CBR_9600`] - 9600 bps
    ///  - [`CBR_14400`] - 14400 bps
    ///  - [`CBR_19200`] - 19200 bps
    ///  - [`CBR_38400`] - 38400 bps
    ///  - [`CBR_57600`] - 57600 bps
    ///  - [`CBR_115200`] - 115200 bps
    ///  - [`CBR_128000`] - 128000 bps
    ///  - [`CBR_256000`] - 256000 bps
    pub baud_rate: DWORD,

    /// The flags consist of the following bits:
    ///  1. `binary` - If this member is [`TRUE`], binary mode is enabled. Windows does not support
    ///                nonbinary mode transfers, so this member must be [`TRUE`].
    ///  2. `parity` - If this member is [`TRUE`], parity checking is performed and errors are
    ///                reported.
    ///  3. `out_x_cts_flow` - If this member is [`TRUE`], the CTS (clear-to-send) signal is
    ///                        monitored for output flow control. If this member is [`TRUE`] and
    ///                        CTS is turned off, output is suspended until CTS is sent again.
    ///  4. `out_x_dsr_flow` - If this member is [`TRUE`], the DSR (data-set-ready) signal is
    ///                        monitored for output flow control. If this member is [`TRUE`] and
    ///                        DSR is turned off, output is suspended until DSR is sent again.
    ///  5. (2-bits) `dtr_control` - The DTR (data-terminal-ready) flow control. This member can be
    ///                              one of the following values:
    ///    * [`DTR_CONTROL_DISABLE`] - Disables the DTR line when the device is opened and leaves
    ///                                it disabled.
    ///    * [`DTR_CONTROL_ENABLE`] - Enables the DTR line when the device is opened and leaves it
    ///                               on.
    ///    * [`DTR_CONTROL_HANDSHAKE`] - Enables DTR handshaking. If handshaking is enabled, it is
    ///                                  an error for the application to adjust the line by using
    ///                                  the [`EscapeCommFunction`] function.
    ///  7. `dsr_sensitivity` - If this member is [`TRUE`], the communications driver is sensitive
    ///                         to the state of the DSR signal. The driver ignores any bytes
    ///                         received, unless the DSR modem input line is high.
    ///  8. `tx_continue_on_x_off` - If this member is [`TRUE`], transmission continues after the
    ///                              input buffer has come within `x_off_lim` bytes of being full
    ///                              and the driver has transmitted the `x_off_char` character to
    ///                              stop receiving bytes. If this member is [`FALSE`],
    ///                              transmission does not continue until the input buffer is
    ///                              within `x_on_lim` bytes of being empty and the driver has
    ///                              transmitted the `x_on_char` character to resume reception.
    ///  9. `out_x` - Indicates whether XON/XOFF flow control is used during transmission. If this
    ///               member is [`TRUE`], transmission stops when the `x_off_char` character is
    ///               received and starts again when the `x_on_char` character is received.
    ///  10. `in_x` - Indicates whether XON/XOFF flow control is used during reception. If this
    ///               member is [`TRUE`], the `x_off_char` character is sent when the input buffer
    ///               comes within `x_off_lim` bytes of being full, and the `x_on_char` character
    ///               is sent when the input buffer comes within `x_on_lim` bytes of being empty.
    ///  11. `error_char` - Indicates whether bytes received with parity errors are replaced with
    ///                     the character specified by the `error_char` member. If this member is
    ///                     [`TRUE`] and the `parity` member is [`TRUE`], replacement occurs.
    ///  12. `null` - If this member is [`TRUE`], null bytes are discarded when received.
    ///  13. (2-bits) `rts_control` - The RTS (request-to-send) flow control. This member can be
    ///                               one of the following values:
    ///    * [`RTS_CONTROL_DISABLE`] - Disables the RTS line when the device is opened and leaves
    ///                                it disabled.
    ///    * [`RTS_CONTROL_ENABLE`] - Enables the RTS line when the device is opened and leaves it
    ///                               on.
    ///    * [`RTS_CONTROL_HANDSHAKE`] - Enables RTS handshaking. The driver raises the RTS line
    ///                                  when the "type-ahead" (input) buffer is less than one-half
    ///                                  full and lowers the RTS line when the buffer is more than
    ///                                  three-quarters full. If handshaking is enabled, it is an
    ///                                  error for the application to adjust the line by using the
    ///                                  [`EscapeCommFunction`] function.
    ///    * [`RTS_CONTROL_TOGGLE`] - Specifies that the RTS line will be high if bytes are
    ///                               available for transmission. After all buffered bytes have
    ///                               been sent, the RTS line will be low.
    ///  15. `abort_on_error` - If this member is [`TRUE`], the driver terminates all read and
    ///                         write operations with an error status if an error occurs. The
    ///                         driver will not accept any further communications operations until
    ///                         the application has acknowledged the error by calling the
    ///                         [`ClearCommError`] function.
    ///
    /// Remaining are reserved and must be 0
    pub flags: DWORD,

    /// Reserved; must be zero.
    pub reserved: WORD,

    /// The minimum number of bytes in use allowed in the input buffer before flow control is
    /// activated to allow transmission by the sender. This assumes that either XON/XOFF, RTS, or
    /// DTR input flow control is specified in the `in_x`, `rts_control`, or `dtr_control` members.
    pub x_on_lim: WORD,

    /// The minimum number of free bytes allowed in the input buffer before flow control is
    /// activated to inhibit the sender. Note that the sender may transmit characters after the
    /// flow control signal has been activated, so this value should never be zero. This assumes
    /// that either XON/XOFF, RTS, or DTR input flow control is specified in the `in_x`,
    /// `rts_control`, or `dtr_control` members. The maximum number of bytes in use allowed is
    /// calculated by subtracting this value from the size, in bytes, of the input buffer.
    pub x_off_lim: WORD,

    /// The number of bits in the bytes transmitted and received.
    pub byte_size: BYTE,

    /// The parity scheme to be used. This member can be one of the following values:
    ///  * [`EVENPARITY`] - Even parity
    ///  * [`MARKPARITY`] - Mark parity
    ///  * [`NOPARITY`] - No parity
    ///  * [`ODDPARITY`] - Odd parity
    ///  * [`SPACEPARITY`] - Space parity
    pub parity: BYTE,

    /// The number of stop bits to be used. This member can be one of the following values:
    ///  * [`ONESTOPBIT`] - 1 stop bit
    ///  * [`ONE5STOPBITS`] - 1.5 stop bits
    ///  * [`TWOSTOPBITS`] - 2 stop bits
    pub stop_bits: BYTE,

    /// The value of the XON character for both transmission and reception.
    pub x_on_char: c_char,

    /// The value of the XOFF character for both transmission and reception.
    pub x_off_char: c_char,

    /// The value of the character used to replace bytes received with a parity error.
    pub error_char: c_char,

    /// The value of the character used to signal the end of data.
    pub eof_char: c_char,

    /// The value of the character used to signal an event.
    pub evt_char: c_char,

    /// Reserved; do not use.
    pub reserved_1: WORD,
}
