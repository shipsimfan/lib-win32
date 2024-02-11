use crate::{ACCESS_MASK, LONG};

/// A registry result
pub type LSTATUS = LONG;

/// A data type used for specifying the security access attributes in the registry
pub type REGSAM = ACCESS_MASK;
