use crate::BYTE;

/// Don't care or don't know.
pub const FF_DONTCARE: BYTE = 0x00;

/// Variable stroke width, serifed.
///
/// Times Roman, Century Schoolbook, etc.
pub const FF_ROMAN: BYTE = 0x10;

/// Variable stroke width, sans-serifed.
///
/// Helvetica, Swiss, etc.
pub const FF_SWISS: BYTE = 0x20;

/// Constant stroke width, serifed or sans-serifed.
///
/// Pica, Elite, Courier, etc.
pub const FF_MODERN: BYTE = 0x30;

/// Cursive, etc.
pub const FF_SCRIPT: BYTE = 0x40;

/// Old English, etc.
pub const FF_DECORATIVE: BYTE = 0x50;
