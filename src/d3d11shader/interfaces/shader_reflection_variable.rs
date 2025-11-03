use crate::{com_interface, d3d11shader::ID3D11ShaderReflectionConstantBuffer, HRESULT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11shader::{ID3D11ShaderReflection, ID3D11ShaderReflectionType, D3D11_SHADER_VARIABLE_DESC},
    E_FAIL,
};

com_interface!(
    /// This shader-reflection interface provides access to a variable.
    ///
    /// # Remarks
    /// To get a shader-reflection-variable interface, call a method like
    /// [`ID3D11ShaderReflection::get_variable_by_name`]. This isn't a COM interface, so you don't
    /// need to worry about reference counts or releasing the interface when you're done with it.
    pub abstract ID3D11ShaderReflectionVariable(ID3D11ShaderReflectionVariableVTable) {
        const IID = 0x51F23923-0xF3E5-0x4BD1-0x91CB-0x606177D8DB4C;

        /// Get a shader-variable description.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a shader-variable description (see
        ///             [`D3D11_SHADER_VARIABLE_DESC`]).
        ///
        /// # Return Value
        /// Returns one of the following Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// This method can be used to determine if the [`ID3D11ShaderReflectionVariable`]
        /// Interface is valid, the method returns [`E_FAIL`] when the variable is not valid.
        ///
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_desc(&mut self, desc: *mut D3D11_SHADER_VARIABLE_DESC) -> HRESULT;

        /// Get a shader-variable type.
        ///
        /// # Return Value
        /// A pointer to a [`ID3D11ShaderReflectionType`] Interface.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_type(&mut self) -> *mut ID3D11ShaderReflectionType;

        /// This method returns the buffer of the current [`ID3D11ShaderReflectionVariable`].
        ///
        /// # Return Value
        /// Returns a pointer to the [`ID3D11ShaderReflectionConstantBuffer`] of the present
        /// [`ID3D11ShaderReflectionVariable`].
        fn get_buffer(&mut self) -> *mut ID3D11ShaderReflectionConstantBuffer;

        /// Gets the corresponding interface slot for a variable that represents an interface
        /// pointer.
        ///
        /// # Parameters
        ///  * `array_index` - Index of the array element to get the slot number for. For a
        ///                    non-array variable this value will be zero.
        ///
        /// # Return Value
        /// Returns the index of the interface in the interface array.
        ///
        /// # Remarks
        /// [`ID3D11ShaderReflectionVariable::get_interface_slot`] gets the corresponding slot in a
        /// dynamic linkage array for an interface instance. The returned slot number is used to
        /// set an interface instance to a particular class instance.
        ///
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_interface_slot(&mut self, array_index: UINT) -> UINT;
    }
);
