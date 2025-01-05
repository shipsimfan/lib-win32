use crate::GUID;

/// A pointer to a [`GUID`]
pub type LPGUID = *mut GUID;

/// Describes a GUID structure used to describe an identifier for a COM interface.
pub type IID = GUID;

/// A reference to an [`IID`]
pub type REFIID = *const IID;
