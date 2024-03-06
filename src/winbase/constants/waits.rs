use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{FALSE, TRUE};

/// If `wait_all` is [`TRUE`], a return value in this range indicates that the state of all
/// specified objects is signaled.
///
/// If `wait_all` is [`FALSE`], the return value minus [`WAIT_OBJECT_0`] indicates the `handles`
/// array index of the object that satisfied the wait. If more than one object became signaled
/// during the call, this is the array index of the signaled object with the smallest index value
/// of all the signaled objects.
pub const WAIT_OBJECT_0: DWORD = 0;

/// If `wait_all` is [`TRUE`], a return value in this range indicates that the state of all
/// specified objects is signaled, and at least one of the objects is an abandoned mutex object.
///
/// If `wait_all` is [`FALSE`], the return value minus [`WAIT_ABANDONED_0`] indicates the `handles`
/// array index of an abandoned mutex object that satisfied the wait. Ownership of the mutex object
/// is granted to the calling thread, and the mutex is set to nonsignaled.
///
/// If a mutex was protecting persistent state information, you should check it for consistency.
pub const WAIT_ABANDONED_0: DWORD = 0x80;

/// The wait was ended by one or more user-mode asynchronous procedure calls (APC) queued to the
/// thread.
pub const WAIT_IO_COMPLETION: DWORD = 0x000000C0;

/// The time-out interval elapsed, the conditions specified by the `wait_all` parameter were not
/// satisfied, and no completion routines are queued.
pub const WAIT_TIMEOUT: DWORD = 0x00000102;

/// The function has failed.
pub const WAIT_FAILED: DWORD = 0xFFFFFFFF;

/// The maximum number of objects the wait functions can wait fors
pub const MAXIMUM_WAIT_OBJECTS: DWORD = 64;
