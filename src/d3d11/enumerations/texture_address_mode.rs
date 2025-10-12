// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_SAMPLER_DESC;

/// Identify a technique for resolving texture coordinates that are outside of the boundaries of a
/// texture.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_TEXTURE_ADDRESS_MODE {
    /// Tile the texture at every (u,v) integer junction. For example, for u values between 0 and
    /// 3, the texture is repeated three times.
    Wrap = 1,

    /// Flip the texture at every (u,v) integer junction. For u values between 0 and 1, for
    /// example, the texture is addressed normally; between 1 and 2, the texture is flipped
    /// (mirrored); between 2 and 3, the texture is normal again; and so on.
    Mirror = 2,

    /// Texture coordinates outside the range [0.0, 1.0] are set to the texture color at 0.0 or
    /// 1.0, respectively.
    Clamp = 3,

    /// Texture coordinates outside the range [0.0, 1.0] are set to the border color specified in
    /// [`D3D11_SAMPLER_DESC`] or HLSL code.
    Border = 4,

    /// Similar to [`D3D11_TEXTURE_ADDRESS_MODE::Mirror`] and
    /// [`D3D11_TEXTURE_ADDRESS_MODE::Clamp`]. Takes the absolute value of the texture coordinate
    /// (thus, mirroring around 0), and then clamps to the maximum value.
    MirrorOnce = 5,
}
