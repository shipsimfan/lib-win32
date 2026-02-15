use crate::DISPLAYCONFIG_VIDEO_SIGNAL_INFO;

/// The [`DISPLAYCONFIG_TARGET_MODE`] structure describes a display path target mode.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_TARGET_MODE {
    /// A [`DISPLAYCONFIG_VIDEO_SIGNAL_INFO`] structure that contains a detailed description of the
    /// current target mode.
    pub target_video_signal_info: DISPLAYCONFIG_VIDEO_SIGNAL_INFO,
}

impl Default for DISPLAYCONFIG_TARGET_MODE {
    fn default() -> Self {
        DISPLAYCONFIG_TARGET_MODE {
            target_video_signal_info: DISPLAYCONFIG_VIDEO_SIGNAL_INFO::default(),
        }
    }
}
