use crate::GUID;

/// The [`KNOWNFOLDERID`] constants represent [`GUID`]s that identify standard folders registered
/// with the system as Known Folders. These folders are installed with Windows Vista and later
/// operating systems, and a computer will have only folders appropriate to it installed.
pub type KNOWNFOLDERID = GUID;

/// A pointer to a [`KNOWNFOLDERID`]
pub type REFKNOWNFOLDERID = *const KNOWNFOLDERID;
