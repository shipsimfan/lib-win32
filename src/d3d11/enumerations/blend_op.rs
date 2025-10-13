/// RGB or alpha blending operation.
///
/// # Remarks
/// The runtime implements RGB blending and alpha blending separately. Therefore, blend state
/// requires separate blend operations for RGB data and alpha data. These blend operations are
/// specified in a blend description.
///
/// Blend state is used by the output-merger stage to determine how to blend together two RGB pixel
/// values and two alpha values. The two RGB pixel values and two alpha values are the RGB pixel
/// value and alpha value that the pixel shader outputs and the RGB pixel value and alpha value
/// already in the output render target. The blend option controls the data source that the
/// blending stage uses to modulate values for the pixel shader, render target, or both. The blend
/// operation controls how the blending stage mathematically combines these modulated values.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_BLEND_OP {
    /// Add source 1 and source 2.
    Add = 1,

    /// Subtract source 1 from source 2.
    Subtract = 2,

    /// Subtract source 2 from source 1.
    RevSubtract = 3,

    /// Find the minimum of source 1 and source 2.
    Min = 4,

    /// Find the maximum of source 1 and source 2.
    Max = 5,
}
