use crate::BYTE;

/// Specifies the default font mapper behavior.
pub const OUT_DEFAULT_PRECIS: BYTE = 0;

/// This value is not used by the font mapper, but it is returned when raster fonts are enumerated.
pub const OUT_STRING_PRECIS: BYTE = 1;

/// Not used.
pub const OUT_CHARACTER_PRECIS: BYTE = 2;

/// This value is not used by the font mapper, but it is returned when TrueType, other
/// outline-based fonts, and vector fonts are enumerated.
pub const OUT_STROKE_PRECIS: BYTE = 3;

/// Instructs the font mapper to choose from only TrueType fonts. If there are no TrueType fonts
/// installed in the system, the font mapper returns to default behavior.
pub const OUT_TT_PRECIS: BYTE = 4;

/// Instructs the font mapper to choose a Device font when the system contains multiple fonts with
/// the same name.
pub const OUT_DEVICE_PRECIS: BYTE = 5;

/// Instructs the font mapper to choose a raster font when the system contains multiple fonts with
/// the same name.
pub const OUT_RASTER_PRECIS: BYTE = 6;

/// Instructs the font mapper to choose from only TrueType fonts. If there are no TrueType fonts
/// installed in the system, the font mapper returns to default behavior.
pub const OUT_TT_ONLY_PRECIS: BYTE = 7;

/// This value instructs the font mapper to choose from TrueType and other outline-based fonts.
pub const OUT_OUTLINE_PRECIS: BYTE = 8;

#[allow(missing_docs)]
pub const OUT_SCREEN_OUTLINE_PRECIS: BYTE = 9;

/// Instructs the font mapper to choose from only PostScript fonts. If there are no PostScript
/// fonts installed in the system, the font mapper returns to default behavior.
pub const OUT_PS_ONLY_PRECIS: BYTE = 10;
