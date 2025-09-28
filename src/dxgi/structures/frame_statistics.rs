use crate::{LARGE_INTEGER, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGIOutput, IDXGISwapChain, DXGI_SWAP_EFFECT},
    QueryPerformanceCounter,
};

/// Describes timing and presentation statistics for a frame.
///
/// You initialize the [`DXGI_FRAME_STATISTICS`] structure with the
/// [`IDXGIOutput::get_frame_statistics`] or [`IDXGISwapChain::get_frame_statistics`] method.
///
/// You can only use [`IDXGISwapChain::get_frame_statistics`] for swap chains that either use the
/// flip presentation model or draw in full-screen mode. You set the
/// [`DXGI_SWAP_EFFECT::FlipSequential`] value in the `swap_effect` member of the
/// [`DXGI_SWAP_CHAIN_DESC1`] structure to specify that the swap chain uses the flip presentation
/// model.
///
/// The values in the `present_count` and `present_refresh_count` members indicate information
/// about when a frame was presented on the display screen. You can use these values to determine
/// whether a glitch occurred. The values in the `sync_refresh_count` and `sync_qpc_time` members
/// indicate timing information that you can use for audio and video synchronization or very
/// precise animation. If the swap chain draws in full-screen mode, these values are based on when
/// the computer booted. If the swap chain draws in windowed mode, these values are based on when
/// the swap chain is created.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_FRAME_STATISTICS {
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
}

impl Default for DXGI_FRAME_STATISTICS {
    fn default() -> Self {
        DXGI_FRAME_STATISTICS {
            present_count: 0,
            present_refresh_count: 0,
            sync_refresh_count: 0,
            sync_qpc_time: LARGE_INTEGER::default(),
            sync_gpu_time: LARGE_INTEGER::default(),
        }
    }
}
