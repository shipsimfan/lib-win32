/// Specifies support for overlay color space.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    /// Overlay color space support is present.
    Present = 0x1,
}
