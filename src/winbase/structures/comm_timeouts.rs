use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::ReadFile;

/// Contains the time-out parameters for a communications device. The parameters determine the
/// behavior of [`ReadFile`], [`WriteFile`], [`ReadFileEx`], and [`WriteFileEx`] operations on the
/// device.
///
/// # Remarks
/// If an application sets `read_interval_timeout` and `read_total_timeout_multiplier` to
/// [`MAXDWORD`] and sets `read_total_timeout_contant` to a value greater than zero and less than
/// [`MAXDWORD`], one of the following occurs when the [`ReadFile`] function is called:
///  - If there are any bytes in the input buffer, [`ReadFile`] returns immediately with the bytes
///    in the buffer.
///  - If there are no bytes in the input buffer, [`ReadFile`] waits until a byte arrives and then
///    returns immediately.
///  - If no bytes arrive within the time specified by `read_total_timeout_contant`, [`ReadFile`]
///    times out.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct COMMTIMEOUTS {
    /// The maximum time allowed to elapse before the arrival of the next byte on the
    /// communications line, in milliseconds. If the interval between the arrival of any two bytes
    /// exceeds this amount, the [`ReadFile`] operation is completed and any buffered data is
    /// returned. A value of zero indicates that interval time-outs are not used.
    ///
    /// A value of [`MAXDWORD`], combined with zero values for both the
    /// `read_total_timeout_contant` and `read_total_timeout_multiplier` members, specifies that
    /// the read operation is to return immediately with the bytes that have already been received,
    /// even if no bytes have been received.
    pub read_interval_timeout: DWORD,

    /// The multiplier used to calculate the total time-out period for read operations, in
    /// milliseconds. For each read operation, this value is multiplied by the requested number of
    /// bytes to be read.
    pub read_total_timeout_mulitplier: DWORD,

    /// A constant used to calculate the total time-out period for read operations, in
    /// milliseconds. For each read operation, this value is added to the product of the
    /// `read_total_timeout_multiplier` member and the requested number of bytes.
    ///
    /// A value of zero for both the `read_total_timeout_multiplier` and
    /// `read_total_timeout_contant` members indicates that total time-outs are not used for read
    /// operations.
    pub read_total_timeout_constant: DWORD,

    /// The multiplier used to calculate the total time-out period for write operations, in
    /// milliseconds. For each write operation, this value is multiplied by the number of bytes to
    /// be written.
    pub write_total_timeout_multiplier: DWORD,

    /// A constant used to calculate the total time-out period for write operations, in
    /// milliseconds. For each write operation, this value is added to the product of the
    /// `write_total_timeout_multiplier` member and the number of bytes to be written.
    ///
    /// A value of zero for both the `write_total_timeout_multiplier` and
    /// `write_total_timeout_constant` members indicates that total time-outs are not used for
    /// write operations.
    pub write_total_timeout_constant: DWORD,
}
