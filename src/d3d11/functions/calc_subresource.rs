use crate::UINT;

#[link(name = "D3D11")]
extern "system" {
    /// Calculates a subresource index for a texture.
    ///
    /// # Parameters
    ///  * `mip_slice` - A zero-based index for the mipmap level to address; 0 indicates the first,
    ///                  most detailed mipmap level.
    ///  * `array_slice` - The zero-based index for the array level to address; always use 0 for
    ///                    volume (3D) textures.
    ///  * `mip_levels` - Number of mipmap levels in the resource.
    ///
    /// # Return Value
    /// The index which equals `mip_slice + (array_slice * mip_levels)`.
    ///
    /// # Remarks
    /// A buffer is an unstructured resource and is therefore defined as containing a single
    /// subresource. APIs that take buffers do not need a subresource index. A texture on the other
    /// hand is highly structured. Each texture object may contain one or more subresources
    /// depending on the size of the array and the number of mipmap levels.
    ///
    /// For volume (3D) textures, all slices for a given mipmap level are a single subresource
    /// index.
    pub fn D3D11CalcSubresource(mip_slice: UINT, array_slice: UINT, mip_levels: UINT) -> UINT;
}
