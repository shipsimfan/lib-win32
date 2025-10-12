use crate::{
    com_interface,
    d3d11shader::{ID3D11ShaderReflectionVariable, D3D11_SHADER_BUFFER_DESC},
    HRESULT, LPCSTR, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11shader::ID3D11ShaderReflection;

com_interface!(
    /// This shader-reflection interface provides access to a constant buffer.
    ///
    /// # Remarks
    /// To create a constant-buffer interface, call
    /// [`ID3D11ShaderReflection::get_constant_buffer_by_index`] or
    /// [`ID3D11ShaderReflection::get_constant_buffer_by_name`]. This isn't a COM interface, so you
    /// don't need to worry about reference counts or releasing the interface when you're done with
    /// it.
    pub abstract ID3D11ShaderReflectionConstantBuffer(ID3D11ShaderReflectionConstantBufferVTable/ID3D11ShaderReflectionConstantBufferTrait) {
        const IID = 0xEB62D63D-0x93DD-0x4318-0x8AE8-0xC6F83AD371B8;

        /// Get a constant-buffer description.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_SHADER_BUFFER_DESC`], which represents a
        ///             shader-buffer description.
        ///
        /// # Return Value
        /// Returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_desc(&mut self, desc: D3D11_SHADER_BUFFER_DESC) -> HRESULT;

        /// Get a shader-reflection variable by index.
        ///
        /// # Parameters
        ///  * `index` - Zero-based index.
        ///
        /// # Return Value
        /// A pointer to a shader-reflection variable interface (see
        /// [`ID3D11ShaderReflectionVariable`] Interface).
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_variable_by_index(&mut self, index: UINT) -> *mut ID3D11ShaderReflectionVariable;

        /// Get a shader-reflection variable by name.
        ///
        /// # Parameters
        ///  * `name` - Variable name.
        ///
        /// # Return Value
        /// Returns a sentinel object (end of list marker). To determine if
        /// [`ID3D11ShaderReflectionConstantBuffer::get_variable_by_name`] successfully completed,
        /// call [`ID3D11ShaderReflectionVariable::get_desc`] and check the returned [`HRESULT`];
        /// any return value other than success means that
        /// [`ID3D11ShaderReflectionConstantBuffer::get_variable_by_name`] failed.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_variable_by_name(&mut self, name: LPCSTR) -> *mut ID3D11ShaderReflectionVariable;
    }
);
