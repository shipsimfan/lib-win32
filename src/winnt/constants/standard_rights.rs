use crate::REGSAM;

/// The right to delete the object.
pub const DELETE: REGSAM = 0x00010000;

/// The right to read the information in the object's security descriptor, not including the
/// information in the system access control list (SACL).
pub const READ_CONTROL: REGSAM = 0x00020000;

/// The right to modify the discretionary access control list (DACL) in the object's security
/// descriptor.
pub const WRITE_DAC: REGSAM = 0x00040000;

/// The right to change the owner in the object's security descriptor.
pub const WRITE_OWNER: REGSAM = 0x00080000;

/// The right to use the object for synchronization. This enables a thread to wait until the object
/// is in the signaled state. Some object types do not support this access right.
pub const SYNCHRONIZE: REGSAM = 0x00100000;

/// Combines [`DELETE`], [`READ_CONTROL`], [`WRITE_DAC`], and [`WRITE_OWNER`] access
pub const STANDARD_RIGHTS_REQUIRED: REGSAM = 0x000F0000;

/// Currently defined to equal [`READ_CONTROL`]
pub const STANDARD_RIGHTS_READ: REGSAM = READ_CONTROL;

/// Currently defined to equal [`READ_CONTROL`]
pub const STANDARD_RIGHTS_WRITE: REGSAM = READ_CONTROL;

/// Currently defined to equal [`READ_CONTROL`]
pub const STANDARD_RIGHTS_EXECUTE: REGSAM = READ_CONTROL;

/// Combines [`DELETE`], [`READ_CONTROL`], [`WRITE_DAC`], [`WRITE_OWNER`], and [`SYNCHRONIZE`]
/// access
pub const STANDARD_RIGHTS_ALL: REGSAM = 0x001F0000;
