use crate::{
    com_interface,
    dxgi1_3::DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
    unknwn::{IUnknown},
    HRESULT, RECT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGIResource, DXGI_PRESENT_USE_DURATION},
    DXGI_ERROR_DEVICE_REMOVED, DXGI_ERROR_INVALID_CALL, DXGI_STATUS_OCCLUDED, E_OUTOFMEMORY, S_OK,
};

com_interface!(
    /// Represents a swap chain that is used by desktop media apps to decode video data and show it
    /// on a DirectComposition surface.
    ///
    /// # Remarks
    /// Decode swap chains are intended for use primarily with YUV surface formats. When using
    /// decode buffers created with an RGB surface format, the `target_rect` and `dest_size` must
    /// be set equal to the buffer dimensions. `source_rect` cannot exceed the buffer dimensions.
    ///
    /// In clone mode, the decode swap chain is only guaranteed to be shown on the primary output.
    ///
    /// Decode swap chains cannot be used with dirty rects.
    pub abstract IDXGIDecodeSwapChain(IDXGIDecodeSwapChainVTable):
        IUnknown(unknown) {
        const IID = 0x2633066B-0x4514-0x4C7A-0x8FD8-0x12EA98059D18;

        /// Presents a frame on the output adapter. The frame is a subresource of the
        /// [`IDXGIResource`] object that was used to create the decode swap chain.
        ///
        /// # Parameters
        ///  * `buffer_to_present` - An index indicating which member of the subresource array to
        ///                          present.
        ///  * `sync_interval` - An integer that specifies how to synchronize presentation of a
        ///                      frame with the vertical blank.
        ///  * `flags` - An integer value that contains swap-chain presentation options. These
        ///              options are defined by the `DXGI_PRESENT` constants. The
        ///              [`DXGI_PRESENT_USE_DURATION`] flag must be set if a custom present
        ///              duration (custom refresh rate) is being used.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or it returns one of the following error
        /// codes:
        ///  - [`DXGI_ERROR_DEVICE_REMOVED`]
        ///  - [`DXGI_STATUS_OCCLUDED`]
        ///  - [`DXGI_ERROR_INVALID_CALL`]
        ///  - [`E_OUTOFMEMORY`]
        fn present_buffer(
            &mut self,
            buffer_to_present: UINT,
            sync_interval: UINT,
            flags: UINT
        ) -> HRESULT;

        /// Sets the rectangle that defines the source region for the video processing blit
        /// operation.
        ///
        /// The source rectangle is the portion of the input surface that is blitted to the
        /// destination surface. The source rectangle is given in pixel coordinates, relative to
        /// the input surface.
        ///
        /// # Parameters
        ///  * `rect` - A pointer to a [`RECT`] structure that contains the source region to set
        ///             for the swap chain.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or it returns one of the error codes that are
        /// described in the DXGI_ERROR topic.
        fn set_source_rect(&mut self, rect: *const RECT) -> HRESULT;

        /// Sets the rectangle that defines the target region for the video processing blit
        /// operation.
        ///
        /// The target rectangle is the area within the destination surface where the output will
        /// be drawn. The target rectangle is given in pixel coordinates, relative to the
        /// destination surface.
        ///
        /// # Parameters
        ///  * `rect` - A pointer to a [`RECT`] structure that contains the target region to set
        ///             for the swap chain.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or it returns one of the error codes that are
        /// described in the DXGI_ERROR topic.
        fn set_target_rect(&mut self, rect: *const RECT) -> HRESULT;

        /// Sets the size of the destination surface to use for the video processing blit
        /// operation.
        ///
        /// The destination rectangle is the portion of the output surface that receives the blit
        /// for this stream. The destination rectangle is given in pixel coordinates, relative to
        /// the output surface.
        ///
        /// # Parameters
        ///  * `width` - The width of the destination size, in pixels.
        ///  * `height` - The height of the destination size, in pixels.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or it returns one of the error codes that are
        /// described in the DXGI_ERROR topic.
        fn set_dest_size(&mut self, width: UINT, height: UINT) -> HRESULT;

        /// Gets the source region that is used for the swap chain.
        ///
        /// # Parameters
        ///  * `rect` - A pointer to a [`RECT`] structure that receives the source region for the
        ///             swap chain.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or it returns one of the error codes that are
        /// described in the DXGI_ERROR topic.
        fn get_source_rect(&mut self, rect: *mut RECT) -> HRESULT;

        /// Gets the rectangle that defines the target region for the video processing blit
        /// operation.
        ///
        /// # Parameters
        ///  * `rect` - A pointer to a [`RECT`] structure that receives the target region for the
        ///             swap chain.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or it returns one of the error codes that are
        /// described in the DXGI_ERROR topic.
        fn get_target_rect(&mut self, rect: *mut RECT) -> HRESULT;

        /// Gets the size of the destination surface to use for the video processing blit
        /// operation.
        ///
        /// # Parameters
        ///  * `width` - A pointer to a variable that receives the width in pixels.
        ///  * `height` - A pointer to a variable that receives the height in pixels.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or it returns one of the error codes that are
        /// described in the DXGI_ERROR topic.
        fn get_dest_size(&mut self, width: *mut UINT, height: *mut UINT) -> HRESULT;

        /// Sets the color space used by the swap chain.
        ///
        /// # Parameters
        ///  * `color_space` - A pointer to a combination of
        ///                    [`DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS`]-typed values that are
        ///                    combined by using a bitwise OR operation. The resulting value
        ///                    specifies the color space to set for the swap chain.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or it returns one of the error codes that are
        /// described in the DXGI_ERROR topic.
        fn set_color_space(&mut self, color_space: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> HRESULT;

        /// Gets the color space used by the swap chain.
        ///
        /// # Return Value
        /// A combination of [`DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS`]-typed values that are combined
        /// by using a bitwise OR operation. The resulting value specifies the color space for the
        /// swap chain.
        fn get_color_space(&mut self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS;
    }
);
