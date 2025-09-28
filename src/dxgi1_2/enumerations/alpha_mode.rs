/// Identifies the alpha value, transparency behavior, of a surface.
///
/// # Remarks
/// For more information about alpha mode, see [`D2D1_ALPHA_MODE`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_ALPHA_MODE {
    /// Indicates that the transparency behavior is not specified.
    Unspecified = 0,

    /// Indicates that the transparency behavior is premultiplied. Each color is first scaled by
    /// the alpha value. The alpha value itself is the same in both straight and premultiplied
    /// alpha. Typically, no color channel value is greater than the alpha channel value. If a
    /// color channel value in a premultiplied format is greater than the alpha channel, the
    /// standard source-over blending math results in an additive blend.
    Premultiplied = 1,

    /// Indicates that the transparency behavior is not premultiplied. The alpha channel indicates
    /// the transparency of the color.
    Straight = 2,

    /// Indicates to ignore the transparency behavior.
    Ignore = 3,

    /// Forces this enumeration to compile to 32 bits in size. Without this value, some compilers
    /// would allow this enumeration to compile to a size other than 32 bits. This value is not
    /// used.
    ForceDWord = 0xFFFFFFFF,
}
