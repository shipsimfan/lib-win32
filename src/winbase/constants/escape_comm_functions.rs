use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::EscapeCommFunction;

/// Restores character transmission and places the transmission line in a nonbreak state. The
/// [`CLRBREAK`] extended function code is identical to the [`ClearCommBreak`] function.
pub const CLRBREAK: DWORD = 9;

/// Clears the DTR (data-terminal-ready) signal.
pub const CLRDTR: DWORD = 6;

/// Clears the RTS (request-to-send) signal.
pub const CLRRTS: DWORD = 4;

/// Sends the DTR (data-terminal-ready) signal.
pub const SETDTR: DWORD = 5;

/// Suspends character transmission and places the transmission line in a break state until the
/// [`ClearCommBreak`] function is called (or [`EscapeCommFunction`] is called with the
/// [`CLRBREAK`] extended function code). The [`SETBREAK`] extended function code is identical to
/// the [`SetCommBreak`] function. Note that this extended function does not flush data that has
/// not been transmitted.
pub const SETBREAK: DWORD = 8;

/// Sends the RTS (request-to-send) signal.
pub const SETRTS: DWORD = 3;

/// Causes transmission to act as if an XOFF character has been received.
pub const SETXOFF: DWORD = 1;

/// Causes transmission to act as if an XON character has been received.
pub const SETXON: DWORD = 2;
