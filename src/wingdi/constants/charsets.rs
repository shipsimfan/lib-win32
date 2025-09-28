use crate::BYTE;

/// This font supports the Windows ANSI character set.
pub const ANSI_CHARSET: BYTE = 0;

/// This font supports character set value based on the system default Windows ANSI code page. For
/// example, when the system locale is English (United States), it is set as [`ANSI_CHARSET`].
pub const DEFAULT_CHARSET: BYTE = 1;

/// This font supports the Windows symbol character set.
pub const SYMBOL_CHARSET: BYTE = 2;

/// This font supports the Shift-JIS (Japanese Industry Standard) character set.
pub const SHIFTJIS_CHARSET: BYTE = 128;

/// This font supports the Korean (Hangul) character set.
pub const HANGEUL_CHARSET: BYTE = 129;

/// This font supports the Korean (Hangul) character set.
pub const HANGUL_CHARSET: BYTE = 129;

/// This font supports the simplified (PRC) Chinese character set.
pub const GB2312_CHARSET: BYTE = 134;

/// This font supports the traditional Chinese (Big 5) character set.
pub const CHINESEBIG5_CHARSET: BYTE = 136;

/// This font supports an OEM-specific character set. The OEM character set is system dependent.
pub const OEM_CHARSET: BYTE = 255;

/// This font supports the Korean (Johab) character set.
pub const JOHAB_CHARSET: BYTE = 130;

/// This font supports the Hebrew character set.
pub const HEBREW_CHARSET: BYTE = 177;

/// This font supports the Arabic character set.
pub const ARABIC_CHARSET: BYTE = 178;

/// This font supports the Greek character set.
pub const GREEK_CHARSET: BYTE = 161;

/// This font supports the Turkish character set.
pub const TURKISH_CHARSET: BYTE = 162;

/// This font supports the Vietnamese character set.
pub const VIETNAMESE_CHARSET: BYTE = 163;

/// This font supports the Thai character set.
pub const THAI_CHARSET: BYTE = 222;

/// This font supports the Eastern European character set.
pub const EASTEUROPE_CHARSET: BYTE = 238;

/// This font supports the Cyrillic character set.
pub const RUSSIAN_CHARSET: BYTE = 204;

/// This font supports character set value based on the current system Macintosh code page. This
/// value is used primarily in legacy code and should not generally be needed since modern
/// Macintosh computers use Unicode for encoding.
pub const MAC_CHARSET: BYTE = 77;

/// This font supports the Baltic character set.
pub const BALTIC_CHARSET: BYTE = 186;
