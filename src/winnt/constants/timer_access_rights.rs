use crate::{DWORD, STANDARD_RIGHTS_REQUIRED, SYNCHRONIZE};

// rustdoc imports
#[allow(unused_imports)]
use crate::SetWaitableTimer;

/// Reserved for future use.
pub const TIMER_QUERY_STATE: DWORD = 0x0001;

/// Modify state access, which is required for the [`SetWaitableTimer`] and [`CancelWaitableTimer`]
/// functions.
pub const TIMER_MODIFY_STATE: DWORD = 0x0002;

/// All possible access rights for a waitable timer object. Use this right only if your application
/// requires access beyond that granted by the standard access rights and [`TIMER_MODIFY_STATE`].
/// Using this access right increases the possibility that your application must be run by an
/// Administrator.
pub const TIMER_ALL_ACCESS: DWORD =
    STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | TIMER_QUERY_STATE | TIMER_MODIFY_STATE;
