use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::SetForegroundWindow;

///  Disables calls to [`SetForegroundWindow`].
pub const LSFW_LOCK: UINT = 1;

///  Enables calls to [`SetForegroundWindow`].
pub const LSFW_UNLOCK: UINT = 2;
