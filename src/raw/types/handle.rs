use crate::raw::{Long, PVoid};

// rustdoc imports
#[allow(unused_imports)]
use crate::{FAILED, SUCCEEDED};

/// A handle to an object.
pub type Handle = PVoid;

/// A handle to a local memory block.
pub type HLocal = Handle;

/// The return codes used by COM interfaces. To test an [`HResult`] value, use
/// the [`FAILED`] and [`SUCCEEDED`] macros.
pub type HResult = Long;
