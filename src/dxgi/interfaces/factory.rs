use crate::{
    com_interface,
    dxgi::{IDXGIAdapter, IDXGIObject, IDXGIObjectTrait, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC},
    unknwn::{IUnknown, IUnknownTrait},
    HMODULE, HRESULT, HWND, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{DXGI_MWA_NO_ALT_ENTER, DXGI_MWA_NO_PRINT_SCREEN, DXGI_MWA_NO_WINDOW_CHANGES},
    FreeLibrary, GetModuleHandle, LoadLibrary, DXGI_ERROR_INVALID_CALL, DXGI_ERROR_NOT_FOUND,
    DXGI_STATUS_OCCLUDED, E_OUTOFMEMORY, S_OK, WM_SIZE, WM_SIZING,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// An [`IDXGIFactory`] interface implements methods for generating DXGI objects (which handle
    /// full screen transitions).
    ///
    /// Create a factory by calling [`CreateDXGIFactory`].
    ///
    /// Because you can create a Direct3D device without creating a swap chain, you might need to
    /// retrieve the factory that is used to create the device in order to create a swap chain. You
    /// can request the [`IDXGIDevice`] interface from the Direct3D device and then use the
    /// [`IDXGIObject::get_parent`] method to locate the factory.
    pub abstract IDXGIFactory(IDXGIFactoryVTable/IDXGIFactoryTrait):
        IDXGIObject/IDXGIObjectTrait(object) +
        IUnknown/IUnknownTrait(object.unknown) {
        const IID = 0x7B7166EC-0x21C7-0x44AE-0xB21A-0xC9AE321AE369;

        /// Enumerates the adapters (video cards).
        ///
        /// # Parameters
        ///  * `adapter` - The index of the adapter to enumerate.
        ///  * `p_adapter` - The address of a pointer to an [`IDXGIAdapter`] interface at the
        ///                  position specified by the Adapter parameter. This parameter must not
        ///                  be [`null_mut`].
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns [`DXGI_ERROR_NOT_FOUND`] if the
        /// index is greater than or equal to the number of adapters in the local system, or
        /// [`DXGI_ERROR_INVALID_CALL`] if `p_adapter` parameter is [`null_mut`].
        ///
        /// # Remarks
        /// When you create a factory, the factory enumerates the set of adapters that are
        /// available in the system. Therefore, if you change the adapters in a system, you must
        /// destroy and recreate the [`IDXGIFactory`] object. The number of adapters in a system
        /// changes when you add or remove a display card, or dock or undock a laptop.
        ///
        /// When the `enum_adapters` method succeeds and fills the `p_adapter` parameter with the
        /// address of the pointer to the adapter interface, `enum_adapters` increments the adapter
        /// interface's reference count. When you finish using the adapter interface, call the
        /// `release` method to decrement the reference count before you destroy the pointer.
        ///
        /// `enum_adapters` first returns the adapter with the output on which the desktop primary
        /// is displayed. This adapter corresponds with an index of zero. `enum_adapters` next
        /// returns other adapters with outputs. `enum_adapters` finally returns adapters without
        /// outputs.
        fn enum_adapters(&mut self, adapter: UINT, p_adapter: *mut *mut IDXGIAdapter) -> HRESULT;

        /// Allows DXGI to monitor an application's message queue for the `alt-enter` key sequence
        /// (which causes the application to switch from windowed to full screen or vice versa).
        ///
        /// # Parameters
        ///  * `window_handle` - The handle of the window that is to be monitored. This parameter
        ///                      can be [`null_mut`]; but only if `flags` is also 0.
        ///  * `flags` - One or more of the following values.
        ///    * [`DXGI_MWA_NO_WINDOW_CHANGES`] - Prevent DXGI from monitoring an applications
        ///                                       message queue; this makes DXGI unable to respond
        ///                                       to mode changes.
        ///    * [`DXGI_MWA_NO_ALT_ENTER`] - Prevent DXGI from responding to an alt-enter sequence.
        ///    * [`DXGI_MWA_NO_PRINT_SCREEN`] - Prevent DXGI from responding to a print-screen key.
        ///
        /// # Return Value
        /// [`DXGI_ERROR_INVALID_CALL`] if `window` is invalid, or [`E_OUTOFMEMORY`].
        ///
        /// # Remarks
        /// The combination of `window` and `flags` informs DXGI to stop monitoring window messages
        /// for the previously-associated window.
        ///
        /// If the application switches to full-screen mode, DXGI will choose a full-screen
        /// resolution to be the smallest supported resolution that is larger or the same size as
        /// the current back buffer size.
        ///
        /// Applications can make some changes to make the transition from windowed to full screen
        /// more efficient. For example, on a [`WM_SIZE`] message, the application should release
        /// any outstanding swap-chain back buffers, call [`IDXGISwapChain::resize_buffers`], then
        /// re-acquire the back buffers from the swap chain(s). This gives the swap chain(s) an
        /// opportunity to resize the back buffers, and/or recreate them to enable full-screen
        /// flipping operation. If the application does not perform this sequence, DXGI will still
        /// make the full-screen/windowed transition, but may be forced to use a stretch operation
        /// (since the back buffers may not be the correct size), which may be less efficient. Even
        /// if a stretch is not required, presentation may not be optimal because the back buffers
        /// might not be directly interchangeable with the front buffer. Thus, a call to
        /// `resize_buffers` on [`WM_SIZE`] is always recommended, since [`WM_SIZE`] is always sent
        /// during a fullscreen transition.
        ///
        /// While windowed, the application can, if it chooses, restrict the size of its window's
        /// client area to sizes to which it is comfortable rendering. A fully flexible application
        /// would make no such restriction, but UI elements or other design considerations can, of
        /// course, make this flexibility untenable. If the application further chooses to restrict
        /// its window's client area to just those that match supported full-screen resolutions,
        /// the application can field [`WM_SIZING`], then check against
        /// [`IDXGIOutput::find_closest_matching_mode`]. If a matching mode is found, allow the
        /// resize. (The [`IDXGIOutput`] can be retrieved from
        /// [`IDXGISwapChain::get_containing_output`]. Absent subsequent changes to desktop
        /// topology, this will be the same output that will be chosen when `alt-enter` is fielded
        /// and fullscreen mode is begun for that swap chain.)
        ///
        /// Applications that want to handle mode changes or `alt+enter` themselves should call
        /// `make_window_association` with the [`DXGI_MWA_NO_WINDOW_CHANGES`] flag after swap chain
        /// creation. The `window` argument, if not [`null_mut`], specifies that the application
        /// message queues will not be handled by the DXGI runtime for all swap chains of a
        /// particular target HWND. Calling `make_window_association` with the
        /// [`DXGI_MWA_NO_WINDOW_CHANGES`] flag after swapchain creation ensures that DXGI will not
        /// interfere with application's handling of window mode changes or `alt+enter`.
        ///
        /// You must call the `make_window_association` method on the factory object associated
        /// with the target [`HWND`] swap chain(s). You can guarantee that by calling the
        /// [`IDXGIObject::get_parent`] method on the swap chain(s) to locate the factory.
        fn make_window_association(&mut self, window: HWND, flags: UINT) -> HRESULT;

        /// Get the window through which the user controls the transition to and from full screen.
        ///
        /// # Parameters
        ///  * `window` - A pointer to a window handle.
        ///
        /// # Return Value
        /// Returns a code that indicates success or failure. [`S_OK`] indicates success,
        /// [`DXGI_ERROR_INVALID_CALL`] indicates `window` was passed in as [`null_mut`].
        fn get_window_association(&mut self, window: *mut HWND) -> HRESULT;

        /// Creates a swap chain.
        ///
        /// # Parameters
        ///  * `device` - For Direct3D 11, and earlier versions of Direct3D, this is a pointer to
        ///               the Direct3D device for the swap chain. For Direct3D 12 this is a pointer
        ///               to a direct command queue (refer to [`ID3D12CommandQueue`]) . This
        ///               parameter cannot be [`null_mut`].
        ///  * `desc` - A pointer to a [`DXGI_SWAP_CHAIN_DESC`] structure for the swap-chain
        ///             description. This parameter cannot be [`null_mut`].
        ///  * `swap_chain` - A pointer to a variable that receives a pointer to the
        ///                   [`IDXGISwapChain`] interface for the swap chain that
        ///                   `create_swap_chain` creates.
        ///
        /// # Return Value
        /// [`DXGI_ERROR_INVALID_CALL`] if `desc` or `swap_chain` is [`null_mut`],
        /// [`DXGI_STATUS_OCCLUDED`] if you request full-screen mode and it is unavailable, or
        /// [`E_OUTOFMEMORY`]. Other error codes defined by the type of device passed in may also
        /// be returned.
        ///
        /// # Remarks
        /// If you attempt to create a swap chain in full-screen mode, and full-screen mode is
        /// unavailable, the swap chain will be created in windowed mode and
        /// [`DXGI_STATUS_OCCLUDED`] will be returned.
        ///
        /// If the buffer width or the buffer height is zero, the sizes will be inferred from the
        /// output window size in the swap-chain description.
        ///
        /// Because the target output can't be chosen explicitly when the swap chain is created, we
        /// recommend not to create a full-screen swap chain. This can reduce presentation
        /// performance if the swap chain size and the output window size do not match. Here are
        /// two ways to ensure that the sizes match:
        ///  - Create a windowed swap chain and then set it full-screen using
        ///    [`IDXGISwapChain::set_fullscreen_state`].
        ///  - Save a pointer to the swap chain immediately after creation, and use it to get the
        ///    output window size during a [`WM_SIZE`] event. Then resize the swap chain buffers
        ///    (with [`IDXGISwapChain::resize_buffers`]) during the transition from windowed to
        ///    full-screen.
        ///
        /// If the swap chain is in full-screen mode, before you release it you must use
        /// [`set_fullscreen_sstate`] to switch it to windowed mode. For more information about
        /// releasing a swap chain, see the "Destroying a Swap Chain" section of DXGI Overview.
        ///
        /// After the runtime renders the initial frame in full screen, the runtime might
        /// unexpectedly exit full screen during a call to [`IDXGISwapChain::present`].
        ///
        /// You can specify `DXGI_SWAP_EFFECT` and `DXGI_SWAP_CHAIN_FLAG` values in the swap-chain
        /// description that `desc` points to. These values allow you to use features like
        /// flip-model presentation and content protection by using pre-Windows 8 APIs.
        ///
        /// However, to use stereo presentation and to change resize behavior for the flip model,
        /// applications must use the [`IDXGIFactory2::create_swap_chain_for_hwnd`] method.
        /// Otherwise, the back-buffer contents implicitly scale to fit the presentation target
        /// size; that is, you can't turn off scaling.
        fn create_swap_chain(
            &mut self,
            device: *mut IUnknown,
            desc: *mut DXGI_SWAP_CHAIN_DESC,
            swap_chain: *mut *mut IDXGISwapChain
        ) -> HRESULT;

        /// Create an adapter interface that represents a software adapter.
        ///
        /// # Parameters
        ///  * `module` - Handle to the software adapter's dll. [`HMODULE`] can be obtained with
        ///               [`GetModuleHandle`] or [`LoadLibrary`].
        ///  * `adapter` - Address of a pointer to an adapter (see [`IDXGIAdapter`]).
        ///
        /// # Return Value
        /// A return code indicating success or failure.
        ///
        /// # Remarks
        /// A software adapter is a DLL that implements the entirety of a device driver interface,
        /// plus emulation, if necessary, of kernel-mode graphics components for Windows. Details
        /// on implementing a software adapter can be found in the Windows Vista Driver Development
        /// Kit. This is a very complex development task, and is not recommended for general
        /// readers.
        ///
        /// Calling this method will increment the module's reference count by one. The reference
        /// count can be decremented by calling [`FreeLibrary`].
        ///
        /// The typical calling scenario is to call [`LoadLibrary`], pass the handle to
        /// `create_software_adapter`, then immediately call [`FreeLibrary`] on the DLL and forget
        /// the DLL's [`HMODULE`]. Since the software adapter calls [`FreeLibrary`] when it is
        /// destroyed, the lifetime of the DLL will now be owned by the adapter, and the
        /// application is free of any further consideration of its lifetime.
        fn create_software_adapter(
            &mut self,
            module: HMODULE,
            adapter: *mut *mut IDXGIAdapter
        ) -> HRESULT;
    }
);
