/// Specifies color space support for the swap chain.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    /// Color space support is present.
    Present = 0x1,

    /// Overlay color space support is present.
    OverlayPresent = 0x2,
}
