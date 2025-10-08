use crate::{
    d3d11::{D3D11_COMPARISON_FUNC, D3D11_FILTER, D3D11_TEXTURE_ADDRESS_MODE},
    FLOAT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_FLOAT32_MAX;

/// Describes a sampler state.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_SAMPLER_DESC {
    /// Filtering method to use when sampling a texture (see [`D3D11_FILTER`]).
    pub filter: D3D11_FILTER,

    /// Method to use for resolving a u texture coordinate that is outside the 0 to 1 range (see
    /// [`D3D11_TEXTURE_ADDRESS_MODE`]).
    pub address_u: D3D11_TEXTURE_ADDRESS_MODE,

    /// Method to use for resolving a v texture coordinate that is outside the 0 to 1 range.
    pub address_v: D3D11_TEXTURE_ADDRESS_MODE,

    /// Method to use for resolving a w texture coordinate that is outside the 0 to 1 range.
    pub address_w: D3D11_TEXTURE_ADDRESS_MODE,

    /// Offset from the calculated mipmap level. For example, if Direct3D calculates that a texture
    /// should be sampled at mipmap level 3 and `mip_lod_bias` is 2, then the texture will be
    /// sampled at mipmap level 5.
    pub mip_lod_bias: FLOAT,

    /// Clamping value used if [`D3D11_FILTER::Anisotropic`] or
    /// [`D3D11_FILTER::ComparisonAnisotropic`] is specified in `filter`. Valid values are between
    /// 1 and 16.
    pub max_anisotropy: UINT,

    /// A function that compares sampled data against existing sampled data. The function options
    /// are listed in [`D3D11_COMPARISON_FUNC`].
    pub comparison_func: D3D11_COMPARISON_FUNC,

    /// Border color to use if [`D3D11_TEXTURE_ADDRESS_MODE::Border`] is specified for `address_u`,
    /// `address_v`, or `address_w`. Range must be between 0.0 and 1.0 inclusive.
    pub border_color: [FLOAT; 4],

    /// Lower end of the mipmap range to clamp access to, where 0 is the largest and most detailed
    /// mipmap level and any level higher than that is less detailed.
    pub min_lod: FLOAT,

    /// Upper end of the mipmap range to clamp access to, where 0 is the largest and most detailed
    /// mipmap level and any level higher than that is less detailed. This value must be greater
    /// than or equal to `min_lod`. To have no upper limit on LOD set this to a large value such as
    /// [`D3D11_FLOAT32_MAX`].
    pub max_lod: FLOAT,
}

impl Default for D3D11_SAMPLER_DESC {
    fn default() -> Self {
        D3D11_SAMPLER_DESC {
            filter: D3D11_FILTER::MinMagMipLinear,
            address_u: D3D11_TEXTURE_ADDRESS_MODE::Clamp,
            address_v: D3D11_TEXTURE_ADDRESS_MODE::Clamp,
            address_w: D3D11_TEXTURE_ADDRESS_MODE::Clamp,
            mip_lod_bias: 0.,
            max_anisotropy: 1,
            comparison_func: D3D11_COMPARISON_FUNC::Never,
            border_color: [1.; 4],
            min_lod: -FLOAT::MAX,
            max_lod: FLOAT::MAX,
        }
    }
}
