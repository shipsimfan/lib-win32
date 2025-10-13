// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11DeviceContext;

/// Blend factors, which modulate values for the pixel shader and render target.
///
/// # Remarks
/// Blend operations are specified in a blend description.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_BLEND {
    /// The blend factor is (0, 0, 0, 0). No pre-blend operation.
    Zero = 1,

    /// The blend factor is (1, 1, 1, 1). No pre-blend operation.
    One = 2,

    /// The blend factor is (Rₛ, Gₛ, Bₛ, Aₛ), that is color data (RGB) from a pixel shader. No
    /// pre-blend operation.
    SrcColor = 3,

    /// The blend factor is (1 - Rₛ, 1 - Gₛ, 1 - Bₛ, 1 - Aₛ), that is color data (RGB) from a pixel
    /// shader. The pre-blend operation inverts the data, generating 1 - RGB.
    InvSrcColor = 4,

    /// The blend factor is (Aₛ, Aₛ, Aₛ, Aₛ), that is alpha data (A) from a pixel shader. No
    /// pre-blend operation.
    SrcAlpha = 5,

    /// The blend factor is (1 - Aₛ, 1 - Aₛ, 1 - Aₛ, 1 - Aₛ), that is alpha data (A) from a pixel
    /// shader. The pre-blend operation inverts the data, generating 1 - A.
    InvSrcAlpha = 6,

    /// The blend factor is (Ad, Ad, Ad, Ad), that is alpha data from a render target. No pre-blend
    /// operation.
    DestAlpha = 7,

    /// The blend factor is (1 - Ad, 1 - Ad, 1 - Ad, 1 - Ad), that is alpha data from a render
    /// target. The pre-blend operation inverts the data, generating 1 - A.
    InvDestAlpha = 8,

    /// The blend factor is (Rd, Gd, Bd, Ad), that is color data from a render target. No pre-blend
    /// operation.
    DestColor = 9,

    /// The blend factor is (1 - Rd, 1 - Gd, 1 - Bd, 1 - Ad), that is color data from a render
    /// target. The pre-blend operation inverts the data, generating 1 - RGB.
    InvDestColor = 10,

    /// The blend factor is (f, f, f, 1); where f = min(Aₛ, 1 - Ad). The pre-blend operation clamps
    /// the data to 1 or less.
    SrcAlphaStat = 11,

    /// The blend factor is the blend factor set with [`ID3D11DeviceContext::om_set_blend_state`].
    /// No pre-blend operation.
    BlendFactor = 14,

    /// The blend factor is the blend factor set with [`ID3D11DeviceContext::om_set_blend_state`].
    /// The pre-blend operation inverts the blend factor, generating 1 - blend_factor.
    InvBlendFactor = 15,

    /// The blend factor is data sources both as color data output by a pixel shader. There is no
    /// pre-blend operation. This blend factor supports dual-source color blending.
    Src1Color = 16,

    /// The blend factor is data sources both as color data output by a pixel shader. The pre-blend
    /// operation inverts the data, generating 1 - RGB. This blend factor supports dual-source
    /// color blending.
    InvSrc1Color = 17,

    /// The blend factor is data sources as alpha data output by a pixel shader. There is no
    /// pre-blend operation. This blend factor supports dual-source color blending.
    Src1Alpha = 18,

    /// The blend factor is data sources as alpha data output by a pixel shader. The pre-blend
    /// operation inverts the data, generating 1 - A. This blend factor supports dual-source color
    /// blending.
    InvSrc1Alpha = 19,
}
