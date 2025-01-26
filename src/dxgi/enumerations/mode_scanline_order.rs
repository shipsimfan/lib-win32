/// Flags indicating the method the raster uses to create an image on a surface.
///
/// # Remarks
/// This enum is used by the [`DXGI_MODE_DESC1`] and [`DXGI_SWAP_CHAIN_FULLSCREEN_DESC`]
/// structures.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_MODE_SCANLINE_ORDER {
    /// Scanline order is unspecified.
    Unspecified = 0,

    /// The image is created from the first scanline to the last without skipping any.
    Progressive = 1,

    /// The image is created beginning with the upper field.
    UpperFieldFirst = 2,

    /// The image is created beginning with the lower field.
    LowerFieldFirst = 3,
}
