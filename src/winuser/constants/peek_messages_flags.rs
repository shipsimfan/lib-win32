use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::PeekMessage;

/// Messages are not removed from the queue after processing by [`PeekMessage`].
pub const PM_NOREMOVE: UINT = 0x0000;

/// Messages are removed from the queue after processing by [`PeekMessage`].
pub const PM_REMOVE: UINT = 0x0001;

/// Prevents the system from releasing any thread that is waiting for the caller to go idle (see
/// [`WaitForInputIdle`]).
///
/// Combine this value with either [`PM_NOREMOVE`] or [`PM_REMOVE`].
pub const PM_NOYIELD: UINT = 0x0002;
