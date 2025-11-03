use crate::{
    com_interface,
    dxgi::{
        IDXGIDeviceSubObject, IDXGIObject,
        IDXGIOutput, DXGI_FORMAT, DXGI_FRAME_STATISTICS, DXGI_MODE_DESC, DXGI_SWAP_CHAIN_DESC,
    },
    unknwn::{IUnknown},
    BOOL, HRESULT, REFIID, UINT,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11")]
use crate::d3d11::{D3D11CreateDeviceAndSwapChain, ID3D11DeviceContext};
#[allow(unused_imports)]
#[cfg(feature = "dxgi1_2")]
use crate::dxgi1_2::{IDXGIFactory2, IDXGISwapChain1, DXGI_SWAP_CHAIN_DESC1};
#[allow(unused_imports)]
use crate::{
    dxgi::{
        IDXGIAdapter, IDXGIFactory, IDXGIResource, IDXGISurface1, DXGI_PRESENT_DO_NOT_SEQUENCE,
        DXGI_SWAP_CHAIN_FLAG, DXGI_SWAP_EFFECT, DXGI_USAGE,
    },
    SetWindowPos, DXGI_ERROR_DEVICE_REMOVED, DXGI_ERROR_DEVICE_RESET,
    DXGI_ERROR_NOT_CURRENTLY_AVAILABLE, DXGI_STATUS_MODE_CHANGE_IN_PROGRESS, DXGI_STATUS_OCCLUDED,
    FALSE, S_OK, TRUE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// An [`IDXGISwapChain`] interface implements one or more surfaces for storing rendered data
    /// before presenting it to an output.
    ///
    /// # Remarks
    /// You can create a swap chain by calling [`IDXGIFactory2::create_swap_chain_for_hwnd`],
    /// [`IDXGIFactory2::create_swap_chain_for_core_window`], or
    /// [`IDXGIFactory2::create_swap_chain_for_composition`]. You can also create a swap chain when
    /// you call [`D3D11CreateDeviceAndSwapChain`]; however, you can then only access the sub-set
    /// of swap-chain functionality that the [`IDXGISwapChain`] interface provides.
    pub abstract IDXGISwapChain(IDXGISwapChainVTable):
        IDXGIDeviceSubObject(device_sub_object) +
        IDXGIObject +
        IUnknown {
        const IID = 0x310D36A0-0xD2E7-0x4C0A-0xAA04-0x6A9D23B8886A;

        /// Presents a rendered image to the user.
        ///
        /// # Parameters
        ///  * `sync_interval` - An integer that specifies how to synchronize presentation of a
        ///                      frame with the vertical blank.
        ///  * `flags` - An integer value that contains swap-chain presentation options. These
        ///              options are defined by the `DXGI_PRESENT` constants.
        ///
        /// # Return Value
        /// Possible return values include: [`S_OK`], [`DXGI_ERROR_DEVICE_RESET`] or
        /// [`DXGI_ERROR_DEVICE_REMOVED`], [`DXGI_STATUS_OCCLUDED`], or
        /// [`D3DDDIERR_DEVICEREMOVED`].
        ///
        /// # Remarks
        /// Starting with Direct3D 11.1, consider using [`IDXGISwapChain1::present1`] because you
        /// can then use dirty rectangles and the scroll rectangle in the swap chain presentation
        /// and as such use less memory bandwidth and as a result less system power.
        ///
        /// Because calling [`IDXGISwapChain::present`] might cause the render thread to wait on
        /// the message-pump thread, be careful when calling this method in an application that
        /// uses multiple threads.
        ///
        /// For flip presentation model swap chains that you create with the
        /// [`DXGI_SWAP_EFFECT_FLIP::Sequential`] or [`DXGI_SWAP_EFFECT::Discard`] value set, a
        /// successful presentation unbinds back buffer 0 (usually set by
        /// [`om_set_render_targets`]) from the graphics pipeline, except for when you pass the
        /// [`DXGI_PRESENT_DO_NOT_SEQUENCE`] flag in the `flags` parameter.
        fn present(&mut self, sync_interval: UINT, flags: UINT) -> HRESULT;

        /// Accesses one of the swap-chain's back buffers.
        ///
        /// # Parameters
        ///  * `buffer` - A zero-based buffer index. If the swap chain's swap effect is
        ///               [`DXGI_SWAP_EFFECT::Discard`], this method can only access the first
        ///               buffer; for this situation, set the index to zero. If the swap chain's
        ///               swap effect is either [`DXGI_SWAP_EFFECT::Sequential`] or
        ///               [`DXGI_SWAP_EFFECT::FlipSequential`], only the swap chain's zero-index
        ///               buffer can be read from and written to. The swap chain's buffers with
        ///               indexes greater than zero can only be read from; so if you call the
        ///               [`IDXGIResource::get_usage`] method for such buffers, they have the
        ///               [`DXGI_USAGE::ReadOnly`] flag set.
        ///  * `iid` - The type of interface used to manipulate the buffer.
        ///  * `surface` - A pointer to a back-buffer interface.
        ///
        /// # Return Value
        /// Returns one of the following `DXGI_ERROR`.
        fn get_buffer(&mut self, buffer: UINT, iid: REFIID, surface: *mut *mut c_void) -> HRESULT;

        /// Sets the display state to windowed or full screen.
        ///
        /// # Parameters
        ///  * `fullscreen` - A Boolean value that specifies whether to set the display state to
        ///                   windowed or full screen. [`TRUE`] for full screen, and [`FALSE`] for
        ///                   windowed.
        ///  * `target` - If you pass [`TRUE`] to the `fullscreen` parameter to set the display
        ///               state to full screen, you can optionally set this parameter to a pointer
        ///               to an [`IDXGIOutput`] interface for the output target that contains the
        ///               swap chain. If you set this parameter to [`null_mut`], DXGI will choose
        ///               the output based on the swap-chain's device and the output window's
        ///               placement. If you pass [`FALSE`] to Fullscreen, then you must set this
        ///               parameter to [`null_mut`].
        ///
        /// # Return Value
        /// This method returns one of these values.
        ///  - [`S_OK`] if the action succeeded and the swap chain was placed in the requested
        ///    state.
        ///  - [`DXGI_ERROR_NOT_CURRENTLY_AVAILABLE`] if the action failed. When this error is
        ///    returned, your application can continue to run in windowed mode and try to switch to
        ///    full-screen mode later. There are many reasons why a windowed-mode swap chain cannot
        ///    switch to full-screen mode. Here are some examples:
        ///    - The application is running over Terminal Server.
        ///    - The output window is occluded.
        ///    - The output window does not have keyboard focus.
        ///    - Another application is already in full-screen mode.
        ///  - [`DXGI_STATUS_MODE_CHANGE_IN_PROGRESS`] is returned if a fullscreen/windowed mode
        ///    transition is occurring when this API is called.
        ///  - Other error codes if you run out of memory or encounter another unexpected fault;
        ///    these codes may be treated as hard, non-continuable errors.
        ///
        /// # Remarks
        /// DXGI may change the display state of a swap chain in response to end user or system
        /// requests.
        ///
        /// We recommend that you create a windowed swap chain and allow the end user to change the
        /// swap chain to full screen through [`IDXGISwapChain::set_fullscreen_state`]; that is, do
        /// not set the Windowed member of [`DXGI_SWAP_CHAIN_DESC`] to [`FALSE`] to force the swap
        /// chain to be full screen. However, if you create the swap chain as full screen, also
        /// provide the end user with a list of supported display modes because a swap chain that
        /// is created with an unsupported display mode might cause the display to go black and
        /// prevent the end user from seeing anything. Also, we recommend that you have a time-out
        /// confirmation screen or other fallback mechanism when you allow the end user to change
        /// display modes.
        fn set_fullscreen_state(&mut self, fullscreen: BOOL, target: *mut IDXGIOutput) -> HRESULT;

        /// Get the state associated with full-screen mode.
        ///
        /// # Parameters
        ///  * `fullscreen` - A pointer to a boolean whose value is either:
        ///    - [`TRUE`] if the swap chain is in full-screen mode
        ///    - [`FALSE`] if the swap chain is in windowed mode
        ///  * `target` - A pointer to the output target (see [`IDXGIOutput`]) when the mode is
        ///               full screen; otherwise [`null_mut`].
        ///
        /// # Return Value
        /// Returns one of the following `DXGI_ERROR`.
        ///
        /// # Remarks
        /// When the swap chain is in full-screen mode, a pointer to the target output will be
        /// returned and its reference count will be incremented.
        fn get_fullscreen_state(
            &mut self,
            fullscreen: BOOL,
            target: *mut *mut IDXGIOutput
        ) -> HRESULT;

        /// Get a description of the swap chain.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to the swap-chain description (see [`DXGI_SWAP_CHAIN_DESC`]).
        ///
        /// # Return Value
        /// Returns one of the following `DXGI_ERROR`.
        fn get_desc(&mut self, desc: *mut DXGI_SWAP_CHAIN_DESC) -> HRESULT;

        /// Changes the swap chain's back buffer size, format, and number of buffers. This should
        /// be called when the application window is resized.
        ///
        /// # Parameters
        ///  * `buffer_count` - The number of buffers in the swap chain (including all back and
        ///                     front buffers). This number can be different from the number of
        ///                     buffers with which you created the swap chain. This number can't be
        ///                     greater than [`DXGI_MAX_SWAP_CHAIN_BUFFERS`]. Set this number to
        ///                     zero to preserve the existing number of buffers in the swap chain.
        ///                     You can't specify less than two buffers for the flip presentation
        ///                     model.
        ///  * `width` - The new width of the back buffer. If you specify zero, DXGI will use the
        ///              width of the client area of the target window. You can't specify the width
        ///              as zero if you called the
        ///              [`IDXGIFactory2::create_swap_chain_for_composition`] method to create the
        ///              swap chain for a composition surface.
        ///  * `height` - The new height of the back buffer. If you specify zero, DXGI will use the
        ///               height of the client area of the target window. You can't specify the
        ///               height as zero if you called the
        ///               [`IDXGIFactory2::create_swap_chain_for_composition`] method to create the
        ///               swap chain for a composition surface.
        ///  * `new_format` - A [`DXGI_FORMAT`]-typed value for the new format of the back buffer.
        ///                   Set this value to [`DXGI_FORMAT::Unknown`] to preserve the existing
        ///                   format of the back buffer. The flip presentation model supports a
        ///                   more restricted set of formats than the bit-block transfer (bitblt)
        ///                   model.
        ///  * `swap_chain_flags` - A combination of [`DXGI_SWAP_CHAIN_FLAG`]-typed values that are
        ///                         combined by using a bitwise OR operation. The resulting value
        ///                         specifies options for swap-chain behavior.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// # Remarks
        /// You can't resize a swap chain unless you release all outstanding references to its back
        /// buffers. You must release all of its direct and indirect references on the back buffers
        /// in order for [`IDXGISwapChain::resize_buffers`] to succeed.
        ///
        /// Direct references are held by the application after it calls [`IUnknown::add_ref`] on a
        /// resource.
        ///
        /// Indirect references are held by views to a resource, binding a view of the resource to
        /// a device context, a command list that used the resource, a command list that used a
        /// view to that resource, a command list that executed another command list that used the
        /// resource, and so on.
        ///
        /// Before you call [`IDXGISwapChain::resize_buffers`], ensure that the application
        /// releases all references (by calling the appropriate number of [`IUnknown::release`]
        /// invocations) on the resources, any views to the resource, and any command lists that
        /// use either the resources or views, and ensure that neither the resource nor a view is
        /// still bound to a device context. You can use [`ID3D11DeviceContext::clear_state`] to
        /// ensure that all references are released. If a view is bound to a deferred context, you
        /// must discard the partially built command list as well (by calling
        /// [`ID3D11DeviceContext::clear_state`], then
        /// [`ID3D11DeviceContext::finish_command_list`], then [`IUnknown::release`] on the command
        /// list). After you call [`IDXGISwapChain::resize_buffers`], you can re-query interfaces
        /// via [`IDXGISwapChain::get_buffer`].
        ///
        /// For swap chains that you created with [`DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE`], before
        /// you call [`IDXGISwapChain::resize_buffers`], also call [`IDXGISurface1::release_dc`] on
        /// the swap chain's back-buffer surface to ensure that you have no outstanding GDI device
        /// contexts (DCs) open.
        ///
        /// We recommend that you call [`IDXGISwapChain::resize_buffers`] when a client window is
        /// resized (that is, when an application receives a [`WM_SIZE`] message).
        ///
        /// The only difference between IDXGISwapChain::[`IDXGISwapChain::resize_buffers`] in
        /// Windows 8 versus Windows 7 is with flip presentation model swap chains that you create
        /// with the [`DXGI_SWAP_EFFECT::FlipSequential`] or [`DXGI_SWAP_EFFECT::FlipDiscard`]
        /// value set. In Windows 8, you must call [`IDXGISwapChain::resize_buffers`] to realize a
        /// transition between full-screen mode and windowed mode; otherwise, your next call to the
        /// [`IDXGISwapChain::present`] method fails.
        fn resize_buffers(
            &mut self,
            buffer_count: UINT,
            width: UINT,
            height: UINT,
            new_format: DXGI_FORMAT,
            swap_chain_flags: UINT
        ) -> HRESULT;

        /// Resizes the output target.
        ///
        /// # Parameters
        ///  * `new_target_parameters` - A pointer to a [`DXGI_MODE_DESC`] structure that describes
        ///                              the mode, which specifies the new width, height, format,
        ///                              and refresh rate of the target. If the format is
        ///                              [`DXGI_FORMAT::Unknown`],
        ///                              [`IDXGISwapChain::resize_target`] uses the existing
        ///                              format. We only recommend that you use
        ///                              [`DXGI_FORMAT::Unknown`] when the swap chain is in
        ///                              full-screen mode as this method is not thread safe.
        ///
        /// # Return Value
        /// Returns a code that indicates success or failure.
        /// [`DXGI_STATUS_MODE_CHANGE_IN_PROGRESS`] is returned if a full-screen/windowed mode
        /// transition is occurring when this API is called.
        ///
        /// # Remarks
        /// [`IDXGISwapChain::resize_target`] resizes the target window when the swap chain is in
        /// windowed mode, and changes the display mode on the target output when the swap chain is
        /// in full-screen mode. Therefore, apps can call [`IDXGISwapChain::resize_target`] to
        /// resize the target window (rather than a Microsoft Win32API such as [`SetWindowPos`])
        /// without knowledge of the swap chain display mode.
        ///
        /// If a Windows Store app calls [`IDXGISwapChain::resize_target`], it fails with
        /// [`DXGI_ERROR_NOT_CURRENTLY_AVAILABLE`].
        ///
        /// You cannot call [`IDXGISwapChain::resize_target`] on a swap chain that you created with
        /// [`IDXGIFactory2::create_swap_chain_for_composition`].
        ///
        /// Apps must still call [`IDXGISwapChain::resize_buffers`] after they call
        /// [`IDXGISwapChain::resize_target`] because only [`IDXGISwapChian::resize_buffers`] can
        /// change the back buffers. But, if those apps have implemented window resize processing
        /// to call [`IDXGISwapChain::resize_buffers`], they don't need to explicitly call
        /// [`IDXGISwapChain::resize_buffers`] after they call [`IDXGISwapChain::resize_target`]
        /// because the window resize processing will achieve what the app requires.
        fn resize_target(&mut self, new_target_parameters: *mut DXGI_MODE_DESC) -> HRESULT;

        /// Get the output (the display monitor) that contains the majority of the client area of
        /// the target window.
        ///
        /// # Parameters
        ///  * `output` - A pointer to the output interface (see [`IDXGIOutput`]).
        ///
        /// # Return Value
        /// Returns one of the following `DXGI_ERROR`.
        ///
        /// # Remarks
        /// If the method succeeds, the output interface will be filled and its reference count
        /// incremented. When you are finished with it, be sure to release the interface to avoid a
        /// memory leak.
        ///
        /// The output is also owned by the adapter on which the swap chain's device was created.
        ///
        /// You cannot call [`IDXGISwapChain::get_containing_output`] on a swap chain that you
        /// created with [`IDXGIFactory2::create_swap_chain_for_composition`].
        ///
        /// To determine the output corresponding to such a swap chain, you should call
        /// [`IDXGIFactory::enum_adapters`] and then [`IDXGIAdapter::enum_outputs`] to enumerate
        /// over all of the available outputs. You should then intersect the bounds of your
        /// [`CoreWindow::Bounds`] with the desktop coordinates of each output, as reported by
        /// [`DXGI_OUTPUT_DESC1::desktop_coordinates`] or
        /// [`DXGI_OUTPUT_DESC::desktop_coordinates`].
        fn get_containing_output(&mut self, output: *mut *mut IDXGIOutput) -> HRESULT;

        /// Gets performance statistics about the last render frame.
        ///
        /// # Parameters
        ///  * `stats` - A pointer to a [`DXGI_FRAME_STATISTICS`] structure for the frame
        ///              statistics.
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        ///
        /// # Remarks
        /// You cannot use [`IDXGISwapChain::get_frame_statistics`] for swap chains that both use
        /// the bit-block transfer (bitblt) presentation model and draw in windowed mode.
        ///
        /// You can only use [`IDXGISwapChain::get_frame_statistics`] for swap chains that either
        /// use the flip presentation model or draw in full-screen mode. You set the
        /// [`DXGI_SWAP_EFFECT::FlipSequential`] value in the SwapEffect member of the
        /// [`DXGI_SWAP_CHAIN_DESC1`] structure to specify that the swap chain uses the flip
        /// presentation model.
        ///
        /// Statistics are not reliable in many multiple monitor scenarios, as well as scenarios
        /// where other fullscreen apps are running.
        ///
        /// It may be necessary to call [`DwmFlush`] before frame statistics are updated on systems
        /// that support Hardware Flip Queue.
        fn get_frame_statistics(&mut self, stats: *mut DXGI_FRAME_STATISTICS) -> HRESULT;

        /// Gets the number of times that [`IDXGISwapChain::present`] or
        /// [`IDXGISwapChain1::present1`] has been called.
        ///
        /// # Parameters
        ///  * `last_present_count` - A pointer to a variable that receives the number of calls.
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        ///
        /// # Remarks
        /// For info about presentation statistics for a frame, see [`DXGI_FRAME_STATISTICS`].
        fn get_last_present_count(&mut self, last_present_count: *mut UINT) -> HRESULT;
    }
);
