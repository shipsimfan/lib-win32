use crate::{
    com_interface,
    dxgi::{
        IDXGIDeviceSubObject, IDXGIDeviceSubObjectTrait, IDXGIObject, IDXGIObjectTrait,
        IDXGIOutput, IDXGISwapChain, IDXGISwapChainTrait, DXGI_MODE_ROTATION, DXGI_RGBA,
    },
    dxgi1_2::{DXGI_PRESENT_PARAMETERS, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FULLSCREEN_DESC},
    unknwn::{IUnknown, IUnknownTrait},
    BOOL, HRESULT, HWND, REFIID, UINT,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{DXGI_PRESENT_DO_NOT_SEQUENCE, DXGI_PRESENT_STEREO_TEMPORARY_MONO, DXGI_SWAP_EFFECT},
    dxgi1_2::{IDXGIFactory2, DXGI_ALPHA_MODE, DXGI_SCALING},
    DXGI_ERROR_DEVICE_REMOVED, DXGI_ERROR_INVALID_CALL, DXGI_STATUS_OCCLUDED, E_INVALIDARG,
    E_OUTOFMEMORY, FALSE, S_OK, TRUE,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

com_interface!(
    /// Provides presentation capabilities that are enhanced from [`IDXGISwapChain`]. These
    /// presentation capabilities consist of specifying dirty rectangles and scroll rectangle to
    /// optimize the presentation.
    ///
    /// # Remarks
    /// You can create a swap chain by calling [`IDXGIFactory2::create_swap_chain_for_hwnd`],
    /// [`IDXGIFactory2::create_swap_chain_for_core_window`], or
    /// [`IDXGIFactory2::create_swap_chain_for_composition`]. You can also create a swap chain when
    /// you call [`D3D11CreateDeviceAndSwapChain`]; however, you can then only access the sub-set
    /// of swap-chain functionality that the [`IDXGISwapChain`] interface provides.
    ///
    /// [`IDXGISwapChain1`] provides the [`IDXGISwapChain1::is_temporary_mono_supported`] method
    /// that you can use to determine whether the swap chain supports "temporary mono”
    /// presentation. This type of swap chain is a stereo swap chain that can be used to present
    /// mono content.
    pub abstract IDXGISwapChain1(IDXGISwapChain1VTable/IDXGISwapChain1Trait):
        IDXGISwapChain/IDXGISwapChainTrait(swap_chain) +
        IDXGIDeviceSubObject/IDXGIDeviceSubObjectTrait(swap_chain.device_sub_object) +
        IDXGIObject/IDXGIObjectTrait(swap_chain.device_sub_object.object) +
        IUnknown/IUnknownTrait(swap_chain.device_sub_object.object.unknown) {
        const IID = 0x790A45F7-0x0D42-0x4876-0x983A-0x0A55CFE6F4AA;

        /// Gets a description of the swap chain.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`DXGI_SWAP_CHAIN_DESC1`] structure that describes the swap
        ///             chain.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn get_desc1(&mut self, desc: *mut DXGI_SWAP_CHAIN_DESC1) -> HRESULT;

        /// Gets a description of a full-screen swap chain.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`DXGI_SWAP_CHAIN_FULLSCREEN_DESC`] structure that describes
        ///             the full-screen swap chain.
        ///
        /// # Return Value
        /// [`IDXGISwapChain1::get_fullscreen_desc`] returns:
        ///  - [`S_OK`] if it successfully retrieved the description of the full-screen swap chain.
        ///  - [`DXGI_ERROR_INVALID_CALL`] for non-HWND swap chains or if `desc` is [`null_mut`].
        ///  - Possibly other error codes that are described in the DXGI_ERROR topic.
        ///
        /// # Remarks
        /// The semantics of [`IDXGISwapChain1::get_fullscreen_desc`] are identical to that of the
        /// [`IDXGISwapChain::get_desc`] method for HWND-based swap chains.
        fn get_fullscreen_desc(&mut self, desc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> HRESULT;

        /// Retrieves the underlying [`HWND`] for this swap-chain object.
        ///
        /// # Parameters
        ///  * `wnd` - A pointer to a variable that receives the [`HWND`] for the swap-chain
        ///            object.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// If `wnd` receives [`null_mut`] (that is, the swap chain is not [`HWND`]-based),
        /// [`IDXGISwapChain1::get_hwnd`] returns [`DXGI_ERROR_INVALID_CALL`].
        ///
        /// # Remarks
        /// Applications call the [`IDXGIFactory2::create_swap_chain_for_hwnd`] method to create a
        /// swap chain that is associated with an [`HWND`].
        fn get_hwnd(&mut self, wnd: *mut HWND) -> HRESULT;

        /// Retrieves the underlying CoreWindow object for this swap-chain object.
        ///
        /// # Parameters
        ///  * `iid` - A pointer to the globally unique identifier (GUID) of the CoreWindow object
        ///            that is referenced by the `unk` parameter.
        ///  * `unk` - A pointer to a variable that receives a pointer to the CoreWindow object.
        ///
        /// # Return Value
        /// [`IDXGISwapChain1::get_core_window`] returns:
        ///  - [`S_OK`] if it successfully retrieved the underlying CoreWindow object.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if `unk` is [`null_mut`]; that is, the swap chain is not
        ///    associated with a CoreWindow object.
        ///  - Any [`HRESULT`] that a call to [`IUnknown::query_interface`] to query for an
        ///    CoreWindow object might typically return.
        ///  - Possibly other error codes that are described in the DXGI_ERROR topic.
        ///
        /// # Remarks
        /// Applications call the [`IDXGIFactory2::create_swap_chain_for_core_window`] method to
        /// create a swap chain that is associated with an CoreWindow object.
        fn get_core_window(&mut self, iid: REFIID, unk: *mut *mut c_void) -> HRESULT;

        /// Presents a frame on the display screen.
        ///
        /// # Parameters
        ///  * `sync_interval` - An integer that specifies how to synchronize presentation of a
        ///                      frame with the vertical blank.
        ///  * `present_flags` - An integer value that contains swap-chain presentation options.
        ///                      These options are defined by the `DXGI_PRESENT` constants.
        ///  * `present_parameters` - A pointer to a [`DXGI_PRESENT_PARAMETERS`] structure that
        ///                           describes updated rectangles and scroll information of the
        ///                           frame to present.
        ///
        /// # Return Value
        /// Possible return values include: [`S_OK`], [`DXGI_ERROR_DEVICE_REMOVED`] ,
        /// [`DXGI_STATUS_OCCLUDED`], [`DXGI_ERROR_INVALID_CALL`], or [`E_OUTOFMEMORY`].
        ///
        /// # Remarks
        /// An app can use [`IDXGISwapChain1::present1`] to optimize presentation by specifying
        /// scroll and dirty rectangles. When the runtime has information about these rectangles,
        /// the runtime can then perform necessary bitblts during presentation more efficiently and
        /// pass this metadata to the Desktop Window Manager (DWM). The DWM can then use the
        /// metadata to optimize presentation and pass the metadata to indirect displays and
        /// terminal servers to optimize traffic over the wire. An app must confine its
        /// modifications to only the dirty regions that it passes to
        /// [`IDXGISwapChain1::present1`], as well as modify the entire dirty region to avoid
        /// undefined resource contents from being exposed.
        ///
        /// For flip presentation model swap chains that you create with the
        /// [`DXGI_SWAP_EFFECT::FlipSequential`] value set, a successful presentation results in an
        /// unbind of back buffer 0 from the graphics pipeline, except for when you pass the
        /// [`DXGI_PRESENT_DO_NOT_SEQUENCE`] flag in the `flags` parameter.
        fn present1(
            &mut self,
            sync_interval: UINT,
            present_flags: UINT,
            present_parameters: *const DXGI_PRESENT_PARAMETERS
        ) -> HRESULT;

        /// Determines whether a swap chain supports “temporary mono.”
        ///
        /// # Return Value
        /// Indicates whether to use the swap chain in temporary mono mode. [`TRUE`] indicates that
        /// you can use temporary-mono mode; otherwise, [`FALSE`].
        ///
        /// # Remarks
        /// Temporary mono is a feature where a stereo swap chain can be presented using only the
        /// content in the left buffer. To present using the left buffer as a mono buffer, an
        /// application calls the [`IDXGISwapChain1::present1`] method with the
        /// [`DXGI_PRESENT_STEREO_TEMPORARY_MONO`] flag. All windowed swap chains support temporary
        /// mono. However, full-screen swap chains optionally support temporary mono because not
        /// all hardware supports temporary mono on full-screen swap chains efficiently.
        fn is_temporary_mono_supported(&mut self) -> BOOL;

        /// Gets the output (the display monitor) to which you can restrict the contents of a
        /// present operation.
        ///
        /// # Parameters
        ///  * `restrict_to_output` - A pointer to a buffer that receives a pointer to the
        ///                           [`IDXGIOutput`] interface for the restrict-to output. An
        ///                           application passes this pointer to [`IDXGIOutput`] in a call
        ///                           to the [`IDXGIFactory2::create_swap_chain_for_hwnd`],
        ///                           [`IDXGIFactory2::create_swap_chain_for_core_window`], or
        ///                           [`IDXGIFactory2::create_swap_chain_for_composition`] method
        ///                           to create the swap chain.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if the restrict-to output was successfully retrieved; otherwise,
        /// returns [`E_INVALIDARG`] if the pointer is invalid.
        ///
        /// # Remarks
        /// If the method succeeds, the runtime fills the buffer at `restrict_to_output` with a
        /// pointer to the restrict-to output interface. This restrict-to output interface has its
        /// reference count incremented. When you are finished with it, be sure to release the
        /// interface to avoid a memory leak.
        ///
        /// The output is also owned by the adapter on which the swap chain's device was created.
        fn get_restrict_to_output(&mut self, restrict_to_output: *mut *mut IDXGIOutput) -> HRESULT;

        /// Changes the background color of the swap chain.
        ///
        /// # Parameters
        ///  * `color` - A pointer to a [`DXGI_RGBA`] structure that specifies the background color
        ///              to set.
        ///
        /// # Return Value
        /// [`IDXGISwapChain1::set_background_color`] returns:
        ///  - [`S_OK`] if it successfully set the background color.
        ///  - [`E_INVALIDARG`] if the `color` parameter is incorrect, for example, `color` is
        ///    [`null`] or any of the floating-point values of the members of [`DXGI_RGBA`] to
        ///    which `color` points are outside the range from 0.0 through 1.0.
        ///  - Possibly other error codes that are described in the DXGI_ERROR topic.
        ///
        /// # Remarks
        /// The background color affects only swap chains that you create with
        /// [`DXGI_SCALING::None`] in windowed mode. You pass this value in a call to
        /// [`IDXGIFactory2::create_swap_chain_for_hwnd`],
        /// [`IDXGIFactory2::create_swap_chain_for_core_window`], or
        /// [`IDXGIFactory2::create_swap_chain_for_composition`]. Typically, the background color
        /// is not visible unless the swap-chain contents are smaller than the destination window.
        ///
        /// When you set the background color, it is not immediately realized. It takes effect in
        /// conjunction with your next call to the [`IDXGISwapChain1::present1`] method. The
        /// `DXGI_PRESENT` flags that you pass to [`IDXGISwapChain1::present1`] can help achieve
        /// the effect that you require. For example, if you call
        /// [`IDXGISwapChain1::set_background_color`] and then call [`IDXGISwapChain1::present1`]
        /// with the `flags` parameter set to [`DXGI_PRESENT_DO_NOT_SEQUENCE`], you change only the
        /// background color without changing the displayed contents of the swap chain.
        ///
        /// When you call the [`IDXGISwapChain1::present1`] method to display contents of the swap
        /// chain, [`IDXGISwapChain1::present1`] uses the [`DXGI_ALPHA_MODE`] value that is
        /// specified in the `alpha_mode` member of the [`DXGI_SWAP_CHAIN_DESC1`] structure to
        /// determine how to handle the a member of the [`DXGI_RGBA`] structure, the alpha value of
        /// the background color, that achieves window transparency. For example, if `alpha_mode`
        /// is [`DXGI_ALPHA_MODE::Ignore`], [`IDXGISwapChain1::present1`] ignores the a member of
        /// [`DXGI_RGBA`].
        fn set_background_color(&mut self, color: *const DXGI_RGBA) -> HRESULT;

        /// Retrieves the background color of the swap chain.
        ///
        /// # Parameters
        ///  * `color` - A pointer to a [`DXGI_RGBA`] structure that receives the background color
        ///              of the swap chain.
        ///
        /// # Return Value
        /// [`IDXGISwapChain1::get_background_color`] returns:
        ///  - [`S_OK`] if it successfully retrieves the background color.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the `color` parameter is invalid, for example,
        ///    `color` is [`null_mut`].
        ///  - Possibly other error codes that are described in the DXGI_ERROR topic.
        fn get_background_color(&mut self, color: *mut DXGI_RGBA) -> HRESULT;

        /// Sets the rotation of the back buffers for the swap chain.
        ///
        /// # Parameters
        ///  * `rotation` - A [`DXGI_MODE_ROTATION`]-typed value that specifies how to set the
        ///                 rotation of the back buffers for the swap chain.
        ///
        /// # Return Value
        /// [`IDXGISwapChain1::set_rotation`] returns:
        ///  - [`S_OK`] if it successfully set the rotation.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the swap chain is bit-block transfer (bitblt) model.
        ///    The swap chain must be flip model to successfully call
        ///    [`IDXGISwapChain1::set_rotation`].
        ///  - Possibly other error codes that are described in the DXGI_ERROR topic.
        ///
        /// # Remarks
        /// You can only use [`IDXGISwapChain1::set_rotation`] to rotate the back buffers for
        /// flip-model swap chains that you present in windowed mode.
        ///
        /// [`IDXGISwapChain1::set_rotation`] isn't supported for rotating the back buffers for
        /// flip-model swap chains that you present in full-screen mode. In this situation,
        /// [`IDXGISwapChain1::set_rotation`] doesn't fail, but you must ensure that you specify no
        /// rotation ([`DXGI_MODE_ROTATION::Identity`]) for the swap chain. Otherwise, when you
        /// call [`IDXGISwapChain1::present1 `]or [`IDXGISwapChain::present`] to present a frame,
        /// the presentation fails.
        fn set_rotation(&mut self, rotation: DXGI_MODE_ROTATION) -> HRESULT;

        /// Gets the rotation of the back buffers for the swap chain.
        ///
        /// # Parameters
        ///  * `rotation` - A pointer to a variable that receives a [`DXGI_MODE_ROTATION`]-typed
        ///                 value that specifies the rotation of the back buffers for the swap
        ///                 chain.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn get_rotation(&mut self, rotation: *mut DXGI_MODE_ROTATION) -> HRESULT;
    }
);
