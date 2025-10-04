use crate::{
    com_interface,
    dxgi::{
        IDXGIFactory, IDXGIFactory1, IDXGIFactory1Trait, IDXGIFactoryTrait, IDXGIObject,
        IDXGIObjectTrait, IDXGIOutput,
    },
    dxgi1_2::{IDXGISwapChain1, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FULLSCREEN_DESC},
    unknwn::{IUnknown, IUnknownTrait},
    BOOL, DWORD, HANDLE, HRESULT, HWND, LUID, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11_1::ID3D11Device1,
    d3dcommon::D3D_DRIVER_TYPE,
    dxgi::{
        CreateDXGIFactory, CreateDXGIFactory1, IDXGIDevice, IDXGIDevice1, IDXGISwapChain,
        DXGI_PRESENT_RESTRICT_TO_OUTPUT, DXGI_SWAP_EFFECT,
    },
    dxgi1_2::{IDXGIDevice2, IDXGIResource1, DXGI_SCALING},
    CreateEvent, DXGI_ERROR_INVALID_CALL, E_OUTOFMEMORY, FALSE, S_OK, TRUE,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

com_interface!(
    /// The [`IDXGIFactory2`] interface includes methods to create a newer version swap chain with
    /// more features than [`IDXGISwapChain`] and to monitor stereoscopic 3D capabilities.
    ///
    /// # Remarks
    /// To create a Microsoft DirectX Graphics Infrastructure (DXGI) 1.2 factory interface, pass
    /// [`IDXGIFactory2`] into either the [`CreateDXGIFactory`] or [`CreateDXGIFactory1`] function
    /// or call [`IUnknown::query_interface`] from a factory object that either
    /// [`CreateDXGIFactory`] or [`CreateDXGIFactory1`] returns.
    ///
    /// Because you can create a Direct3D device without creating a swap chain, you might need to
    /// retrieve the factory that is used to create the device in order to create a swap chain. You
    /// can request the [`IDXGIDevice`], [`IDXGIDevice1`], or [`IDXGIDevice2`] interface from the
    /// Direct3D device and then use the [`IDXGIObject::get_parent`] method to locate the factory.
    pub abstract IDXGIFactory2(IDXGIFactory2VTable/IDXGIFactory2Trait):
        IDXGIFactory1/IDXGIFactory1Trait(factory1) +
        IDXGIFactory/IDXGIFactoryTrait(factory1.factory) +
        IDXGIObject/IDXGIObjectTrait(factory1.factory.object) +
        IUnknown/IUnknownTrait(factory1.factory.object.unknown) {
        const IID = 0x50C83A1C-0xE072-0x4C48-0x87B0-0x3630FA36A6D0;

        /// Determines whether to use stereo mode.
        ///
        /// # Return Value
        /// Indicates whether to use stereo mode. [`TRUE`] indicates that you can use stereo mode;
        /// otherwise, [`FALSE`].
        ///
        /// # Remarks
        /// We recommend that windowed applications call
        /// [`IDXGIFactory2::is_windowed_stereo_enabled`] before they attempt to use stereo.
        /// [`IDXGIFactory2::is_windowed_stereo_enabled`] returns [`TRUE`] if both of the following
        /// items are true:
        ///  - All adapters in the computer have drivers that are capable of stereo. This only
        ///    means that the driver is implemented to the Windows Display Driver Model (WDDM) for
        ///    Windows 8 (WDDM 1.2). However, the adapter does not necessarily have to be able to
        ///    scan out stereo.
        ///  - The current desktop mode (desktop modes are mono) and system policy and hardware are
        ///    configured so that the Desktop Window Manager (DWM) performs stereo composition on
        ///    at least one adapter output.
        ///
        /// The creation of a windowed stereo swap chain succeeds if the first requirement is met.
        /// However, if the adapter can't scan out stereo, the output on that adapter is reduced to
        /// mono.
        fn is_windowed_stereo_enabled(&mut self) -> BOOL;

        /// Creates a swap chain that is associated with an [`HWND`] handle to the output window
        /// for the swap chain.
        ///
        /// # Parameters
        ///  * `device` - For Direct3D 11, and earlier versions of Direct3D, this is a pointer to
        ///               the Direct3D device for the swap chain. For Direct3D 12 this is a pointer
        ///               to a direct command queue (refer to [`ID3D12CommandQueue`]). This
        ///               parameter cannot be [`null_mut`].
        ///  * `wnd` - The [`HWND`] handle that is associated with the swap chain that
        ///            [`IDXGIFactory2::create_swap_chain_for_hwnd`] creates. This parameter cannot
        ///            be [`null_mut`].
        ///  * `desc` - A pointer to a [`DXGI_SWAP_CHAIN_DESC1`] structure for the swap-chain
        ///             description. This parameter cannot be [`null_mut`].
        ///  * `fullscreen_desc` - A pointer to a [`DXGI_SWAP_CHAIN_FULLSCREEN_DESC`] structure for
        ///                        the description of a full-screen swap chain. You can optionally
        ///                        set this parameter to create a full-screen swap chain. Set it to
        ///                        [`null`] to create a windowed swap chain.
        ///  * `restrict_to_output` - A pointer to the [`IDXGIOutput`] interface for the output to
        ///                           restrict content to. You must also pass the
        ///                           [`DXGI_PRESENT_RESTRICT_TO_OUTPUT`] flag in a
        ///                           [`IDXGISwapChain1::present1`] call to force the content to
        ///                           appear blacked out on any other output. If you want to
        ///                           restrict the content to a different output, you must create a
        ///                           new swap chain. However, you can conditionally restrict
        ///                           content based on the [`DXGI_PRESENT_RESTRICT_TO_OUTPUT`]
        ///                           flag. Set this parameter to [`null_mut`] if you don't want to
        ///                           restrict content to an output target.
        ///  * `swap_chain` - A pointer to a variable that receives a pointer to the
        ///                   [`IDXGISwapChain1`] interface for the swap chain that
        ///                   [`IDXGIFactory2::create_swap_chain_for_hwnd`] creates.
        ///
        /// # Return Value
        /// [`IDXGIFactory2::create_swap_chain_for_hwnd`] returns:
        ///  - [`S_OK`] if it successfully created a swap chain.
        ///  - [`E_OUTOFMEMORY`] if memory is unavailable to complete the operation.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the calling application provided invalid data, for
        ///    example, if `desc` or `swap_chain` is [`null`], or `desc` data members are invalid.
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic that are
        ///    defined by the type of device that you pass to `device`.
        ///
        /// # Remarks
        /// If you specify the width, height, or both (`width` and `height` members of
        /// [`DXGI_SWAP_CHAIN_DESC1`] that `desc` points to) of the swap chain as zero, the runtime
        /// obtains the size from the output window that the `wnd` parameter specifies.
        ///
        /// You can subsequently call the [`IDXGISwapChain1::get_desc1`] method to retrieve the
        /// assigned width or height value.
        ///
        /// Because you can associate only one flip presentation model swap chain at a time with an
        /// [`HWND`], the Microsoft Direct3D 11 policy of deferring the destruction of objects can
        /// cause problems if you attempt to destroy a flip presentation model swap chain and
        /// replace it with another swap chain.
        fn create_swap_chain_for_hwnd(
            &mut self,
            device: *mut IUnknown,
            wnd: HWND,
            desc: *const DXGI_SWAP_CHAIN_DESC1,
            fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
            restrict_to_output: *mut IDXGIOutput,
            swap_chain: *mut *mut IDXGISwapChain1
        ) -> HRESULT;

        /// Creates a swap chain that is associated with the CoreWindow object for the output
        /// window for the swap chain.
        ///
        /// # Parameters
        ///  * `device` - For Direct3D 11, and earlier versions of Direct3D, this is a pointer to
        ///               the Direct3D device for the swap chain. For Direct3D 12 this is a pointer
        ///               to a direct command queue (refer to [`ID3D12CommandQueue`]). This
        ///               parameter cannot be [`null_mut`].
        ///  * `window` - A pointer to the CoreWindow object that is associated with the swap chain
        ///               that [`IDXGIFactory2::create_swap_chain_for_core_window`] creates.
        ///  * `desc` - A pointer to a [`DXGI_SWAP_CHAIN_DESC1`] structure for the swap-chain
        ///             description. This parameter cannot be [`null`].
        ///  * `restrict_to_output` - A pointer to the [`IDXGIOutput`] interface that the swap
        ///                           chain is restricted to. If the swap chain is moved to a
        ///                           different output, the content is black. You can optionally
        ///                           set this parameter to an output target that uses
        ///                           [`DXGI_PRESENT_RESTRICT_TO_OUTPUT`] to restrict the content
        ///                           on this output. If you do not set this parameter to restrict
        ///                           content on an output target, you can set it to [`nul_mut`].
        ///  * `swap_chain` - A pointer to a variable that receives a pointer to the
        ///                   [`IDXGISwapChain1`] interface for the swap chain that
        ///                   [`IDXGIFactory2::create_swap_chain_for_core_window`] creates.
        ///
        /// # Return Value
        /// [`IDXGIFactory2::create_swap_chain_for_core_window`] returns:
        ///  - [`S_OK`] if it successfully created a swap chain.
        ///  - [`E_OUTOFMEMORY`] if memory is unavailable to complete the operation.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the calling application provided invalid data, for
        ///    example, if `desc` or `swap_chain` is [`null`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic that are
        ///    defined by the type of device that you pass to `device`.
        ///
        /// # Remarks
        /// If you specify the width, height, or both (`width` and `height` members of
        /// [`DXGI_SWAP_CHAIN_DESC1`] that `desc` points to) of the swap chain as zero, the runtime
        /// obtains the size from the output window that the pWindow parameter specifies.
        ///
        /// You can subsequently call the [`IDXGISwapChain1::get_desc1`] method to retrieve the
        /// assigned width or height value.
        ///
        /// Because you can associate only one flip presentation model swap chain (per layer) at a
        /// time with a CoreWindow, the Microsoft Direct3D 11 policy of deferring the destruction
        /// of objects can cause problems if you attempt to destroy a flip presentation model swap
        /// chain and replace it with another swap chain.
        fn create_swap_chain_for_core_window(
            &mut self,
            device: *mut IUnknown,
            window: *mut IUnknown,
            desc: *const DXGI_SWAP_CHAIN_DESC1,
            restrict_to_output: *mut IDXGIOutput,
            swap_chain: *mut *mut IDXGISwapChain1
        ) -> HRESULT;

        /// Identifies the adapter on which a shared resource object was created.
        ///
        /// # Parameters
        ///  * `resource` - A handle to a shared resource object. The
        ///                 [`IDXGIResource1::CreateSharedHandle`] method returns this handle.
        ///  * `luid` - A pointer to a variable that receives a locally unique identifier
        ///             ([`LUID`]) value that identifies the adapter. An [`LUID`] is a 64-bit value
        ///             that is guaranteed to be unique only on the operating system on which it
        ///             was generated. The uniqueness of an [`LUID`] is guaranteed only until the
        ///             operating system is restarted.
        ///
        /// # Return Value
        /// [`IDXGIFactory2::get_shared_resource_adapter_luid`] returns:
        ///  - [`S_OK`] if it identified the adapter.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if `resource` is invalid.
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// You cannot share resources across adapters. Therefore, you cannot open a shared
        /// resource on an adapter other than the adapter on which the resource was created. Call
        /// [`IDXGIFactory2::get_shared_resource_adapter_luid`] before you open a shared resource
        /// to ensure that the resource was created on the appropriate adapter. To open a shared
        /// resource, call the [`ID3D11Device1::open_shared_resource1`] or
        /// [`ID3D11Device1::open_shared_resource_by_name`] method.
        fn get_shared_resource_adapter_luid(
            &mut self,
            resource: HANDLE,
            luid: *mut LUID
        ) -> HRESULT;

        /// Registers an application window to receive notification messages of changes of stereo
        /// status.
        ///
        /// # Parameters
        ///  * `window_handle` - The handle of the window to send a notification message to when
        ///                      stereo status change occurs.
        ///  * `msg` - Identifies the notification message to send.
        ///  * `cookie` - A pointer to a key value that an application can pass to the
        ///               [`IDXGIFactory2::unregister_stereo_status`] method to unregister the
        ///               notification message that wMsg specifies.
        ///
        /// # Return Value
        /// [`IDXGIFactory2::register_stereo_status_window`] returns:
        ///  - [`S_OK`] if it successfully registered the window.
        ///  - [`E_OUTOFMEMORY`] if memory is unavailable to complete the operation.
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        fn register_stereo_status_window(
            &mut self,
            window_handle: HWND,
            msg: UINT,
            cookie: *mut DWORD
        ) -> HRESULT;

        /// Registers to receive notification of changes in stereo status by using event signaling.
        ///
        /// # Parameters
        ///  * `event` - A handle to the event object that the operating system sets when
        ///              notification of stereo status change occurs. The [`CreateEvent`] or
        ///              [`OpenEvent`] function returns this handle.
        ///  * `cookie` - A pointer to a key value that an application can pass to the
        ///               [`IDXGIFactory2::unregister_stereo_status`] method to unregister the
        ///               notification event that `event` specifies.
        ///
        /// # Return Value
        /// [`IDXGIFactory2::register_stereo_status_event`] returns:
        ///  - [`S_OK`] if it successfully registered the event.
        ///  - [`E_OUTOFMEMORY`] if memory is unavailable to complete the operation.
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        fn register_stereo_status_event(&mut self, event: HANDLE, cookie: *mut DWORD) -> HRESULT;

        /// Unregisters a window or an event to stop it from receiving notification when stereo
        /// status changes.
        ///
        /// # Parameters
        ///  * `cookie` - A key value for the window or event to unregister. The
        /// [`IDXGIFactory2::register_stereo_status_window`] or
        /// [`IDXGIFactory2::register_stereo_status_event`] method returns this value.
        fn unregister_stereo_status(&mut self, cookie: DWORD);

        /// Registers an application window to receive notification messages of changes of
        /// occlusion status.
        ///
        /// # Parameters
        ///  * `window_handle` - The handle of the window to send a notification message to when
        ///                      occlusion status change occurs.
        ///  * `msg` - Identifies the notification message to send.
        ///  * `cookie` - A pointer to a key value that an application can pass to the
        ///               [`IDXGIFactory2::unregister_occlusion_status`] method to unregister the
        ///               notification message that wMsg specifies.
        ///
        /// # Return Value
        /// [`IDXGIFactory2::register_occlusion_status_window`] returns:
        ///  - [`S_OK`] if it successfully registered the window.
        ///  - [`E_OUTOFMEMORY`] if memory is unavailable to complete the operation.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if `window_handle` is not a valid window handle or not
        ///    the window handle that the current process owns.
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// Apps choose the Windows message that Windows sends when occlusion status changes.
        fn register_occlusion_status_window(
            &mut self,
            window_handle: HWND,
            msg: UINT,
            cookie: *mut DWORD
        ) -> HRESULT;

        /// Registers to receive notification of changes in occlusion status by using event
        /// signaling.
        ///
        /// # Parameters
        ///  * `event` - A handle to the event object that the operating system sets when
        ///              notification of occlusion status change occurs. The [`CreateEvent`] or
        ///              [`OpenEvent`] function returns this handle.
        ///  * `cookie` - A pointer to a key value that an application can pass to the
        ///               [`IDXGIFactory2::unregister_occlusion_status`] method to unregister the
        ///               notification event that `event` specifies.
        ///
        /// # Return Value
        /// [`IDXGIFactory2::register_occlusion_status_event`] returns:
        ///  - [`S_OK`] if the method successfully registered the event.
        ///  - [`E_OUTOFMEMORY`] if memory is unavailable to complete the operation.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if `event` is not a valid handle or not an event handle.
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// If you call [`IDXGIFactory2::register_occlusion_status_event`] multiple times with the
        /// same event handle, [`IDXGIFactory2::register_occlusion_status_event`] fails with
        /// [`DXGI_ERROR_INVALID_CALL`].
        ///
        /// If you call [`IDXGIFactory2::register_occlusion_status_event`] multiple times with the
        /// different event handles, [`IDXGIFactory2::register_occlusion_status_event`] properly
        /// registers the events.
        fn register_occlusion_status_event(
            &mut self,
            event: HANDLE,
            cookie: *mut DWORD
        ) -> HRESULT;

        /// Unregisters a window or an event to stop it from receiving notification when occlusion
        /// status changes.
        ///
        /// # Parameters
        ///  * `cookie` - A key value for the window or event to unregister. The
        ///               [`IDXGIFactory2::register_occlusion_status_window`] or
        ///               [`IDXGIFactory2::register_occlusion_status_event`] method returns this
        ///               value.
        fn unregister_occlusion_status(&mut self, cookie: DWORD);

        /// Creates a swap chain that you can use to send Direct3D content into the
        /// DirectComposition API, to the Windows.UI.Xaml framework, or to Windows UI Library
        /// (WinUI) XAML, to compose in a window.
        ///
        /// # Parameters
        ///  * `device` - For Direct3D 11, and earlier versions of Direct3D, this is a pointer to
        ///               the Direct3D device for the swap chain. For Direct3D 12 this is a pointer
        ///               to a direct command queue (refer to [`ID3D12CommandQueue`]). This
        ///               parameter cannot be [`null_mut`]. Software drivers, like
        ///               [`D3D_DRIVER_TYPE::Reference`], are not supported for composition swap
        ///               chains.
        ///  * `desc` - A pointer to a [`DXGI_SWAP_CHAIN_DESC1`] structure for the swap-chain
        ///             description. This parameter cannot be [`null`]. You must specify the
        ///             [`DXGI_SWAP_EFFECT::FlipSequential`] value in the `swap_effect` member of
        ///             [`DXGI_SWAP_CHAIN_DESC1`] because
        ///             [`IDXGIFactory2::create_swap_chain_for_composition`] supports only flip
        ///             presentation model. You must also specify the [`DXGI_SCALING::Stretch`]
        ///             value in the `scaling` member of [`DXGI_SWAP_CHAIN_DESC1`].
        ///  * `restrict_to_output` - A pointer to the [`IDXGIOutput`] interface for the output to
        ///                           restrict content to. You must also pass the
        ///                           [`DXGI_PRESENT_RESTRICT_TO_OUTPUT`] flag in a
        ///                           [`IDXGISwapChain1::present1`] call to force the content to
        ///                           appear blacked out on any other output. If you want to
        ///                           restrict the content to a different output, you must create a
        ///                           new swap chain. However, you can conditionally restrict
        ///                           content based on the [`DXGI_PRESENT_RESTRICT_TO_OUTPUT`]
        ///                           flag. Set this parameter to [`null_mut`] if you don't want to
        ///                           restrict content to an output target.
        ///  * `swap_chain` - A pointer to a variable that receives a pointer to the
        ///                   [`IDXGISwapChain1`] interface for the swap chain that
        ///                   [`IDXGIFactory2::create_swap_chain_for_composition`] creates.
        ///
        /// # Return Value
        /// [`IDXGIFactory2::create_swap_chain_for_composition`] returns:
        ///  - [`S_OK`] if it successfully created a swap chain.
        ///  - [`E_OUTOFMEMORY`] if memory is unavailable to complete the operation.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the calling application provided invalid data, for
        ///    example, if `desc` or `swap_chain` is [`null`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic that are
        ///    defined by the type of device that you pass to `device`.
        ///
        /// # Remarks
        /// You can use composition swap chains with either:
        ///  - DirectComposition's [`IDCompositionVisual`] interface,
        ///  - System XAML's `SwapChainPanel` or `SwapChainBackgroundPanel` classes.
        ///  - Windows UI Library (WinUI) 3 XAML's `SwapChainPanel` or `SwapChainBackgroundPanel`
        ///    classes.
        ///
        /// For DirectComposition, you can call the [`IDCompositionVisual::set_content`] method to
        /// set the swap chain as the content of a visual object, which then allows you to bind the
        /// swap chain to the visual tree. For XAML, the `SwapChainBackgroundPanel` class exposes
        /// a classic COM interface [`ISwapChainBackgroundPanelNative`]. You can use the
        /// [`ISwapChainBackgroundPanelNative::set_swap_chain`] method to bind to the XAML UI
        /// graph.
        ///
        /// The [`IDXGISwapChain::set_fullscreen_state`], [`IDXGISwapChain::resize_target`],
        /// [`IDXGISwapChain::get_containing_output`], [`IDXGISwapChain1::get_hwnd`], and
        /// [`IDXGISwapChain::get_core_window`] methods aren't valid on this type of swap chain. If
        /// you call any of these methods on this type of swap chain, they fail.
        fn create_swap_chain_for_composition(
            &mut self,
            device: *mut IUnknown,
            desc: *const DXGI_SWAP_CHAIN_DESC1,
            restrict_to_output: *mut IDXGIOutput,
            swap_chain: *mut *mut IDXGISwapChain1
        ) -> HRESULT;
    }
);
