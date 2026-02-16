use std::ffi::c_int;

/// Device is capable of character output precision.
pub const TC_OP_CHARACTER: c_int = 0x00000001;

/// Device is capable of stroke output precision.
pub const TC_OP_STROKE: c_int = 0x00000002;

/// Device is capable of stroke clip precision.
pub const TC_CP_STROKE: c_int = 0x00000004;

/// Device is capable of 90-degree character rotation.
pub const TC_CR_90: c_int = 0x00000008;

/// Device is capable of any character rotation.
pub const TC_CR_ANY: c_int = 0x00000010;

/// Device can scale independently in the x- and y-directions.
pub const TC_SF_X_YINDEP: c_int = 0x00000020;

/// Device is capable of doubled character for scaling.
pub const TC_SA_DOUBLE: c_int = 0x00000040;

/// Device uses integer multiples only for character scaling.
pub const TC_SA_INTEGER: c_int = 0x00000080;

/// Device uses any multiples for exact character scaling.
pub const TC_SA_CONTIN: c_int = 0x00000100;

/// Device can draw double-weight characters.
pub const TC_EA_DOUBLE: c_int = 0x00000200;

/// Device can italicize.
pub const TC_IA_ABLE: c_int = 0x00000400;

/// Device can underline.
pub const TC_UA_ABLE: c_int = 0x00000800;

/// Device can draw strikeouts.
pub const TC_SO_ABLE: c_int = 0x00001000;

/// Device can draw raster fonts.
pub const TC_RA_ABLE: c_int = 0x00002000;

/// Device can draw vector fonts.
pub const TC_VA_ABLE: c_int = 0x00004000;

/// Reserved; must be zero.
pub const TC_RESERVED: c_int = 0x00008000;

/// Device cannot scroll using a bit-block transfer. Note that this meaning may be the opposite of
/// what you expect.        
pub const TC_SCROLLBLT: c_int = 0x00010000;
