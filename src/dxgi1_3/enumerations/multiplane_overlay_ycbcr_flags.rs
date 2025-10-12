// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_3::IDXGIDecodeSwapChain;

/// Options for swap-chain color space.
///
/// # Remarks
/// This enum is used by [`IDXGIDecodeSwapChain::set_color_space`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    /// Specifies nominal range YCbCr, which isn't an absolute color space, but a way of encoding
    /// RGB info.
    NominalRange = 0x1,

    /// Specifies BT.709, which standardizes the format of high-definition television and has 16:9
    /// (widescreen) aspect ratio.
    BT709 = 0x2,

    /// Specifies xvYCC or extended-gamut YCC (also x.v.Color) color space that can be used in the
    /// video electronics of television sets to support a gamut 1.8 times as large as that of the
    /// sRGB color space.
    xvYCC = 0x4,
}
