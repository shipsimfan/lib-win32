// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_4::IDXGIAdapter3;

/// Specifies the memory segment group to use.
///
/// # Remarks
/// This enum is used by [`IDXGIAdapter3::query_video_memory_info`] and
/// [`IDXGIAdapter3::set_video_memory_reservation`].
///
/// Refer to the remarks for [`D3D12_MEMORY_POOL`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_MEMORY_SEGMENT_GROUP {
    /// The grouping of segments which is considered local to the video adapter, and represents the
    /// fastest available memory to the GPU. Applications should target the local segment group as
    /// the target size for their working set.
    Local = 0,

    /// The grouping of segments which is considered non-local to the video adapter, and may have
    /// slower performance than the local segment group.
    NonLocal = 1,
}
