use crate::BYTE;

/// Appearance of the font does not matter.
pub const DEFAULT_QUALITY: BYTE = 0;

/// Appearance of the font is less important than when [`PROOF_QUALITY`] is used. For GDI raster
/// fonts, scaling is enabled, which means that more font sizes are available, but the quality may
/// be lower. Bold, italic, underline, and strikeout fonts are synthesized if necessary.
pub const DRAFT_QUALITY: BYTE = 1;

/// Character quality of the font is more important than exact matching of the logical-font
/// attributes. For GDI raster fonts, scaling is disabled and the font closest in size is chosen.
/// Although the chosen font size may not be mapped exactly when [`PROOF_QUALITY`] is used, the
/// quality of the font is high and there is no distortion of appearance. Bold, italic, underline,
/// and strikeout fonts are synthesized if necessary.
pub const PROOF_QUALITY: BYTE = 2;

/// Font is never antialiased.
pub const NONANTIALIASED_QUALITY: BYTE = 3;

/// Font is always antialiased if the font supports it and the size of the font is not too small or
/// too large.
pub const ANTIALIASED_QUALITY: BYTE = 4;

/// If set, text is rendered (when possible) using ClearType antialiasing method.
pub const CLEARTYPE_QUALITY: BYTE = 5;

#[allow(missing_docs)]
pub const CLEARTYPE_NATURAL_QUALITY: BYTE = 6;
