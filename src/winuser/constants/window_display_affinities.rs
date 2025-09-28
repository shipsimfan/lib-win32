use crate::DWORD;

/// Imposes no restrictions on where the window can be displayed.
pub const WDA_NONE: DWORD = 0x00000000;

/// The window content is displayed only on a monitor. Everywhere else, the window appears with no
/// content.
pub const WDA_MONITOR: DWORD = 0x00000001;

/// The window is displayed only on a monitor. Everywhere else, the window does not appear at all.
/// One use for this affinity is for windows that show video recording controls, so that the
/// controls are not included in the capture.
pub const WDA_EXCLUDEFROMCAPTURE: DWORD = 0x00000011;
