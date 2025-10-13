use crate::{
    com_interface,
    d3d11shader::{
        ID3D11ShaderReflectionConstantBuffer, ID3D11ShaderReflectionVariable, D3D11_SHADER_DESC,
        D3D11_SHADER_INPUT_BIND_DESC, D3D11_SIGNATURE_PARAMETER_DESC,
    },
    d3dcommon::{D3D_FEATURE_LEVEL, D3D_PRIMITIVE},
    unknwn::{IUnknown, IUnknownTrait},
    BOOL, HRESULT, LPCSTR, UINT, UINT64,
};

// rustdoc imports
#[cfg(feature = "d3dcompiler")]
#[allow(unused_imports)]
use crate::d3dcompiler::D3DReflect;
#[allow(unused_imports)]
use crate::{
    d3d11::{ID3D11DeviceContext, D3D11_PRIMITIVE_TOPOLOGY},
    d3dcommon::D3D_PRIMITIVE_TOPOLOGY,
};

com_interface!(
    /// A shader-reflection interface accesses shader information.
    ///
    /// # Remarks
    /// An [`ID3D11ShaderReflection`] interface can be retrieved for a shader by using
    /// [`D3DReflect`]. The following code illustrates retrieving a [`ID3D11ShaderReflection`] from
    /// a shader.
    pub abstract ID3D11ShaderReflection(ID3D11ShaderReflectionVTable/ID3D11ShaderReflectionTrait):
        IUnknown/IUnknownTrait(unknown) {
        const IID = 0x8D536CA1-0x0CCA-0x4956-0xA837-0x786963755584;

        /// Get a shader description.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a shader description. See [`D3D11_SHADER_DESC`].
        ///
        /// # Return Value
        /// Returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_desc(&mut self, desc: *mut D3D11_SHADER_DESC) -> HRESULT;

        /// Get a constant buffer by index.
        ///
        /// # Parameters
        ///  * `index` - Zero-based index.
        ///
        /// # Return Value
        /// A pointer to a constant buffer (see [`ID3D11ShaderReflectionConstantBuffer`]
        /// Interface).
        ///
        /// # Remarks
        /// A constant buffer supplies either scalar constants or texture constants to a shader. A
        /// shader can use one or more constant buffers. For best performance, separate constants
        /// into buffers based on the frequency they are updated.
        ///
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_constant_buffer_by_index(
            &mut self,
            index: UINT
        ) -> *mut ID3D11ShaderReflectionConstantBuffer;

        /// Get a constant buffer by name.
        ///
        /// # Parameters
        ///  * `name` - The constant-buffer name.
        ///
        /// # Return Value
        /// A pointer to a constant buffer (see [`ID3D11ShaderReflectionConstantBuffer`]
        /// Interface).
        ///
        /// # Remarks
        /// A constant buffer supplies either scalar constants or texture constants to a shader. A
        /// shader can use one or more constant buffers. For best performance, separate constants
        /// into buffers based on the frequency they are updated.
        ///
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_constant_buffer_by_name(
            &mut self,
            name: LPCSTR
        ) -> *mut ID3D11ShaderReflectionConstantBuffer;

        /// Get a description of how a resource is bound to a shader.
        ///
        /// # Parameters
        ///  * `resource_index` - A zero-based resource index.
        ///  * `desc` - A pointer to an input-binding description. See
        ///             [`D3D11_SHADER_INPUT_BIND_DESC`].
        ///
        /// # Return Value
        /// Returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// A shader consists of executable code (the compiled HLSL functions) and a set of
        /// resources that supply the shader with input data. GetResourceBindingDesc gets
        /// information about how one resource in the set is bound as an input to the shader. The
        /// `resource_index` parameter specifies the index for the resource.
        ///
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_resource_binding_desc(
            &mut self,
            resource_index: UINT,
            desc: *mut D3D11_SHADER_INPUT_BIND_DESC
        ) -> HRESULT;

        /// Get an input-parameter description for a shader.
        ///
        /// # Parameters
        ///  * `parameter_index` - A zero-based parameter index.
        ///  * `desc` - A pointer to a shader-input-signature description. See
        ///             [`D3D11_SIGNATURE_PARAMETER_DESC`].
        ///
        /// # Return Value
        /// Returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// An input-parameter description is also called a shader signature. The shader signature
        /// contains information about the input parameters such as the order or parameters, their
        /// data type, and a parameter semantic.
        ///
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_input_parameter_desc(
            &mut self,
            parameter_index: UINT,
            desc: *mut D3D11_SIGNATURE_PARAMETER_DESC
        ) -> HRESULT;

        /// Get an output-parameter description for a shader.
        ///
        /// # Parameters
        ///  * `parameter_index` - A zero-based parameter index.
        ///  * `desc` - A pointer to a shader-output-parameter description. See
        ///             [`D3D11_SIGNATURE_PARAMETER_DESC`].
        ///
        /// # Return Value
        /// Returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// An output-parameter description is also called a shader signature. The shader signature
        /// contains information about the output parameters such as the order or parameters, their
        /// data type, and a parameter semantic.
        ///
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_output_parameter_desc(
            &mut self,
            parameter_index: UINT,
            desc: *mut D3D11_SIGNATURE_PARAMETER_DESC
        ) -> HRESULT;

        /// Get a patch-constant parameter description for a shader.
        ///
        /// # Parameters
        ///  * `parameter_index` - A zero-based parameter index.
        ///  * `desc` - A pointer to a shader-input-signature description. See
        ///             [`D3D11_SIGNATURE_PARAMETER_DESC`].
        ///
        /// # Return Value
        /// Returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_patch_constant_parameter_desc(
            &mut self,
            parameter_index: UINT,
            desc: *mut D3D11_SIGNATURE_PARAMETER_DESC
        ) -> HRESULT;

        /// Gets a variable by name.
        ///
        /// # Parameters
        ///  * `name` - A pointer to a string containing the variable name.
        ///
        /// # Return Value
        /// Returns a [`ID3D11ShaderReflectionVariable`] Interface interface.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_variable_by_name(&mut self, name: LPCSTR) -> *mut ID3D11ShaderReflectionVariable;

        /// Get a description of how a resource is bound to a shader.
        ///
        /// # Parameters
        ///  * `name` - The constant-buffer name of the resource.
        ///  * `desc` - A pointer to an input-binding description. See
        ///             [`D3D11_SHADER_INPUT_BIND_DESC`].
        ///
        /// # Return Value
        /// Returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// A shader consists of executable code (the compiled HLSL functions) and a set of
        /// resources that supply the shader with input data.
        /// [`ID3D11ShaderReflection::get_resource_binding_desc_by_name`] gets information about
        /// how one resource in the set is bound as an input to the shader. The Name parameter
        /// specifies the name of the resource.
        ///
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_resource_binding_desc_by_name(
            &mut self,
            name: LPCSTR,
            desc: *mut D3D11_SHADER_INPUT_BIND_DESC
        ) -> HRESULT;

        /// Gets the number of Mov instructions.
        ///
        /// # Return Value
        /// Returns the number of Mov instructions.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_mov_instruction_count(&mut self) -> UINT;

        /// Gets the number of Movc instructions.
        ///
        /// # Return Value
        /// Returns the number of Movc instructions.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_movc_instruction_count(&mut self) -> UINT;

        /// Gets the number of conversion instructions.
        ///
        /// # Return Value
        /// Returns the number of conversion instructions.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_conversion_instruction_count(&mut self) -> UINT;

        /// Gets the number of bitwise instructions.
        ///
        /// # Return Value
        /// The number of bitwise instructions.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_bitwise_instruction_count(&mut self) -> UINT;

        /// Gets the geometry-shader input-primitive description.
        ///
        /// # Return Value
        /// The input-primitive description. See [`D3D_PRIMITIVE_TOPOLOGY`],
        /// [`D3D11_PRIMITIVE_TOPOLOGY`], or [`D3D10_PRIMITIVE_TOPOLOGY`].
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_gs_input_primitive(&mut self) -> D3D_PRIMITIVE;

        /// Indicates whether a shader is a sample frequency shader.
        ///
        /// # Return Value
        /// Returns true if the shader is a sample frequency shader; otherwise returns false.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn is_sample_frequency_shader(&mut self) -> BOOL;

        /// Gets the number of interface slots in a shader.
        ///
        /// # Return Value
        /// The number of interface slots in the shader.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_num_interface_slots(&mut self) -> UINT;

        /// Gets the minimum feature level.
        ///
        /// # Parameters
        ///  * `level` - A pointer to one of the enumerated values in [`D3D_FEATURE_LEVEL`], which
        ///              represents the minimum feature level.
        ///
        /// # Return Value
        /// Returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        fn get_min_feature_level(&mut self, level: *mut D3D_FEATURE_LEVEL) -> HRESULT;

        /// Retrieves the sizes, in units of threads, of the X, Y, and Z dimensions of the shader's
        /// thread-group grid.
        ///
        /// # Parameters
        ///  * `size_x` - A pointer to the size, in threads, of the x-dimension of the thread-group
        ///               grid. The maximum size is 1024.
        ///  * `size_y` - A pointer to the size, in threads, of the y-dimension of the thread-group
        ///               grid. The maximum size is 1024.
        ///  * `size_z` - A pointer to the size, in threads, of the z-dimension of the thread-group
        ///               grid. The maximum size is 64.
        ///
        /// # Return Value
        /// Returns the total size, in threads, of the thread-group grid by calculating the product
        /// of the size of each dimension.
        ///
        /// # Remarks
        /// This method's interface is hosted in the out-of-box DLL `D3DCompiler_xx.dll`.
        ///
        /// When a compute shader is written it defines the actions of a single thread group only.
        /// If multiple thread groups are required, it is the role of the
        /// [`ID3D11DeviceContext::dispatch`] call to issue multiple thread groups.
        fn get_thread_group_size(
            &mut self,
            size_x: *mut UINT,
            size_y: *mut UINT,
            size_z: *mut UINT
        ) -> UINT;

        /// Gets a group of flags that indicates the requirements of a shader.
        ///
        /// # Return Value
        /// Gets a group of flags that indicates the requirements of a shader.
        fn get_requires_flags(&mut self) -> UINT64;
    }
);
