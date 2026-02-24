use crate::WORD;

/// Change affects media in drive. If not set, change affects physical device or drive.
pub const DBTF_MEDIA: WORD = 0x0001;

///  Indicated logical volume is a network volume.
pub const DBTF_NET: WORD = 0x0002;
