use crate::{UINT, UINT8};

/// Describes the tile structure of a tiled resource with mipmaps.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_PACKED_MIP_DESC {
    /// Number of standard mipmaps in the tiled resource.
    pub num_standard_mips: UINT8,

    /// Number of packed mipmaps in the tiled resource.
    ///
    /// This number starts from the least detailed mipmap (either sharing tiles or using non
    /// standard tile layout). This number is 0 if no such packing is in the resource. For array
    /// surfaces, this value is the number of mipmaps that are packed for a given array slice where
    /// each array slice repeats the same packing.
    ///
    /// On Tier 2 tiled resources hardware, mipmaps that fill at least one standard shaped tile in
    /// all dimensions are not allowed to be included in the set of packed mipmaps. On Tier 1
    /// hardware, mipmaps that are an integer multiple of one standard shaped tile in all
    /// dimensions are not allowed to be included in the set of packed mipmaps. Mipmaps with at
    /// least one dimension less than the standard tile shape may or may not be packed. When a
    /// given mipmap needs to be packed, all coarser mipmaps for a given array slice are considered
    /// packed as well.
    pub num_packed_mips: UINT8,

    /// Number of tiles for the packed mipmaps in the tiled resource.
    ///
    /// If there is no packing, this value is meaningless and is set to 0. Otherwise, it is set to
    /// the number of tiles that are needed to represent the set of packed mipmaps. The pixel
    /// layout within the packed mipmaps is hardware specific. If apps define only partial mappings
    /// for the set of tiles in packed mipmaps, read and write behavior is vendor specific and
    /// undefined. For arrays, this value is only the count of packed mipmaps within the
    /// subresources for each array slice.
    pub num_tiles_for_packed_mips: UINT,

    /// Offset of the first packed tile for the resource in the overall range of tiles. If
    /// `num_packed_mips` is 0, this value is meaningless and is 0. Otherwise, it is the offset of
    /// the first packed tile for the resource in the overall range of tiles for the resource. A
    /// value of 0 for `start_tile_index_in_overall_resource` means the entire resource is packed.
    pub start_tile_index_in_overall_resource: UINT,
}

impl Default for D3D11_PACKED_MIP_DESC {
    fn default() -> Self {
        D3D11_PACKED_MIP_DESC {
            num_standard_mips: 0,
            num_packed_mips: 0,
            num_tiles_for_packed_mips: 0,
            start_tile_index_in_overall_resource: 0,
        }
    }
}
