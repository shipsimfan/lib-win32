use crate::GUID;

/// Describes a GUID structure used to describe an identifier for a COM interface.
pub type IID = GUID;

/// A pointer to a [`GUID`]
pub type LPGUID = *mut GUID;

/// A reference to an [`IID`]
pub type REFCLSID = *const IID;

/// A reference to an [`IID`]
pub type REFIID = *const IID;

/// A reference to a [`GUID`]
pub type REFGUID = *const GUID;
