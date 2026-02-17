use crate::{d3dcommon::ID3DBlob, HRESULT, LPCVOID, SIZE_T};

#[link(name = "D3dcompiler")]
unsafe extern "system" {
    /// Gets the output signature from a compilation result.
    ///
    /// # Parameters
    ///  * `src_data` - A pointer to source data as compiled HLSL code.
    ///  * `src_data_size` - Length of `src_data`.
    ///  * `signature_blob` - A pointer to a buffer that receives the [`ID3DBlob`] interface that
    ///                       contains a compiled shader.
    ///
    /// # Remarks
    /// Returns one of the Direct3D 11 return codes.
    pub fn D3DGetOutputSignatureBlob(
        src_data: LPCVOID,
        src_data_size: SIZE_T,
        signature_blob: *mut *mut ID3DBlob,
    ) -> HRESULT;
}
