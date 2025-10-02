use crate::UINT64;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_4::IDXGIAdapter3;

/// Describes the current video memory budgeting parameters.
///
/// # Remarks
/// Use this structure with [`IDXGIAdapter3::query_video_memory_info`].
///
/// Refer to the remarks for [`D3D12_MEMORY_POOL`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    /// Specifies the OS-provided video memory budget, in bytes, that the application should
    /// target. If `current_usage` is greater than `budget`, the application may incur stuttering
    /// or performance penalties due to background activity by the OS to provide other applications
    /// with a fair usage of video memory.
    pub budget: UINT64,

    /// Specifies the application’s current video memory usage, in bytes.
    pub current_usage: UINT64,

    /// The amount of video memory, in bytes, that the application has available for reservation.
    /// To reserve this video memory, the application should call
    /// [`IDXGIAdapter3::set_video_memory_reservation`].
    pub available_for_reservation: UINT64,

    /// The amount of video memory, in bytes, that is reserved by the application. The OS uses the
    /// reservation as a hint to determine the application’s minimum working set. Applications
    /// should attempt to ensure that their video memory usage can be trimmed to meet this
    /// requirement.
    pub current_reservation: UINT64,
}

impl Default for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn default() -> Self {
        DXGI_QUERY_VIDEO_MEMORY_INFO {
            budget: 0,
            current_usage: 0,
            available_for_reservation: 0,
            current_reservation: 0,
        }
    }
}
