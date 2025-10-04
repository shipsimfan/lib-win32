// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{D3D11CreateDevice, D3D11CreateDeviceAndSwapChain};

/// Driver type options.
///
/// # Remarks
/// The driver type is required when calling [`D3D11CreateDevice`] or
/// [`D3D11CreateDeviceAndSwapChain`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D_DRIVER_TYPE {
    /// The driver type is unknown.
    Unknown = 0,

    /// A hardware driver, which implements Direct3D features in hardware. This is the primary
    /// driver that you should use in your Direct3D applications because it provides the best
    /// performance. A hardware driver uses hardware acceleration (on supported hardware) but can
    /// also use software for parts of the pipeline that are not supported in hardware. This driver
    /// type is often referred to as a hardware abstraction layer or HAL.
    Hardware,

    /// A reference driver, which is a software implementation that supports every Direct3D
    /// feature. A reference driver is designed for accuracy rather than speed and as a result is
    /// slow but accurate. The rasterizer portion of the driver does make use of special CPU
    /// instructions whenever it can, but it is not intended for retail applications; use it only
    /// for feature testing, demonstration of functionality, debugging, or verifying bugs in other
    /// drivers. The reference device for this driver is installed by the Windows SDK 8.0 or later
    /// and is intended only as a debug aid for development purposes. This driver may be referred
    /// to as a REF driver, a reference driver, or a reference rasterizer.
    Reference,

    /// A NULL driver, which is a reference driver without render capability. This driver is
    /// commonly used for debugging non-rendering API calls, it is not appropriate for retail
    /// applications. This driver is installed by the DirectX SDK.
    Null,

    /// A software driver, which is a driver implemented completely in software. The software
    /// implementation is not intended for a high-performance application due to its very slow
    /// performance.
    Software,

    /// A WARP driver, which is a high-performance software rasterizer. The rasterizer supports
    /// feature levels 9_1 through level 10_1 with a high performance software implementation.
    Warp,
}
