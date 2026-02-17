use crate::{d3dcommon::ID3DBlob, HRESULT, SIZE_T};

#[link(name = "D3dcompiler")]
unsafe extern "system" {
    /// Creates a buffer.
    ///
    /// # Parameters
    ///  * `size` - Number of bytes in the blob.
    ///  * `blob` - The address of a pointer to the [`ID3DBlob`] interface that is used to retrieve
    ///             the buffer.
    ///
    /// # Return Value
    /// Returns one of the Direct3D 11 return codes.
    ///
    /// # Remarks
    /// The latest `D3dcompiler_nn.dll` contains the [`D3DCreateBlob`] compiler function.
    /// Therefore, you are no longer required to create and use an arbitrary length data buffer by
    /// using the [`D3D10CreateBlob`] function that is contained in `D3d10.dll`.
    pub fn D3DCreateBlob(size: SIZE_T, blob: *mut *mut ID3DBlob) -> HRESULT;
}
