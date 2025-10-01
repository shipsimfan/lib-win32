use crate::{dxgi1_3::DXGI_FRAME_PRESENTATION_MODE, LARGE_INTEGER, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi1_3::IDXGISwapChainMedia, QueryPerformanceCounter};

/// Used to verify system approval for the app's custom present duration (custom refresh rate).
/// Approval should be continuously verified on a frame-by-frame basis.
///
/// # Remarks
/// This structure is used with the [`IDXGISwapChainMedia::get_frame_statistics_media`] method.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    /// A value that represents the running total count of times that an image was presented to the
    /// monitor since the computer booted.
    pub present_count: UINT,

    /// A value that represents the running total count of v-blanks at which the last image was
    /// presented to the monitor and that have happened since the computer booted (for windowed
    /// mode, since the swap chain was created).
    pub present_refresh_count: UINT,

    /// A value that represents the running total count of v-blanks when the scheduler last sampled
    /// the machine time by calling [`QueryPerformanceCounter`] and that have happened since the
    /// computer booted (for windowed mode, since the swap chain was created).
    pub sync_refresh_count: UINT,

    /// A value that represents the high-resolution performance counter timer. This value is the
    /// same as the value returned by the [`QueryPerformanceCounter`] function.
    pub sync_qpc_time: LARGE_INTEGER,

    /// Reserved. Always returns 0.
    pub sync_gpu_time: LARGE_INTEGER,

    /// A value indicating the composition presentation mode. This value is used to determine
    /// whether the app should continue to use the decode swap chain. See
    /// [`DXGI_FRAME_PRESENTATION_MODE`].
    pub composition_mode: DXGI_FRAME_PRESENTATION_MODE,

    /// If the system approves an app's custom present duration request, this field is set to the
    /// approved custom present duration.
    ///
    /// If the app's custom present duration request is not approved, this field is set to zero.
    pub approved_present_duration: UINT,
}

impl Default for DXGI_FRAME_STATISTICS_MEDIA {
    fn default() -> Self {
        DXGI_FRAME_STATISTICS_MEDIA {
            present_count: 0,
            present_refresh_count: 0,
            sync_refresh_count: 0,
            sync_qpc_time: LARGE_INTEGER::default(),
            sync_gpu_time: LARGE_INTEGER::default(),
            composition_mode: DXGI_FRAME_PRESENTATION_MODE::Composed,
            approved_present_duration: 0,
        }
    }
}
