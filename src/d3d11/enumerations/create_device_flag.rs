// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{D3D11CreateDevice, D3D11CreateDeviceAndSwapChain, ID3D11Device},
    d3dcommon::D3D_DRIVER_TYPE,
    dxgi::DXGI_FORMAT,
};

/// Describes parameters that are used to create a device.
///
/// # Remarks
/// Device creation flags are used by [`D3D11CreateDevice`] and [`D3D11CreateDeviceAndSwapChain`].
///
/// An application might dynamically create (and destroy) threads to improve performance especially
/// on a machine with multiple CPU cores. There may be cases, however, when an application needs to
/// prevent extra threads from being created. This can happen when you want to simplify debugging,
/// profile code or develop a tool for instance. For these cases, use
/// [`D3D11_CREATE_DEVICE_FLAG::PreventInternalThreadingOptimizations`] to request that the runtime
/// and video driver not create any additional threads that might interfere with the application.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_CREATE_DEVICE_FLAG {
    /// Use this flag if your application will only call methods of Direct3D 11 interfaces from a
    /// single thread. By default, the [`ID3D11Device`] object is thread-safe. By using this flag,
    /// you can increase performance. However, if you use this flag and your application calls
    /// methods of Direct3D 11 interfaces from multiple threads, undefined behavior might result.
    SingleThreaded = 0x1,

    /// Creates a device that supports the debug layer. To use this flag, you must have
    /// `D3D11*SDKLayers.dll` installed; otherwise, device creation fails. To get
    /// `D3D11_1SDKLayers.dll`, install the SDK for Windows 8.
    Debug = 0x2,

    #[allow(missing_docs)]
    SwitchToRef = 0x4,

    /// Prevents multiple threads from being created. When this flag is used with a Windows
    /// Advanced Rasterization Platform (WARP) device, no additional threads will be created by
    /// WARP and all rasterization will occur on the calling thread. This flag is not recommended
    /// for general use.
    PreventInternalThreadingOptimizations = 0x8,

    /// Creates a device that supports BGRA formats ([`DXGI_FORMAT::B8G8R8A8UNorm`] and
    /// [`DXGI_FORMAT::B8G8R8A8UNormSRGB`]). All 10level9 and higher hardware with WDDM 1.1+
    /// drivers support BGRA formats.
    BgraSupport = 0x20,

    /// Causes the device and driver to keep information that you can use for shader debugging. The
    /// exact impact from this flag will vary from driver to driver. To use this flag, you must
    /// have `D3D11_1SDKLayers.dll` installed; otherwise, device creation fails. The created device
    /// supports the debug layer. To get `D3D11_1SDKLayers.dll`, install the SDK for Windows 8. If
    /// you use this flag and the current driver does not support shader debugging, device creation
    /// fails. Shader debugging requires a driver that is implemented to the WDDM for Windows 8
    /// (WDDM 1.2).
    Debuggable = 0x40,

    /// Causes the Direct3D runtime to ignore registry settings that turn on the debug layer. You
    /// can turn on the debug layer by using the DirectX Control Panel that was included as part of
    /// the DirectX SDK. We shipped the last version of the DirectX SDK in June 2010; you can
    /// download it from the Microsoft Download Center. You can set this flag in your app,
    /// typically in release builds only, to prevent end users from using the DirectX Control Panel
    /// to monitor how the app uses Direct3D.
    PreventAlteringLayerSettingsFromRegistry = 0x80,

    /// Use this flag if the device will produce GPU workloads that take more than two seconds to
    /// complete, and you want the operating system to allow them to successfully finish. If this
    /// flag is not set, the operating system performs timeout detection and recovery when it
    /// detects a GPU packet that took more than two seconds to execute. If this flag is set, the
    /// operating system allows such a long running packet to execute without resetting the GPU. We
    /// recommend not to set this flag if your device needs to be highly responsive so that the
    /// operating system can detect and recover from GPU timeouts. We recommend to set this flag if
    /// your device needs to perform time consuming background tasks such as compute, image
    /// recognition, and video encoding to allow such tasks to successfully finish.
    DisableGpuTimeout = 0x100,

    /// Forces the creation of the Direct3D device to fail if the display driver is not implemented
    /// to the WDDM for Windows 8 (WDDM 1.2). When the display driver is not implemented to
    /// WDDM 1.2, only a Direct3D device that is created with feature level 9.1, 9.2, or 9.3
    /// supports video; therefore, if this flag is set, the runtime creates the Direct3D device
    /// only for feature level 9.1, 9.2, or 9.3. We recommend not to specify this flag for
    /// applications that want to favor Direct3D capability over video. If feature level 10 and
    /// higher is available, the runtime will use that feature level regardless of video support.
    ///
    /// If this flag is set, device creation on the Basic Render Device (BRD) will succeed
    /// regardless of the BRD's missing support for video decode. This is because the Media
    /// Foundation video stack operates in software mode on BRD. In this situation, if you force
    /// the video stack to create the Direct3D device twice (create the device once with this flag,
    /// next discover BRD, then again create the device without the flag), you actually degrade
    /// performance.
    ///
    /// If you attempt to create a Direct3D device with driver type [`D3D_DRIVER_TYPE::Null`],
    /// [`D3D_DRIVER_TYPE::Reference`], or [`D3D_DRIVER_TYPE::Software`], device creation fails at
    /// any feature level because none of the associated drivers provide video capability. If you
    /// attempt to create a Direct3D device with driver type [`D3D_DRIVER_TYPE::Warp`], device
    /// creation succeeds to allow software fallback for video.
    VideoSupport = 0x800,
}
