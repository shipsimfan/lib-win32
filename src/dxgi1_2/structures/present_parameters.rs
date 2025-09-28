use crate::{POINT, RECT, UINT};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi::DXGI_SWAP_EFFECT, dxgi1_2::IDXGISwapChain1};

/// Describes information about present that helps the operating system optimize presentation.
///
/// # Remarks
/// This structure is used by the [`IDXGISwapChain1::present1`] method.
///
/// The scroll rectangle and the list of dirty rectangles could overlap. In this situation, the
/// dirty rectangles take priority. Applications can then have pieces of dynamic content on top of
/// a scrolled area. For example, an application could scroll a page and play video at the same
/// time.
///
/// Parts of the previous frame and content that the application renders are combined to produce
/// the final frame that the operating system presents on the display screen. Most of the window is
/// scrolled from the previous frame. The application must update the video frame with the new
/// chunk of content that appears due to scrolling.
///
/// The dashed rectangle shows the scroll rectangle in the current frame. The scroll rectangle is
/// specified by the `scroll_rect` member. The arrow shows the scroll offset. The scroll offset is
/// specified by the `scroll_offset` member. Filled rectangles show dirty rectangles that the
/// application updated with new content. The filled rectangles are specified by the
/// `dirty_rects_count` and `dirty_rects` members.
///
/// The scroll rectangle and offset are not supported for the [`DXGI_SWAP_EFFECT::Discard`] or
/// [`DXGI_SWAP_EFFECT::Sequential`] present option. Dirty rectangles and scroll rectangle are not
/// supported for multisampled swap chains.
///
/// The actual implementation of composition and necessary bitblts is different for the bitblt
/// model and the flip model.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_PRESENT_PARAMETERS {
    /// The number of updated rectangles that you update in the back buffer for the presented
    /// frame. The operating system uses this information to optimize presentation. You can set
    /// this member to 0 to indicate that you update the whole frame.
    pub dirty_rects_count: UINT,

    /// A list of updated rectangles that you update in the back buffer for the presented frame. An
    /// application must update every single pixel in each rectangle that it reports to the
    /// runtime; the application cannot assume that the pixels are saved from the previous frame.
    /// You can set this member to [`null_mut`] if `dirty_rects_count` is 0. An application must
    /// not update any pixel outside of the dirty rectangles.
    pub dirty_rects: *mut RECT,

    /// A pointer to the scrolled rectangle. The scrolled rectangle is the rectangle of the
    /// previous frame from which the runtime bit-block transfers (bitblts) content. The runtime
    /// also uses the scrolled rectangle to optimize presentation in terminal server and indirect
    /// display scenarios.
    ///
    /// The scrolled rectangle also describes the destination rectangle, that is, the region on the
    /// current frame that is filled with scrolled content. You can set this member to [`null_mut`]
    /// to indicate that no content is scrolled from the previous frame.
    pub scroll_rect: *mut RECT,

    /// A pointer to the offset of the scrolled area that goes from the source rectangle (of
    /// previous frame) to the destination rectangle (of current frame). You can set this member to
    /// [`null_mut`] to indicate no offset.
    pub scroll_offset: *mut POINT,
}

impl Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        DXGI_PRESENT_PARAMETERS {
            dirty_rects_count: 0,
            dirty_rects: null_mut(),
            scroll_rect: null_mut(),
            scroll_offset: null_mut(),
        }
    }
}
