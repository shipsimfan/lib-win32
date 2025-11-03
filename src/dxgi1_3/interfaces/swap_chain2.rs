use crate::{
    com_interface,
    dxgi::{
        IDXGIDeviceSubObject, IDXGIObject,
        IDXGISwapChain,
    },
    dxgi1_2::{IDXGISwapChain1},
    dxgi1_3::DXGI_MATRIX_3X2_F,
    unknwn::{IUnknown},
    HANDLE, HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::DXGI_SWAP_CHAIN_FLAG, dxgi1_2::IDXGIFactory2, CloseHandle, WaitForSingleObjectEx,
    DXGI_ERROR_DEVICE_REMOVED, DXGI_ERROR_INVALID_CALL, E_INVALIDARG, S_OK,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

com_interface!(
    /// Extends [`IDXGISwapChain1`] with methods to support swap back buffer scaling and
    /// lower-latency swap chains.
    ///
    /// # Remarks
    /// You can create a swap chain by calling [`IDXGIFactory2::create_swap_chain_for_hwnd`],
    /// [`IDXGIFactory2::create_swap_chain_for_core_window`], or
    /// [`IDXGIFactory2::create_swap_chain_for_composition`].
    pub abstract IDXGISwapChain2(IDXGISwapChain2VTable):
        IDXGISwapChain1(swap_chain1) +
        IDXGISwapChain +
        IDXGIDeviceSubObject +
        IDXGIObject +
        IUnknown {
        const IID = 0xA8BE2AC4-0x199F-0x4946-0xB331-0x79599FB98DE7;

        /// Sets the source region to be used for the swap chain.
        ///
        /// Use [`IDXGISwapChain2::set_source_size`] to specify the portion of the swap chain from
        /// which the operating system presents. This allows an effective resize without calling
        /// the more-expensive [`IDXGISwapChain::resize_buffers`] method. Prior to Windows 8.1,
        /// calling [`IDXGISwapChain::resize_buffers`] was the only way to resize the swap chain.
        /// The source rectangle is always defined by the region `[0, 0, width, height]`.
        ///
        /// # Parameters
        ///  * `width` - Source width to use for the swap chain. This value must be greater than
        ///              zero, and must be less than or equal to the overall width of the swap
        ///              chain.
        ///  * `height` - Source height to use for the swap chain. This value must be greater than
        ///               zero, and must be less than or equal to the overall height of the swap
        ///               chain.
        ///
        /// # Return Value
        /// This method can return:
        ///  - [`E_INVALIDARG`] if one or more parameters exceed the size of the back buffer.
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        fn set_source_size(&mut self, width: UINT, height: UINT) -> HRESULT;

        /// Gets the source region used for the swap chain.
        ///
        /// Use [`IDXGISwapChain2::get_source_size`] to get the portion of the swap chain from
        /// which the operating system presents. The source rectangle is always defined by the
        /// region `[0, 0, width, height]`. Use [`IDXGISwapChain2::set_source_size`] to set this
        /// portion of the swap chain.
        ///
        /// # Parameters
        ///  * `width` - The current width of the source region of the swap chain. This value can
        ///              range from 1 to the overall width of the swap chain.
        ///  * `height` - The current height of the source region of the swap chain. This value can
        ///               range from 1 to the overall height of the swap chain.
        ///
        /// # Return Value
        /// This method can return error codes that are described in the `DXGI_ERROR` topic.
        fn get_source_size(&mut self, width: *mut UINT, height: *mut UINT) -> HRESULT;

        /// Sets the number of frames that the swap chain is allowed to queue for rendering.
        ///
        /// # Parameters
        ///  * `max_latency` - The maximum number of back buffer frames that will be queued for the
        ///                    swap chain. This value is 1 by default.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, [`DXGI_ERROR_DEVICE_REMOVED`] if the device
        /// was removed.
        ///
        /// # Remarks
        /// This method is only valid for use on swap chains created with
        /// [`DXGI_SWAP_CHAIN_FLAG::FrameLatencyWaitableObject`]. Otherwise, the result will be
        /// [`DXGI_ERROR_INVALID_CALL`].
        fn set_maximum_frame_latency(&mut self, max_latency: UINT) -> HRESULT;

        /// Gets the number of frames that the swap chain is allowed to queue for rendering.
        ///
        /// # Parameters
        ///  * `max_latency` - The maximum number of back buffer frames that will be queued for the
        ///                    swap chain. This value is 1 by default, but should be set to 2 if
        ///                    the scene takes longer than it takes for one vertical refresh
        ///                    (typically about 16ms) to draw.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the following members of the
        /// `D3DERR` enumerated type:
        ///  - [`D3DERR_DEVICELOST`]
        ///  - [`D3DERR_DEVICEREMOVED`]
        ///  - [`D3DERR_DRIVERINTERNALERROR`]
        ///  - [`D3DERR_INVALIDCALL`]
        ///  - [`D3DERR_OUTOFVIDEOMEMORY`]
        fn get_maximum_frame_latency(&mut self, max_latency: *mut UINT) -> HRESULT;

        /// Returns a waitable handle that signals when the DXGI adapter has finished presenting a
        /// new frame.
        ///
        /// Windows 8.1 introduces new APIs that allow lower-latency rendering by waiting until the
        /// previous frame is presented to the display before drawing the next frame. To use this
        /// method, first create the DXGI swap chain with the
        /// [`DXGI_SWAP_CHAIN_FLAG::FrameLatencyWaitableObject`] flag set, then call
        /// [`IDXGISwapChain2::get_frame_latency_waitable_object`] to retrieve the waitable handle.
        /// Use the waitable handle with [`WaitForSingleObjectEx`] to synchronize rendering of each
        /// new frame with the end of the previous frame. For every frame it renders, the app
        /// should wait on this handle before starting any rendering operations. Note that this
        /// requirement includes the first frame the app renders with the swap chain. When you are
        /// done with the handle, use [`CloseHandle`] to close it.
        ///
        /// # Return Value
        /// A handle to the waitable object, or [`null_mut`] if the swap chain was not created with
        /// [`DXGI_SWAP_CHAIN_FLAG::FrameLatencyWaitableObject`].
        ///
        /// # Remarks
        /// When an application is finished using the object handle returned by
        /// [`IDXGISwapChain2::get_frame_latency_waitable_object`], use the [`CloseHandle`]
        /// function to close the handle.
        fn get_frame_latency_waitable_object(&mut self) -> HANDLE;

        /// Sets the transform matrix that will be applied to a composition swap chain upon the
        /// next present.
        ///
        /// Starting with Windows 8.1, Windows Store apps are able to place DirectX swap chain
        /// visuals in XAML pages using the `SwapChainPanel` element, which can be placed and sized
        /// arbitrarily. This exposes the DirectX swap chain visuals to touch scaling and
        /// translation scenarios using touch UI. The [`IDXGISwapChain2::get_matrix_transform`] and
        /// [`IDXGISwapChain2::set_matrix_transform`] methods are used to synchronize scaling of
        /// the DirectX swap chain with its associated `SwapChainPanel` element. Only simple
        /// scale/translation elements in the matrix are allowed – the call will fail if the matrix
        /// contains skew/rotation elements.
        ///
        /// # Parameters
        ///  * `matrix` - The transform matrix to use for swap chain scaling and translation. This
        ///               function can only be used with composition swap chains created by
        ///               [`IDXGIFactory2::create_swap_chain_for_composition`]. Only scale and
        ///               translation components are allowed in the matrix.
        ///
        /// # Return Value
        /// [`IDXGISwapChain2::set_matrix_transform`] returns:
        ///  - [`S_OK`] if it successfully retrieves the transform matrix.
        ///  - [`E_INVALIDARG`] if the `matrix` parameter is incorrect, for example, `matrix` is
        ///    [`null`] or the matrix represented by [`DXGI_MATRIX_3X2_F`] includes components
        ///    other than scale and translation.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the method is called on a swap chain that was not
        ///    created with [`IDXGIFactory2::create_swap_chain_for_composition`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        fn set_matrix_transform(&mut self, matrix: *const DXGI_MATRIX_3X2_F) -> HRESULT;

        /// Gets the transform matrix that will be applied to a composition swap chain upon the
        /// next present.
        ///
        /// Starting with Windows 8.1, Windows Store apps are able to place DirectX swap chain
        /// visuals in XAML pages using the `SwapChainPanel` element, which can be placed and sized
        /// arbitrarily. This exposes the DirectX swap chain visuals to touch scaling and
        /// translation scenarios using touch UI. The [`IDXGISwapChain2::get_matrix_transform`] and
        /// [`IDXGISwapChain2::set_matrix_transform`] methods are used to synchronize scaling of
        /// the DirectX swap chain with its associated `SwapChainPanel` element. Only simple
        /// scale/translation elements in the matrix are allowed – the call will fail if the matrix
        /// contains skew/rotation elements.
        ///
        /// # Parameters
        ///  * `matrix` - The transform matrix currently used for swap chain scaling and
        ///               translation.
        ///
        /// # Return Value
        /// [`IDXGISwapChain2::get_matrix_transform`] returns:
        ///  - [`S_OK`] if it successfully retrieves the transform matrix.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the method is called on a swap chain that was not
        ///    created with [`IDXGIFactory2::create_swap_chain_for_composition`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        fn get_matrix_transform(&mut self, matrix: *mut DXGI_MATRIX_3X2_F) -> HRESULT;
    }
);
