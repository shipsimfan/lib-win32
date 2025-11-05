use crate::USHORT;

/// The key is down.
pub const RI_KEY_MAKE: USHORT = 0;

/// The key is up.
pub const RI_KEY_BREAK: USHORT = 1;

/// The scan code has the E0 prefix.
pub const RI_KEY_E0: USHORT = 2;

/// The scan code has the E1 prefix.
pub const RI_KEY_E1: USHORT = 4;

#[allow(missing_docs)]
pub const KEYBOARD_OVERRUN_MAKE_CODE: USHORT = 0xFF;
