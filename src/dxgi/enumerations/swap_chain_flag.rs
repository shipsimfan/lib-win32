// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{
        IDXGIDevice1, IDXGIFactory, IDXGISurface1, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC,
        DXGI_SWAP_EFFECT,
    },
    dxgi1_2::{IDXGIFactory2, IDXGISwapChain1, DXGI_SWAP_CHAIN_DESC1},
    dxgi1_3::IDXGIOutput2,
    SetWindowDisplayAffinity, HWND,
};

/// Options for swap-chain behavior.
///
/// # Remarks
/// This enumeration is used by the [`DXGI_SWAP_CHAIN_DESC`] structure and the
/// [`IDXGISwapChain::resize_target`] method.
///
/// This enumeration is also used by the [`DXGI_SWAP_CHAIN_DESC1`] structure.
///
/// You don't need to set [`DXGI_SWAP_CHAIN_FLAG::DisplayOnly`] for swap chains that you create in
/// full-screen mode with the [`IDXGIFactory::create_swap_chain`] method because those swap chains
/// already behave as if [`DXGI_SWAP_CHAIN_FLAG::DisplayOnly`] is set. That is, presented content
/// is not accessible by remote access or through the desktop duplication APIs.
///
/// Swap chains that you create with the [`IDXGIFactory2::create_swap_chain_for_hwnd`],
/// [`IDXGIFactory2::create_swap_chain_for_core_window`], and
/// [`IDXGIFactory2::create_swap_chain_for_composition`] methods are not protected if
/// [`DXGI_SWAP_CHAIN_FLAG::DisplayOnly`] is not set and are protected if
/// [`DXGI_SWAP_CHAIN_FLAG::DisplayOnly`] is set. When swap chains are protected, screen scraping
/// is prevented and, in full-screen mode, presented content is not accessible through the desktop
/// duplication APIs.
///
/// When you call [`IDXGISwapChain::resize_buffers`] to change the swap chain's back buffer, you
/// can reset or change all [`DXGI_SWAP_CHAIN_FLAG`] flags.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_SWAP_CHAIN_FLAG {
    /// Set this flag to turn off automatic image rotation; that is, do not perform a rotation when
    /// transferring the contents of the front buffer to the monitor.
    ///
    /// Use this flag to avoid a bandwidth penalty when an application expects to handle rotation.
    /// This option is valid only during full-screen mode.
    NonPrerotated = 1,

    /// Set this flag to enable an application to switch modes by calling
    /// [`IDXGISwapChain::resize_target`].
    ///
    /// When switching from windowed to full-screen mode, the display mode (or monitor resolution)
    /// will be changed to match the dimensions of the application window.
    AllowModeSwitch = 2,

    /// Set this flag to enable an application to render using GDI on a swap chain or a surface.
    ///
    /// This will allow the application to call [`IDXGISurface1::get_dc`] on the 0th back buffer or
    /// a surface.
    ///
    /// This flag is not applicable for Direct3D 12.
    GdiCompatible = 4,

    /// Set this flag to indicate that the swap chain might contain protected content; therefore,
    /// the operating system supports the creation of the swap chain only when driver and hardware
    /// protection is used. If the driver and hardware do not support content protection, the call
    /// to create a resource for the swap chain fails.
    RestrictedContent = 8,

    /// Set this flag to indicate that shared resources that are created within the swap chain must
    /// be protected by using the driver’s mechanism for restricting access to shared surfaces.
    RestrictSharedResourceDriver = 16,

    /// Set this flag to restrict presented content to the local displays. Therefore, the presented
    /// content is not accessible via remote accessing or through the desktop duplication APIs.
    ///
    /// This flag supports the window content protection features of Windows. Applications can use
    /// this flag to protect their own onscreen window content from being captured or copied
    /// through a specific set of public operating system features and APIs.
    ///
    /// If you use this flag with windowed swap chains where another process created the [`HWND`],
    /// the owner of the [`HWND`] must use the [`SetWindowDisplayAffinity`] function appropriately
    /// in order to allow calls to [`IDXGISwapChain::present`] or [`IDXGISwapChain1::present1`] to
    /// succeed.
    DisplayOnly = 32,

    /// Set this flag to create a waitable object you can use to ensure rendering does not begin
    /// while a frame is still being presented. When this flag is used, the swapchain's latency
    /// must be set with the [`IDXGISwapChain2::set_maximum_frame_latency`] API instead of
    /// [`IDXGIDevice1::set_maximum_frame_latency`].
    ///
    /// This flag isn't supported in full-screen mode, unless the render API is Direct3D 12.
    FrameLatencyWaitableObject = 64,

    /// Set this flag to create a swap chain in the foreground layer for multi-plane rendering.
    /// This flag can only be used with CoreWindow swap chains, which are created with
    /// [`IDXGIFactory2::create_swap_chain_for_core_window`]. Apps should not create foreground
    /// swap chains if [`IDXGIOutput2::supports_overlays`] indicates that hardware support for
    /// overlays is not available.
    ///
    /// Note that [`IDXGISwapChain::resize_buffers`] cannot be used to add or remove this flag.
    ForegroundLayer = 128,

    /// Set this flag to create a swap chain for full-screen video.
    FullscreenVideo = 256,

    /// Set this flag to create a swap chain for YUV video.
    YuvVideo = 512,

    /// Indicates that the swap chain should be created such that all underlying resources can be
    /// protected by the hardware. Resource creation will fail if hardware content protection is
    /// not supported.
    ///
    /// This flag has the following restrictions:
    ///  - This flag can only be used with swap effect [`DXGI_SWAP_EFFECT::FlipSequential`].
    HwProtected = 1024,

    /// Tearing support is a requirement to enable displays that support variable refresh rates to
    /// function properly when the application presents a swap chain tied to a full screen
    /// borderless window. Win32 apps can already achieve tearing in fullscreen exclusive mode by
    /// calling `SetFullscreenState(TRUE)`, but the recommended approach for Win32 developers is to
    /// use this tearing flag instead. This flag requires the use of a `DXGI_SWAP_EFFECT_FLIP_*`
    /// swap effect.
    ///
    /// To check for hardware support of this feature, refer to
    /// [`IDXGIFactory5::check_feature_support`]. For usage information refer to
    /// [`IDXGISwapChain::present`] and the `DXGI_PRESENT` flags.
    AllowTearing = 2048,

    #[allow(missing_docs)]
    RestrictedToAllHolographicDisplays = 4096,
}
