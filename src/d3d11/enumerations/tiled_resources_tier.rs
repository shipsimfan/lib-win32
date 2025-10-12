// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, D3D11_PACKED_MIP_DESC, D3D11_RESOURCE_MISC_FLAG};

/// Indicates the tier level at which tiled resources are supported.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_TILED_RESOURCES_TIER {
    /// Tiled resources are not supported.
    NotSupported = 0,

    /// Tier 1 tiled resources are supported.
    ///
    /// The device supports calls to [`ID3D11Device::create_texture_2d`] and so on with the
    /// [`D3D11_RESOURCE_MISC_FLAG::Tiled`] flag.
    ///
    /// The device supports calls to [`ID3D11Device::create_buffer`] with the
    /// [`D3D11_RESOURCE_MISC_FLAG::TilePool`] flag.
    ///
    /// If you access tiles (read or write) that are `NULL`-mapped, you get undefined behavior,
    /// which includes device-removed. Apps can map all tiles to a single "default" tile to avoid
    /// this condition.
    _1 = 1,

    /// Tier 2 tiled resources are supported.
    ///
    /// Superset of [`D3D11_TILED_RESOURCES_TIER::_1`] functionality, which includes this
    /// additional support:
    ///  - On [`D3D11_TILED_RESOURCES_TIER::_1`], if the size of a texture mipmap level is an
    ///    integer multiple of the standard tile shape for its format, it is guaranteed to be
    ///    nonpacked. On Tier_2, this guarantee is expanded to include mipmap levels whose size is
    ///    at least one standard tile shape. For more info, see [`D3D11_PACKED_MIP_DESC`].
    ///  - Shader instructions are available for clamping level-of-detail (LOD) and for obtaining
    ///    status about the shader operation.
    ///  - Reading from `NULL`-mapped tiles treat that sampled value as zero. Writes to
    ///    `NULL`-mapped tiles are discarded.
    _2 = 2,

    /// Tier 3 tiled resources are supported.
    ///
    /// Superset of [`D3D11_TILED_RESOURCES_TIER::_2`] functionality, Tier 3 is essentially Tier 2
    /// but with the additional support of Texture 3D for Tiled Resources.
    _3 = 3,
}
