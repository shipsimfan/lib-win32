use crate::{d3d11shader::ID3D11ShaderReflection, HRESULT, LPCVOID, REFIID, SIZE_T};
use std::ffi::c_void;

#[link(name = "D3dcompiler")]
unsafe extern "system" {
    /// Gets a pointer to a reflection interface.
    ///
    /// # Parameters
    ///  * `src_data` - A pointer to source data as compiled HLSL code.
    ///  * `src_data_size` - Length of `src_data`.
    ///  * `reflector` - The address of a pointer to the [`ID3D11ShaderReflection`] interface.
    ///
    /// # Remarks
    /// Returns one of the Direct3D 11 return codes.
    ///
    /// # Remarks
    /// The inline [`D3D11Reflect`] compiler function is a wrapper for the [`D3DReflect`] compiler
    /// function. [`D3D11Reflect`] can retrieve only a [`ID3D11ShaderReflection`] interface from a
    /// shader. [`D3DReflect`] can retrieve a [`ID3D11ShaderReflection`] interface or a Direct3D 10
    /// or Direct3D 10.1 reflection interface, for example, [`ID3D10ShaderReflection`].
    ///
    /// Shader code contains metadata that can be inspected using the reflection APIs.
    #[cfg(feature = "d3d11")]
    pub fn D3D11Reflect(
        src_data: LPCVOID,
        src_data_size: SIZE_T,
        reflector: ID3D11ShaderReflection,
    ) -> HRESULT;

    /// Gets a pointer to a reflection interface.
    ///
    /// # Parameters
    ///  * `src_data` - A pointer to source data as compiled HLSL code.
    ///  * `src_data_size` - Length of `src_data`.
    ///  * `interface` - The reference GUID of the COM interface to use.
    ///  * `reflector` - A pointer to a reflection interface.
    ///
    /// # Return Value
    /// Returns one of the Direct3D 11 return codes.
    ///
    /// # Remarks
    /// Shader code contains metadata that can be inspected using the reflection APIs.
    pub fn D3DReflect(
        src_data: LPCVOID,
        src_data_size: SIZE_T,
        interface: REFIID,
        reflector: *mut *mut c_void,
    ) -> HRESULT;
}
