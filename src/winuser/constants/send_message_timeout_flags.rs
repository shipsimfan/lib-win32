use crate::UINT;

/// The calling thread is not prevented from processing other requests while waiting for the
/// function to return.
pub const SMTO_NORMAL: UINT = 0x0000;

/// Prevents the calling thread from processing any other requests until the function returns.
pub const SMTO_BLOCK: UINT = 0x0001;

/// The function returns without waiting for the time-out period to elapse if the receiving thread
/// appears to not respond or "hangs."
pub const SMTO_ABORTIFHUNG: UINT = 0x0002;

/// The function does not enforce the time-out period as long as the receiving thread is processing
/// messages.
pub const SMTO_NOTIMEOUTIFNOTHUNG: UINT = 0x0008;

/// The function should return 0 if the receiving window is destroyed or its owning thread dies
/// while the message is being processed.
pub const SMTO_ERRORONEXIT: UINT = 0x0020;
