use crate::{
    d3d11::D3D11_USAGE,
    dxgi::{DXGI_FORMAT, DXGI_SAMPLE_DESC},
    UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{
        D3D11CreateDevice, ID3D11Device, D3D11_BIND_FLAG, D3D11_CPU_ACCESS_FLAG,
        D3D11_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION, D3D11_REQ_TEXTURE2D_U_OR_V_DIMENSION,
        D3D11_REQ_TEXTURECUBE_DIMENSION, D3D11_RESOURCE_MISC_FLAG, D3D11_TEX1D_SRV,
        D3D11_TEXCUBE_ARRAY_SRV,
    },
    d3dcommon::D3D_FEATURE_LEVEL,
};

/// Describes a 2D texture.
///
/// # Remarks
/// This structure is used in a call to [`ID3D11Device::create_texture_2d`].
///
/// The device places some size restrictions (must be multiples of a minimum size) for a
/// subsampled, block compressed, or bit-format resource.
///
/// The texture size range is determined by the feature level at which you create the device and
/// not the Microsoft Direct3D interface version. For example, if you use Microsoft Direct3D 10
/// hardware at feature level 10 ([`D3D_FEATURE_LEVEL::_10_0`]) and call [`D3D11CreateDevice`] to
/// create an [`ID3D11Device`], you must constrain the maximum texture size to
/// [`D3D10_REQ_TEXTURE2D_U_OR_V_DIMENSION`] when you create your 2D texture.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEXTURE2D_DESC {
    /// Texture width (in texels). The range is from 1 to [`D3D11_REQ_TEXTURE2D_U_OR_V_DIMENSION`].
    /// For a texture cube-map, the range is from 1 to [`D3D11_REQ_TEXTURECUBE_DIMENSION`].
    pub width: UINT,

    /// Texture height (in texels). The range is from 1 to
    /// [`D3D11_REQ_TEXTURE2D_U_OR_V_DIMENSION`]. For a texture cube-map, the range is from 1 to
    /// [`D3D11_REQ_TEXTURECUBE_DIMENSION`].
    pub height: UINT,

    /// The maximum number of mipmap levels in the texture. See the remarks in [`D3D11_TEX1D_SRV`].
    /// Use 1 for a multisampled texture; or 0 to generate a full set of subtextures.
    pub mip_levels: UINT,

    /// Number of textures in the texture array. The range is from 1 to
    /// [`D3D11_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION`]. For a texture cube-map, this value is a
    /// multiple of 6 (that is, 6 times the value in the `num_cubes` member of
    /// [`D3D11_TEXCUBE_ARRAY_SRV`]), and the range is from 6 to 2046. The range is actually
    /// constrained by the feature level at which you create the rendering device.
    pub array_size: UINT,

    /// Texture format (see [`DXGI_FORMAT`]).
    pub format: DXGI_FORMAT,

    /// Structure that specifies multisampling parameters for the texture. See
    /// [`DXGI_SAMPLE_DESC`].
    pub sample_desc: DXGI_SAMPLE_DESC,

    /// Value that identifies how the texture is to be read from and written to. The most common
    /// value is [`D3D11_USAGE::Default`]; see [`D3D11_USAGE`] for all possible values.
    pub usage: D3D11_USAGE,

    /// Flags (see [`D3D11_BIND_FLAG`]) for binding to pipeline stages. The flags can be combined
    /// by a bitwise OR.
    pub bind_flags: UINT,

    /// Flags (see [`D3D11_CPU_ACCESS_FLAG`]) to specify the types of CPU access allowed. Use 0 if
    /// CPU access is not required. These flags can be combined with a bitwise OR.
    pub cpu_access_flags: UINT,

    /// Flags (see [`D3D11_RESOURCE_MISC_FLAG`]) that identify other, less common resource options.
    /// Use 0 if none of these flags apply. These flags can be combined by using a bitwise OR. For
    /// a texture cube-map, set the [`D3D11_RESOURCE_MISC_FLAG::TextureCube`] flag. Cube-map arrays
    /// (that is, `array_size` > 6) require feature level [`D3D_FEATURE_LEVEL::_10_1`] or higher.
    pub misc_flags: UINT,
}

impl Default for D3D11_TEXTURE2D_DESC {
    fn default() -> Self {
        D3D11_TEXTURE2D_DESC {
            width: 0,
            height: 0,
            mip_levels: 0,
            array_size: 0,
            format: DXGI_FORMAT::Unknown,
            sample_desc: DXGI_SAMPLE_DESC::default(),
            usage: D3D11_USAGE::Default,
            bind_flags: 0,
            cpu_access_flags: 0,
            misc_flags: 0,
        }
    }
}
