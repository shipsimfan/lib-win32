use crate::BYTE;

/// Specifies default clipping behavior.
pub const CLIP_DEFAULT_PRECIS: BYTE = 0;

/// Not used.
pub const CLIP_CHARACTER_PRECIS: BYTE = 1;

/// Not used by the font mapper, but is returned when raster, vector, or TrueType fonts are
/// enumerated. For compatibility, this value is always returned when enumerating fonts.
pub const CLIP_STROKE_PRECIS: BYTE = 2;

/// Not used.
pub const CLIP_MASK: BYTE = 0xF;

/// When this value is used, the rotation for all fonts depends on whether the orientation of the
/// coordinate system is left-handed or right-handed. If not used, device fonts always rotate
/// counterclockwise, but the rotation of other fonts is dependent on the orientation of the
/// coordinate system.
pub const CLIP_LH_ANGLES: BYTE = 1 << 4;

/// Not used.
pub const CLIP_TT_ALWAYS: BYTE = 2 << 4;

/// Windows XP SP1: Turns off font association for the font. Note that this flag is not guaranteed
/// to have any effect on any platform after Windows Server 2003.
pub const CLIP_DFA_DISABLE: BYTE = 4 << 4;

///  Turns off font association for the font. This is identical to [`CLIP_DFA_DISABLE`], but it can
/// have problems in some situations; the recommended flag to use is [`CLIP_DFA_DISABLE`].
pub const CLIP_DFA_OVERRIDE: BYTE = CLIP_DFA_DISABLE;

/// You must specify this flag to use an embedded read-only font.
pub const CLIP_EMBEDDED: BYTE = 8 << 4;
