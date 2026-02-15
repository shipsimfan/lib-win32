use crate::{
    DISPLAYCONFIG_2DREGION, DISPLAYCONFIG_RATIONAL, DISPLAYCONFIG_SCANLINE_ORDERING, UINT32, UINT64,
};
use std::ops::{Deref, DerefMut};

/// The [`DISPLAYCONFIG_VIDEO_SIGNAL_INFO`] structure contains information about the video signal
/// for a display.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    /// The pixel clock rate.
    pub pixel_rate: UINT64,

    /// A [`DISPLAYCONFIG_RATIONAL`] structure that represents horizontal sync.
    pub h_sync_freq: DISPLAYCONFIG_RATIONAL,

    /// A [`DISPLAYCONFIG_RATIONAL`] structure that represents vertical sync.
    pub v_sync_freq: DISPLAYCONFIG_RATIONAL,

    /// A [`DISPLAYCONFIG_2DREGION`] structure that specifies the width and height (in pixels) of
    /// the active portion of the video signal.
    pub activate_size: DISPLAYCONFIG_2DREGION,

    /// A [`DISPLAYCONFIG_2DREGION`] structure that specifies the width and height (in pixels) of
    /// the entire video signal.
    pub total_size: DISPLAYCONFIG_2DREGION,

    #[allow(missing_docs)]
    pub dummy: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_UNION,

    /// The scan-line ordering (for example, progressive or interlaced) of the video signal. For a
    /// list of possible values, see the [`DISPLAYCONFIG_SCANLINE_ORDERING`] enumerated type.
    pub scanline_ordering: DISPLAYCONFIG_SCANLINE_ORDERING,
}

impl Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    fn default() -> Self {
        DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
            pixel_rate: 0,
            h_sync_freq: DISPLAYCONFIG_RATIONAL::default(),
            v_sync_freq: DISPLAYCONFIG_RATIONAL::default(),
            activate_size: DISPLAYCONFIG_2DREGION::default(),
            total_size: DISPLAYCONFIG_2DREGION::default(),
            dummy: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_UNION::default(),
            scanline_ordering: DISPLAYCONFIG_SCANLINE_ORDERING::Unspecified,
        }
    }
}

impl Deref for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    type Target = DISPLAYCONFIG_VIDEO_SIGNAL_INFO_UNION;

    fn deref(&self) -> &Self::Target {
        &self.dummy
    }
}

impl DerefMut for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dummy
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub union DISPLAYCONFIG_VIDEO_SIGNAL_INFO_UNION {
    /// Supported by WDDM 1.3 and later display miniport drivers running on Windows 8.1 and later.
    pub additional_signal_info: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_STRUCT,

    /// The video standard (if any) that defines the video signal. For a list of possible values,
    /// see the [`D3DKMDT_VIDEO_SIGNAL_STANDARD`] enumerated type.
    pub video_standard: UINT32,
}

impl Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_UNION {
    fn default() -> Self {
        DISPLAYCONFIG_VIDEO_SIGNAL_INFO_UNION { video_standard: 0 }
    }
}

impl Deref for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_UNION {
    type Target = DISPLAYCONFIG_VIDEO_SIGNAL_INFO_STRUCT;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.additional_signal_info }
    }
}

impl DerefMut for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_UNION {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.additional_signal_info }
    }
}

/// Supported by WDDM 1.3 and later display miniport drivers running on Windows 8.1 and later.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO_STRUCT {
    /// # Bits 0 - 15
    /// The video standard (if any) that defines the video signal. For a list of possible values,
    /// see the [`D3DKMDT_VIDEO_SIGNAL_STANDARD`] enumerated type.
    ///
    /// Supported starting with Windows 8.1.
    ///
    /// # Bits 16 - 21
    /// The ratio of the VSync rate of a monitor that displays through a Miracast connected session
    /// to the VSync rate of the Miracast sink.
    ///
    /// To avoid visual artifacts, the VSync rate of the display monitor that's connected to the
    /// Miracast sink must be an integer multiple of the VSync rate of the Miracast sink. The
    /// display miniport driver reports the latter rate to the operating system as the refresh rate
    /// of the desktop present path.
    ///
    /// For a non-Miracast target, the driver should set `v_sync_freq_divider` to zero.
    ///
    /// Supported starting with Windows 8.1.
    pub value: UINT32,
}

impl Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_STRUCT {
    fn default() -> Self {
        DISPLAYCONFIG_VIDEO_SIGNAL_INFO_STRUCT { value: 0 }
    }
}
