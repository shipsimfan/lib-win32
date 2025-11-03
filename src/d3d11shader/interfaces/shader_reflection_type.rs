use crate::{com_interface, d3d11shader::D3D11_SHADER_TYPE_DESC, HRESULT, LPCSTR, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11shader::ID3D11ShaderReflectionVariable, S_FALSE, S_OK};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// This shader-reflection interface provides access to variable type.
    ///
    /// # Remarks
    /// The get a shader-reflection-type interface, call
    /// [`ID3D11ShaderReflectionVariable::get_type`]. This isn't a COM interface, so you don't need
    /// to worry about reference counts or releasing the interface when you're done with it.
    pub abstract ID3D11ShaderReflectionType(ID3D11ShaderReflectionTypeVTable) {
        const IID = 0x6E6FFA6A-0x9BAE-0x4613-0xA51E-0x91652D508C21;

        /// Get the description of a shader-reflection-variable type.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a shader-type description (see [`D3D11_SHADER_TYPE_DESC`]).
        ///
        /// # Return Value
        /// Returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_desc(&mut self, desc: *mut D3D11_SHADER_TYPE_DESC) -> HRESULT;

        /// Get a shader-reflection-variable type by index.
        ///
        /// # Parameters
        ///  * `index` - Zero-based index.
        ///
        /// # Return Value
        /// A pointer to a [`ID3D11ShaderReflectionType`] Interface.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_member_type_by_index(&mut self, index: UINT) -> *mut ID3D11ShaderReflectionType;

        /// Get a shader-reflection-variable type by name.
        ///
        /// # Parameters
        ///  * `name` - Member name.
        ///
        /// # Return Value
        /// A pointer to a [`ID3D11ShaderReflectionType`] Interface.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_member_type_by_name(&mut self, name: LPCSTR) -> *mut ID3D11ShaderReflectionType;

        /// Get a shader-reflection-variable type.
        ///
        /// # Parameters
        ///  * `index` - Zero-based index.
        ///
        /// # Return Value
        /// The variable type.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_member_type_name(&mut self, index: UINT) -> LPCSTR;

        /// Indicates whether two [`ID3D11ShaderReflectionType`] Interface pointers have the same
        /// underlying type.
        ///
        /// # Parameters
        ///  * `r#type` - A pointer to a [`ID3D11ShaderReflectionType`] Interface.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if the pointers have the same underlying type; otherwise returns
        /// [`S_FALSE`].
        ///
        /// # Remarks
        /// [`ID3D11ShaderReflectionType::is_equal`] indicates whether the sources of the
        /// [`ID3D11ShaderReflectionType`] Interface pointers have the same underlying type. For
        /// example, if two [`ID3D11ShaderReflectionType`] Interface pointers were retrieved from
        /// variables, [`ID3D11ShaderReflectionType::is_equal`] can be used to see if the variables
        /// have the same type.
        ///
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn is_equal(&mut self, r#type: *mut ID3D11ShaderReflectionType) -> HRESULT;

        /// Gets the base class of a class.
        ///
        /// # Return Value
        /// Returns a pointer to a [`ID3D11ShaderReflectionType`] Interface containing the base
        /// class type. Returns [`null_mut`] if the class does not have a base class.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_sub_type(&mut self) -> *mut ID3D11ShaderReflectionType;

        /// Gets an [`ID3D11ShaderReflectionType`] Interface interface containing the variable base
        /// class type.
        ///
        /// # Return Value
        /// Returns A pointer to a [`ID3D11ShaderReflectionType`] Interface.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_base_class(&mut self) -> *mut ID3D11ShaderReflectionType;

        /// Gets the number of interfaces.
        ///
        /// # Return Value
        /// Returns the number of interfaces.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_num_interfaces(&mut self) -> UINT;

        /// Get an interface by index.
        ///
        /// # Parameters
        ///  * `index` - Zero-based index.
        ///
        /// # Return Value
        /// A pointer to a [`ID3D11ShaderReflectionType`] Interface.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_interface_by_index(&mut self, index: UINT) -> *mut ID3D11ShaderReflectionType;

        /// Indicates whether a variable is of the specified type.
        ///
        /// # Parameters
        ///  * `r#type` - A pointer to a [`ID3D11ShaderReflectionType`] Interface.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if object being queried is equal to or inherits from the type in the
        /// `type` parameter; otherwise returns [`S_FALSE`].
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn is_of_type(&mut self, r#type: *mut ID3D11ShaderReflectionType) -> HRESULT;

        /// Indicates whether a class type implements an interface.
        ///
        /// # Parameters
        ///  * `base` - A pointer to a [`ID3D11ShaderReflectionType`] Interface.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if the interface is implemented; otherwise return [`S_FALSE`].
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn implements_interface(&mut self, base: *mut ID3D11ShaderReflectionType) -> HRESULT;
    }
);
