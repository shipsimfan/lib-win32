use crate::{dxgi1_2::DXGI_OUTDUPL_POINTER_POSITION, BOOL, LARGE_INTEGER, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi1_2::IDXGIOutputDuplication, QueryPerformanceCounter, FALSE, TRUE};

/// The [`DXGI_OUTDUPL_FRAME_INFO`] structure describes the current desktop image.
///
/// # Remarks
/// A non-zero `last_mouse_update_time` indicates an update to either a mouse pointer position or a
/// mouse pointer position and shape. That is, the mouse pointer position is always valid for a
/// non-zero `last_mouse_update_time`; however, the application must check the value of the
/// `pointer_shape_buffer_size` member to determine whether the shape was updated too.
///
/// If only the pointer was updated (that is, the desktop image was not updated), the
/// `accumulated_frames`, `total_metadata_buffer_size`, and `last_present_time` members are set to
/// zero.
///
/// An `accumulated_frames` value of one indicates that the application completed processing the
/// last frame before a new desktop image was presented. If the `accumulated_frames` value is
/// greater than one, more desktop image updates have occurred while the application processed the
/// last desktop update. In this situation, the operating system accumulated the update regions.
///
/// A non-zero `total_metadata_buffer_size` indicates the total size of the buffers that are
/// required to store all the desktop update metadata. An application cannot determine the size of
/// each type of metadata. The application must call the
/// [`IDXGIOutputDuplication::get_frame_dirty_rects`],
/// [`IDXGIOutputDuplication::get_frame_move_rects`], or
/// [`IDXGIOutputDuplication::get_frame_pointer_shape`] method to obtain information about each
/// type of metadata.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    /// The time stamp of the last update of the desktop image. The operating system calls the
    /// [`QueryPerformanceCounter`] function to obtain the value. A zero value indicates that the
    /// desktop image was not updated since an application last called the
    /// [`IDXGIOutputDuplication::acquire_next_frame`] method to acquire the next frame of the
    /// desktop image.
    pub last_present_time: LARGE_INTEGER,

    /// The time stamp of the last update to the mouse. The operating system calls the
    /// [`QueryPerformanceCounter`] function to obtain the value. A zero value indicates that the
    /// position or shape of the mouse was not updated since an application last called the
    /// [`IDXGIOutputDuplication::acquire_next_frame`] method to acquire the next frame of the
    /// desktop image. The mouse position is always supplied for a mouse update. A new pointer
    /// shape is indicated by a non-zero value in the `pointer_shape_buffer_size` member.
    pub last_mouse_update_time: LARGE_INTEGER,

    /// The number of frames that the operating system accumulated in the desktop image surface
    /// since the calling application processed the last desktop image.
    pub accumulated_frames: UINT,

    /// Specifies whether the operating system accumulated updates by coalescing dirty regions.
    /// Therefore, the dirty regions might contain unmodified pixels. [`TRUE`] if dirty regions
    /// were accumulated; otherwise, [`FALSE`].
    pub rects_coalesced: BOOL,

    /// Specifies whether the desktop image might contain protected content that was already
    /// blacked out in the desktop image. [`TRUE`] if protected content was already blacked;
    /// otherwise, [`FALSE`]. The application can use this information to notify the remote user
    /// that some of the desktop content might be protected and therefore not visible.
    pub protected_content_masked_out: BOOL,

    /// A [`DXGI_OUTDUPL_POINTER_POSITION`] structure that describes the most recent mouse position
    /// if the `last_mouse_update_time` member is a non-zero value; otherwise, this value is
    /// ignored. This value provides the coordinates of the location where the top-left-hand corner
    /// of the pointer shape is drawn; this value is not the desktop position of the hot spot.
    pub pointer_position: DXGI_OUTDUPL_POINTER_POSITION,

    /// Size in bytes of the buffers to store all the desktop update metadata for this frame.
    pub total_metadata_buffer_size: UINT,

    /// Size in bytes of the buffer to hold the new pixel data for the mouse shape.
    pub pointer_shape_buffer_size: UINT,
}

impl Default for DXGI_OUTDUPL_FRAME_INFO {
    fn default() -> Self {
        DXGI_OUTDUPL_FRAME_INFO {
            last_present_time: LARGE_INTEGER::default(),
            last_mouse_update_time: LARGE_INTEGER::default(),
            accumulated_frames: 0,
            rects_coalesced: 0,
            protected_content_masked_out: 0,
            pointer_position: DXGI_OUTDUPL_POINTER_POSITION::default(),
            total_metadata_buffer_size: 0,
            pointer_shape_buffer_size: 0,
        }
    }
}
