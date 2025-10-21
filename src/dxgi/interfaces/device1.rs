use crate::{
    com_interface,
    dxgi::{IDXGIDevice, IDXGIDeviceTrait, IDXGIObject, IDXGIObjectTrait},
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "dxgi1_3")]
use crate::dxgi1_3::IDXGISwapChain2;
#[allow(unused_imports)]
use crate::{dxgi::DXGI_SWAP_CHAIN_FLAG, DXGI_ERROR_DEVICE_REMOVED, S_OK};

com_interface!(
    /// An [`IDXGIDevice1`] interface implements a derived class for DXGI objects that produce
    /// image data.
    ///
    /// # Remarks
    /// This interface is not supported by Direct3D 12 devices. Direct3D 12 applications have
    /// direct control over their swapchain management, so better latency control should be handled
    /// by the application. You can make use of Waitable objects (refer to
    /// [`DXGI_SWAP_CHAIN_FLAG::FrameLatencyWaitableObject`]) and the
    /// [`IDXGISwapChain2::set_maximum_frame_latency`] method if desired.
    ///
    /// This interface is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
    /// Server 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows Server
    /// 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows Server
    /// 2008.
    ///
    /// The [`IDXGIDevice1`] interface is designed for use by DXGI objects that need access to
    /// other DXGI objects. This interface is useful to applications that do not use Direct3D to
    /// communicate with DXGI.
    ///
    /// The Direct3D create device functions return a Direct3D device object. This Direct3D device
    /// object implements the [`IUnknown`] interface. You can query this Direct3D device object for
    /// the device's corresponding [`IDXGIDevice1`] interface.
    pub abstract IDXGIDevice1(IDXGIDevice1VTable/IDXGIDevice1Trait):
        IDXGIDevice/IDXGIDeviceTrait(device) +
        IDXGIObject/IDXGIObjectTrait(device.object) +
        IUnknown/IUnknownTrait(device.object.unknown) {
        const IID = 0x77DB970F-0x6276-0x48BA-0xBA28-0x070143B4392C;

        /// Sets the number of frames that the system is allowed to queue for rendering.
        ///
        /// # Parameters
        ///  * `max_latency` - The maximum number of back buffer frames that a driver can queue.
        ///                    The value defaults to 3, but can range from 1 to 16. A value of 0
        ///                    will reset latency to the default. For multi-head devices, this
        ///                    value is specified per-head.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, [`DXGI_ERROR_DEVICE_REMOVED`] if the device
        /// was removed.
        ///
        /// # Remarks
        /// This method is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
        /// Server 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows
        /// Server 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows
        /// Server 2008.
        ///
        /// Frame latency is the number of frames that are allowed to be stored in a queue before
        /// submission for rendering. Latency is often used to control how the CPU chooses between
        /// responding to user input and frames that are in the render queue. It is often
        /// beneficial for applications that have no user input (for example, video playback) to
        /// queue more than 3 frames of data.
        fn set_maximum_frame_latency(&mut self, max_latency: UINT) -> HRESULT;

        /// Gets the number of frames that the system is allowed to queue for rendering.
        ///
        /// # Parameters
        ///  * `max_latency` - This value is set to the number of frames that can be queued for
        ///                    render. This value defaults to 3, but can range from 1 to 16.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the following members of the
        /// `D3DERR` enumerated type:
        ///  - [`D3DERR_DEVICELOST`]
        ///  - [`D3DERR_DEVICEREMOVED`]
        ///  - [`D3DERR_DRIVERINTERNALERROR`]
        ///  - [`D3DERR_INVALIDCALL`]
        ///  - [`D3DERR_OUTOFVIDEOMEMORY`]
        ///
        /// # Remarks
        /// This method is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
        /// Server 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows
        /// Server 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows
        /// Server 2008.
        ///
        /// Frame latency is the number of frames that are allowed to be stored in a queue before
        /// submission for rendering. Latency is often used to control how the CPU chooses between
        /// responding to user input and frames that are in the render queue. It is often
        /// beneficial for applications that have no user input (for example, video playback) to
        /// queue more than 3 frames of data.
        fn get_maximum_frame_latency(&mut self, max_latency: *mut UINT) -> HRESULT;
    }
);
