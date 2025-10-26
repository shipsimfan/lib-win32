use crate::{
    d3d11::{
        ID3D11BlendState, ID3D11Buffer, ID3D11ClassLinkage, ID3D11ComputeShader,
        ID3D11DepthStencilState, ID3D11DepthStencilView, ID3D11DomainShader, ID3D11GeometryShader,
        ID3D11HullShader, ID3D11InputLayout, ID3D11PixelShader, ID3D11RasterizerState,
        ID3D11RenderTargetView, ID3D11Resource, ID3D11SamplerState, ID3D11ShaderResourceView,
        ID3D11Texture1D, ID3D11Texture2D, ID3D11Texture3D, ID3D11UnorderedAccessView,
        ID3D11VertexShader, D3D11_BLEND_DESC, D3D11_BUFFER_DESC, D3D11_DEPTH_STENCIL_DESC,
        D3D11_DEPTH_STENCIL_VIEW_DESC, D3D11_INPUT_ELEMENT_DESC, D3D11_RASTERIZER_DESC,
        D3D11_RENDER_TARGET_VIEW_DESC, D3D11_SAMPLER_DESC, D3D11_SHADER_RESOURCE_VIEW_DESC,
        D3D11_SO_DECLARATION_ENTRY, D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE1D_DESC,
        D3D11_TEXTURE2D_DESC, D3D11_TEXTURE3D_DESC, D3D11_UNORDERED_ACCESS_VIEW_DESC,
    },
    immut_com_interface,
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, SIZE_T, UINT,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3dcompiler")]
use crate::d3dcompiler::D3DGetOutputSignatureBlob;
#[allow(unused_imports)]
use crate::{
    d3d11::{
        ID3D11DeviceContext, D3D11_BIND_FLAG, D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT,
        D3D11_RTV_DIMENSION, D3D11_SO_BUFFER_SLOT_COUNT, D3D11_SO_NO_RASTERIZED_STREAM,
        D3D11_SRV_DIMENSION, D3D11_UAV_DIMENSION, D3D11_USAGE,
    },
    dxgi::DXGI_FORMAT,
    E_OUTOFMEMORY, S_FALSE, S_OK,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

immut_com_interface!(
    /// The device interface represents a virtual adapter; it is used to create resources.
    pub abstract ID3D11Device(ID3D11DeviceVTable/ID3D11DeviceTrait):
        IUnknown/IUnknownTrait(unknown) {
        const IID = 0xDB6F6DDB-0xAC77-0x4E88-0x8253-0x819DF9BBF140;

        /// Creates a buffer (vertex buffer, index buffer, or shader-constant buffer).
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_BUFFER_DESC`] structure that describes the buffer.
        ///  * `initial_data` - A pointer to a [`D3D11_SUBRESOURCE_DATA`] structure that describes
        ///                     the initialization data; use [`null`] to allocate space only (with
        ///                     the exception that it cannot be [`null`] if the usage flag is
        ///                     [`D3D11_USAGE::Immutable`]). If you don't pass anything to
        ///                     `inital_data`, the initial content of the memory for the buffer is
        ///                     undefined. In this case, you need to write the buffer content some
        ///                     other way before the resource is read.
        ///  * `buffer` - Address of a pointer to the [`ID3D11Buffer`] interface for the buffer
        ///               object created. Set this parameter to [`null`] to validate the other
        ///               input parameters ([`S_FALSE`] indicates a pass).
        ///
        /// # Return Value
        /// This method returns [`E_OUTOFMEMORY`] if there is insufficient memory to create the
        /// buffer.
        ///
        /// # Remarks
        /// For a constant buffer (`bind_flags` of [`D3D11_BUFFER_DESC`] set to
        /// [`D3D11_BIND_FLAG::ConstantBuffer`]), you must set the `byte_width` value of
        /// [`D3D11_BUFFER_DESC`] in multiples of 16, and less than or equal to
        /// [`D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT`].
        ///
        /// The Direct3D 11.1 runtime, which is available on Windows 8 and later operating
        /// systems, provides the following new functionality for [`ID3D11Device::create_buffer`]:
        ///
        /// You can create a constant buffer that is larger than the maximum constant buffer size
        /// that a shader can access (4096 32-bit*4-component constants – 64KB). When you bind the
        /// constant buffer to the pipeline (for example, via
        /// [`ID3D11DeviceContext::ps_set_constant_buffers`] or
        /// [`ID3D11DeviceContext::ps_set_constant_buffers1`]), you can define a range of the
        /// buffer that the shader can access that fits within the 4096 constant limit.
        ///
        /// The Direct3D 11.1 runtime (available in Windows 8 and later operating systems) emulates
        /// this feature for feature level 9.1, 9.2, and 9.3; therefore, this feature is supported
        /// for feature level 9.1, 9.2, and 9.3.
        ///
        /// This feature is always available on new drivers for feature level 10 and higher.
        ///
        /// On runtimes older than Direct3D 11.1, a call to [`ID3D11Device::create_buffer`] to
        /// request a constant buffer that is larger than 4096 fails.
        fn create_buffer(
            &self,
            desc: *const D3D11_BUFFER_DESC,
            initial_data: *const D3D11_SUBRESOURCE_DATA,
            buffer: *mut *mut ID3D11Buffer
        ) -> HRESULT;

        /// Creates an array of 1D textures.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_TEXTURE1D_DESC`] structure that describes a 1D
        ///             texture resource. To create a typeless resource that can be interpreted at
        ///             runtime into different, compatible formats, specify a typeless format in
        ///             the texture description. To generate mipmap levels automatically, set the
        ///             number of mipmap levels to 0.
        ///  * `initial_data` - A pointer to an array of [`D3D11_SUBRESOURCE_DATA`] structures that
        ///                     describe subresources for the 1D texture resource. Applications
        ///                     can't specify [`null`] for `initial_data` when creating
        ///                     [`D3D11_USAGE::Immutable`] resources. If the resource is
        ///                     multisampled, `initial_data` must be [`null`] because multisampled
        ///                     resources cannot be initialized with data when they are created. If
        ///                     you don't pass anything to `initial_data`, the initial content of
        ///                     the memory for the resource is undefined. In this case, you need to
        ///                     write the resource content some other way before the resource is
        ///                     read. You can determine the size of this array from values in the
        ///                     `mip_levels` and `array_size` members of the
        ///                     [`D3D11_TEXTURE1D_DESC`] structure to which `desc` points by using
        ///                     the following calculation: `mip_levels * array_size`
        ///  * `texture_1d` - A pointer to a buffer that receives a pointer to a
        ///                   [`ID3D11Texture1D`] interface for the created texture. Set this
        ///                   parameter to [`null_mut`] to validate the other input parameters (the
        ///                   method will return [`S_FALSE`] if the other input parameters pass
        ///                   validation).
        ///
        /// # Return Value
        /// If the method succeeds, the return code is [`S_OK`].
        ///
        /// # Remarks
        /// [`ID3D11Device::create_texture_1d`] creates a 1D texture resource, which can contain a
        /// number of 1D subresources. The number of textures is specified in the texture
        /// description. All textures in a resource must have the same format, size, and number of
        /// mipmap levels.
        ///
        /// All resources are made up of one or more subresources. To load data into the texture,
        /// applications can supply the data initially as an array of [`D3D11_SUBRESOURCE_DATA`]
        /// structures pointed to by `initial_data`, or they can use one of the D3DX texture
        /// functions such as [`D3DX11CreateTextureFromFile`].
        fn create_texture_1d(
            &self,
            desc: *const D3D11_TEXTURE1D_DESC,
            initial_data: *const D3D11_SUBRESOURCE_DATA,
            texture_1d: *mut *mut ID3D11Texture1D
        ) -> HRESULT;

        /// Create an array of 2D textures.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_TEXTURE2D_DESC`] structure that describes a 2D
        ///             texture resource. To create a typeless resource that can be interpreted at
        ///             runtime into different, compatible formats, specify a typeless format in
        ///             the texture description. To generate mipmap levels automatically, set the
        ///             number of mipmap levels to 0.
        ///  * `initial_data` - A pointer to an array of [`D3D11_SUBRESOURCE_DATA`] structures that
        ///                     describe subresources for the 2D texture resource. Applications
        ///                     can't specify [`null`] for `initial_data` when creating
        ///                     [`D3D11_USAGE::Immutable`] resources. If the resource is
        ///                     multisampled, `initial_data` must be [`null`] because multisampled
        ///                     resources cannot be initialized with data when they are created. If
        ///                     you don't pass anything to `initial_data`, the initial content of
        ///                     the memory for the resource is undefined. In this case, you need to
        ///                     write the resource content some other way before the resource is
        ///                     read. You can determine the size of this array from values in the
        ///                     `mip_levels` and `array_size` members of the
        ///                     [`D3D11_TEXTURE2D_DESC`] structure to which `desc` points by using
        ///                     the following calculation: `mip_levels * array_size`
        ///  * `texture_2d` - A pointer to a buffer that receives a pointer to a
        ///                   [`ID3D11Texture2D`] interface for the created texture. Set this
        ///                   parameter to [`null_mut`] to validate the other input parameters (the
        ///                   method will return [`S_FALSE`] if the other input parameters pass
        ///                   validation).
        ///
        /// # Return Value
        /// If the method succeeds, the return code is [`S_OK`].
        ///
        /// # Remarks
        /// [`ID3D11Device::create_texture_2d`] creates a 2D texture resource, which can contain a
        /// number of 2D subresources. The number of textures is specified in the texture
        /// description. All textures in a resource must have the same format, size, and number of
        /// mipmap levels.
        ///
        /// All resources are made up of one or more subresources. To load data into the texture,
        /// applications can supply the data initially as an array of [`D3D11_SUBRESOURCE_DATA`]
        /// structures pointed to by `initial_data`, or it may use one of the D3DX texture
        /// functions such as [`D3DX11CreateTextureFromFile`].
        fn create_texture_2d(
            &self,
            desc: *const D3D11_TEXTURE2D_DESC,
            initial_data: *const D3D11_SUBRESOURCE_DATA,
            texture_2d: *mut *mut ID3D11Texture2D
        ) -> HRESULT;

        /// Create a single 3D texture.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_TEXTURE3D_DESC`] structure that describes a 3D
        ///             texture resource. To create a typeless resource that can be interpreted at
        ///             runtime into different, compatible formats, specify a typeless format in
        ///             the texture description. To generate mipmap levels automatically, set the
        ///             number of mipmap levels to 0.
        ///  * `initial_data` - A pointer to an array of [`D3D11_SUBRESOURCE_DATA`] structures that
        ///                     describe subresources for the 3D texture resource. Applications
        ///                     cannot specify [`null`] for `initial_data` when creating
        ///                     [`D3D11_USAGE::Immutable`] resources. If the resource is
        ///                     multisampled, `initial_data` must be [`null`] because multisampled
        ///                     resources cannot be initialized with data when they are created. If
        ///                     you don't pass anything to `initial_data`, the initial content of
        ///                     the memory for the resource is undefined. In this case, you need to
        ///                     write the resource content some other way before the resource is
        ///                     read. You can determine the size of this array from the value in
        ///                     the `mip_levels` member of the [`D3D11_TEXTURE3D_DESC`] structure
        ///                     to which `desc` points. Arrays of 3D volume textures are not
        ///                     supported.
        ///  * `texture_3d` - A pointer to a buffer that receives a pointer to a
        ///                   [`ID3D11Texture3D`] interface for the created texture. Set this
        ///                   parameter to [`null_mut`] to validate the other input parameters (the
        ///                   method will return [`S_FALSE`] if the other input parameters pass
        ///                   validation).
        ///
        /// # Return Value
        /// If the method succeeds, the return code is [`S_OK`].
        ///
        /// # Remarks
        /// [`ID3D11Device::create_texture_3d`] creates a 3D texture resource, which can contain a
        /// number of 3D subresources. The number of textures is specified in the texture
        /// description. All textures in a resource must have the same format, size, and number of
        /// mipmap levels.
        ///
        /// All resources are made up of one or more subresources. To load data into the texture,
        /// applications can supply the data initially as an array of [`D3D11_SUBRESOURCE_DATA`]
        /// structures pointed to by `initial_data`, or they can use one of the D3DX texture
        /// functions such as [`D3DX11CreateTextureFromFile`].
        fn create_texture_3d(
            &self,
            desc: *const D3D11_TEXTURE3D_DESC,
            initial_data: *const D3D11_SUBRESOURCE_DATA,
            texture_3d: *mut *mut ID3D11Texture3D
        ) -> HRESULT;

        /// Create a shader-resource view for accessing data in a resource.
        ///
        /// # Parameters
        ///  * `resource` - Pointer to the resource that will serve as input to a shader. This
        ///                 resource must have been created with the
        ///                 [`D3D11_BIND_FLAG::ShaderResource`] flag.
        ///  * `desc` - Pointer to a shader-resource view description (see
        ///             [`D3D11_SHADER_RESOURCE_VIEW_DESC`]). Set this parameter to [`null`] to
        ///             create a view that accesses the entire resource (using the format the
        ///             resource was created with).
        ///  * `sr_view` - Address of a pointer to an [`ID3D11ShaderResourceView`]. Set this
        ///                parameter to [`null_mut`] to validate the other input parameters (the
        ///                method will return [`S_FALSE`] if the other input parameters pass
        ///                validation).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// A resource is made up of one or more subresources; a view identifies which subresources
        /// to allow the pipeline to access. In addition, each resource is bound to the pipeline
        /// using a view. A shader-resource view is designed to bind any buffer or texture resource
        /// to the shader stages using the following API methods:
        /// [`ID3D11DeviceContext::vs_set_shader_resources`],
        /// [`ID3D11DeviceContext::gs_set_shader_resources`] and
        /// [`ID3D11DeviceContext::ps_set_shader_resources`].
        ///
        /// Because a view is fully typed, this means that typeless resources become fully typed
        /// when bound to the pipeline.
        ///
        /// The Direct3D 11.1 runtime, which is available starting with Windows 8, allows you to
        /// use [`ID3D11Device::create_shader_resource_view`] for the following new purpose.
        ///
        /// You can create shader-resource views of video resources so that Direct3D shaders can
        /// process those shader-resource views. These video resources are either Texture 2D or
        /// Texture 2D Array. The value in the `view_dimension` member of the
        /// [`D3D11_SHADER_RESOURCE_VIEW_DESC`] structure for a created shader-resource view must
        /// match the type of video resource, [`D3D11_SRV_DIMENSION::Texture2D`] for Texture 2D and
        /// [`D3D11_SRV_DIMENSION::Texture2DArray`] for Texture 2D Array. Additionally, the format
        /// of the underlying video resource restricts the formats that the view can use. The video
        /// resource format values on the [`DXGI_FORMAT`] reference page specify the format values
        /// that views are restricted to.
        ///
        /// The runtime read+write conflict prevention logic (which stops a resource from being
        /// bound as an SRV and RTV or UAV at the same time) treats views of different parts of the
        /// same video surface as conflicting for simplicity. Therefore, the runtime does not allow
        /// an application to read from luma while the application simultaneously renders to chroma
        /// in the same surface even though the hardware might allow these simultaneous operations.
        fn create_shader_resource_view(
            &self,
            resource: *mut ID3D11Resource,
            desc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
            sr_view: *mut *mut ID3D11ShaderResourceView
        ) -> HRESULT;

        /// Creates a view for accessing an unordered access resource.
        ///
        /// # Parameters
        ///  * `resource` - Pointer to an [`ID3D11Resource`] that represents a resources that will
        ///                 serve as an input to a shader.
        ///  * `desc` - Pointer to an [`D3D11_UNORDERED_ACCESS_VIEW_DESC`] that represents a
        ///             shader-resource view description. Set this parameter to [`null`] to create
        ///             a view that accesses the entire resource (using the format the resource was
        ///             created with).
        ///  * `ua_view` - Address of a pointer to an [`ID3D11UnorderedAccessView`] that represents
        ///                an unordered-access view. Set this parameter to [`null_mut`] to validate
        ///                the other input parameters (the method will return [`S_FALSE`] if the
        ///                other input parameters pass validation).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// The Direct3D 11.1 runtime, which is available starting with Windows 8, allows you to
        /// use [`ID3D11Device::create_unordered_access_view`] for the following new purpose.
        ///
        /// You can create unordered-access views of video resources so that Direct3D shaders can
        /// process those unordered-access views. These video resources are either Texture 2D or
        /// Texture 2D Array. The value in the `view_dimension` member of the
        /// [`D3D11_UNORDERED_ACCESS_VIEW_DESC`] structure for a created unordered-access view must
        /// match the type of video resource, [`D3D11_UAV_DIMENSION::Texture2D`] for Texture 2D and
        /// [`D3D11_UAV_DIMENSION::Texture2DArray`] for Texture 2D Array. Additionally, the format
        /// of the underlying video resource restricts the formats that the view can use. The video
        /// resource format values on the [`DXGI_FORMAT`] reference page specify the format values
        /// that views are restricted to.
        ///
        /// The runtime read+write conflict prevention logic (which stops a resource from being
        /// bound as an SRV and RTV or UAV at the same time) treats views of different parts of the
        /// same video surface as conflicting for simplicity. Therefore, the runtime does not allow
        /// an application to read from luma while the application simultaneously renders to chroma
        /// in the same surface even though the hardware might allow these simultaneous operations.
        fn create_unordered_access_view(
            &self,
            resource: *mut ID3D11Resource,
            desc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
            ua_view: *mut *mut ID3D11UnorderedAccessView
        ) -> HRESULT;

        /// Creates a render-target view for accessing resource data.
        ///
        /// # Parameters
        ///  * `resource` - Pointer to a [`ID3D11Resource`] that represents a render target. This
        ///                 resource must have been created with the
        ///                 [`D3D11_BIND_FLAG::RenderTarget`] flag.
        ///  * `desc` - Pointer to a [`D3D11_RENDER_TARGET_VIEW_DESC`] that represents a
        ///             render-target view description. Set this parameter to [`null`] to create a
        ///             view that accesses all of the subresources in mipmap level 0.
        ///  * `rt_view` - Address of a pointer to an [`ID3D11RenderTargetView`]. Set this
        ///                parameter to [`null_mut`] to validate the other input parameters (the
        ///                method will return [`S_FALSE`] if the other input parameters pass
        ///                validation).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// A render-target view can be bound to the output-merger stage by calling
        /// [`ID3D11DeviceContext::om_set_render_targets`].
        ///
        /// The Direct3D 11.1 runtime, which is available starting with Windows 8, allows you to
        /// use [`ID3D11Device::create_render_target_view`] for the following new purpose.
        ///
        /// You can create render-target views of video resources so that Direct3D shaders can
        /// process those render-target views. These video resources are either Texture 2D or
        /// Texture 2D Array. The value in the `view_dimension` member of the
        /// [`D3D11_RENDER_TARGET_VIEW_DESC`] structure for a created render-target view must match
        /// the type of video resource, [`D3D11_RTV_DIMENSION::Texture2D`] for Texture 2D and
        /// [`D3D11_RTV_DIMENSION::Texture2DArray`] for Texture 2D Array. Additionally, the format
        /// of the underlying video resource restricts the formats that the view can use. The video
        /// resource format values on the [`DXGI_FORMAT`] reference page specify the format values
        /// that views are restricted to.
        ///
        /// The runtime read+write conflict prevention logic (which stops a resource from being
        /// bound as an SRV and RTV or UAV at the same time) treats views of different parts of the
        /// same video surface as conflicting for simplicity. Therefore, the runtime does not allow
        /// an application to read from luma while the application simultaneously renders to chroma
        /// in the same surface even though the hardware might allow these simultaneous operations.
        fn create_render_target_view(
            &self,
            resource: *mut ID3D11Resource,
            desc: *const D3D11_RENDER_TARGET_VIEW_DESC,
            rt_view: *mut *mut ID3D11RenderTargetView
        ) -> HRESULT;

        /// Create a depth-stencil view for accessing resource data.
        ///
        /// # Parameters
        ///  * `resource` - Pointer to the resource that will serve as the depth-stencil surface.
        ///                 This resource must have been created with the
        ///                 [`D3D11_BIND_FLAG::DepthStencil`] flag.
        ///  * `desc` - Pointer to a depth-stencil-view description (see
        ///             [`D3D11_DEPTH_STENCIL_VIEW_DESC`]). Set this parameter to [`null`] to
        ///             create a view that accesses mipmap level 0 of the entire resource (using
        ///             the format the resource was created with).
        ///  * `depth_stencil_view` - Address of a pointer to an [`ID3D11DepthStencilView`]. Set
        ///                           this parameter to [`null_mut`] to validate the other input
        ///                           parameters (the method will return [`S_FALSE`] if the other
        ///                           input parameters pass validation).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// A depth-stencil view can be bound to the output-merger stage by calling
        /// [`ID3D11DeviceContext::om_set_render_targets`].
        fn create_depth_stencil_view(
            &self,
            resource: *mut ID3D11Resource,
            desc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
            depth_stencil_view: *mut *mut ID3D11DepthStencilView
        ) -> HRESULT;

        /// Create an input-layout object to describe the input-buffer data for the input-assembler
        /// stage.
        ///
        /// # Parameters
        ///  * `input_element_descs` - An array of the input-assembler stage input data types; each
        ///                            type is described by an element description (see
        ///                            [`D3D11_INPUT_ELEMENT_DESC`]).
        ///  * `num_elements` - The number of input-data types in the array of input-elements.
        ///  * `shader_bytecode_with_input_signature` - A pointer to the compiled shader. The
        ///                                             compiled shader code contains a input
        ///                                             signature which is validated against the
        ///                                             array of elements.
        ///  * `bytecode_length` - Size of the compiled shader.
        ///  * `input_layout` - A pointer to the input-layout object created (see
        ///                     [`ID3D11InputLayout`]). To validate the other input parameters, set
        ///                     this pointer to be [`null_mut`] and verify that the method returns
        ///                     [`S_FALSE`].
        ///
        /// # Return Value
        /// If the method succeeds, the return code is [`S_OK`].
        ///
        /// # Remarks
        /// After creating an input layout object, it must be bound to the input-assembler stage
        /// before calling a draw API.
        ///
        /// Once an input-layout object is created from a shader signature, the input-layout object
        /// can be reused with any other shader that has an identical input signature (semantics
        /// included). This can simplify the creation of input-layout objects when you are working
        /// with many shaders with identical inputs.
        ///
        /// If a data type in the input-layout declaration does not match the data type in a
        /// shader-input signature, [`ID3D11Device::create_input_layout`] will generate a warning
        /// during compilation. The warning is simply to call attention to the fact that the data
        /// may be reinterpreted when read from a register. You may either disregard this warning
        /// (if reinterpretation is intentional) or make the data types match in both declarations
        /// to eliminate the warning.
        fn create_input_layout(
            &self,
            input_element_descs: *const D3D11_INPUT_ELEMENT_DESC,
            num_elements: UINT,
            shader_bytecode_with_input_signature: *const c_void,
            bytecode_length: SIZE_T,
            input_layout: *mut *mut ID3D11InputLayout
        ) -> HRESULT;

        /// Create a vertex-shader object from a compiled shader.
        ///
        /// # Parameters
        ///  * `shader_bytecode` - A pointer to the compiled shader.
        ///  * `bytecode_length` - Size of the compiled vertex shader.
        ///  * `class_linkage` - A pointer to a class linkage interface (see
        ///                      [`ID3D11ClassLinkage`]); the value can be [`null_mut`].
        ///  * `vertex_shader` - Address of a pointer to a [`ID3D11VertexShader`] interface. If
        ///                      this is [`null_mut`], all other parameters will be validated, and
        ///                      if all parameters pass validation this API will return [`S_FALSE`]
        ///                      instead of [`S_OK`].
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// The Direct3D 11.1 runtime, which is available starting with Windows 8, provides the
        /// following new functionality for [`ID3D11Device::create_vertex_shader`].
        ///
        /// The following shader model 5.0 instructions are available to just pixel shaders and
        /// compute shaders in the Direct3D 11.0 runtime. For the Direct3D 11.1 runtime, because
        /// unordered access views (UAV) are available at all shader stages, you can use these
        /// instructions in all shader stages.
        ///
        /// Therefore, if you use the following shader model 5.0 instructions in a vertex shader,
        /// you can successfully pass the compiled vertex shader to `shader_bytecode`. That is, the
        /// call to [`ID3D11Device::create_vertex_shader`] succeeds.
        ///
        /// If you pass a compiled shader to `shader_bytecode` that uses any of the following
        /// instructions on a device that doesn’t support UAVs at every shader stage (including
        /// existing drivers that are not implemented to support UAVs at every shader stage),
        /// [`ID3D11Device::create_vertex_shader`] fails. [`ID3D11Device::create_vertex_shader`]
        /// also fails if the shader tries to use a UAV slot beyond the set of UAV slots that the
        /// hardware supports.
        ///
        ///  - `dcl_uav_typed`
        ///  - `dcl_uav_raw`
        ///  - `dcl_uav_structured`
        ///  - `ld_raw`
        ///  - `ld_structured`
        ///  - `ld_uav_typed`
        ///  - `store_raw`
        ///  - `store_structured`
        ///  - `store_uav_typed`
        ///  - `sync_uglobal`
        ///  - All atomics and immediate atomics (for example, `atomic_and` and `imm_atomic_and`)
        fn create_vertex_shader(
            &self,
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *mut ID3D11ClassLinkage,
            vertex_shader: *mut *mut ID3D11VertexShader
        ) -> HRESULT;

        /// Create a geometry shader.
        ///
        /// # Parameters
        ///  * `shader_bytecode` - A pointer to the compiled shader.
        ///  * `bytecode_length` - Size of the compiled geometry shader.
        ///  * `class_linkage` - A pointer to a class linkage interface (see
        ///                      [`ID3D11ClassLinkage`]); the value can be [`null_mut`].
        ///  * `geometry_shader` - Address of a pointer to a [`ID3D11GeometryShader`] interface. If
        ///                        this is [`null_mut`], all other parameters will be validated,
        ///                        and if all parameters pass validation this API will return
        ///                        [`S_FALSE`] instead of [`S_OK`].
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// After it is created, the shader can be set to the device by calling
        /// [`ID3D11DeviceContext::gs_set_shader`].
        ///
        /// The Direct3D 11.1 runtime, which is available starting with Windows 8, provides the
        /// following new functionality for [`ID3D11Device::create_geometry_shader`].
        ///
        /// The following shader model 5.0 instructions are available to just pixel shaders and
        /// compute shaders in the Direct3D 11.0 runtime. For the Direct3D 11.1 runtime, because
        /// unordered access views (UAV) are available at all shader stages, you can use these
        /// instructions in all shader stages.
        ///
        /// Therefore, if you use the following shader model 5.0 instructions in a geometry shader,
        /// you can successfully pass the compiled geometry shader to `shader_bytecode`. That is,
        /// the call to [`ID3D11Device::create_geometry_shader`] succeeds.
        ///
        /// If you pass a compiled shader to `shader_bytecode` that uses any of the following
        /// instructions on a device that doesn’t support UAVs at every shader stage (including
        /// existing drivers that are not implemented to support UAVs at every shader stage),
        /// [`ID3D11Device::create_geometry_shader`] fails.
        /// [`ID3D11Device::create_geometry_shader`] also fails if the shader tries to use a UAV
        /// slot beyond the set of UAV slots that the hardware supports.
        ///
        ///  - `dcl_uav_typed`
        ///  - `dcl_uav_raw`
        ///  - `dcl_uav_structured`
        ///  - `ld_raw`
        ///  - `ld_structured`
        ///  - `ld_uav_typed`
        ///  - `store_raw`
        ///  - `store_structured`
        ///  - `store_uav_typed`
        ///  - `sync_uglobal`
        ///  - All atomics and immediate atomics (for example, `atomic_and` and `imm_atomic_and`)
        fn create_geometry_shader(
            &self,
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *mut ID3D11ClassLinkage,
            geometry_shader: *mut *mut ID3D11GeometryShader
        ) -> HRESULT;

        /// Creates a geometry shader that can write to streaming output buffers.
        ///
        /// # Parameters
        ///  * `shader_bytecode` - A pointer to the compiled geometry shader for a standard
        ///                        geometry shader plus stream output. To create the stream output
        ///                        without using a geometry shader, pass a pointer to the output
        ///                        signature for the prior stage. To obtain this output signature,
        ///                        call the [`D3DGetOutputSignatureBlob`] compiler function. You
        ///                        can also pass a pointer to the compiled shader for the prior
        ///                        stage (for example, the vertex-shader stage or domain-shader
        ///                        stage). This compiled shader provides the output signature for
        ///                        the data.
        ///  * `bytecode_length` - Size of the compiled geometry shader.
        ///  * `so_declaration` - Pointer to a [`D3D11_SO_DECLARATION_ENTRY`] array. Cannot be
        ///                       [`null`] if `num_entries > 0`.
        ///  * `num_entries` - The number of entries in the stream output declaration ( ranges from
        ///                    0 to `D3D11_SO_STREAM_COUNT * D3D11_SO_OUTPUT_COMPONENT_COUNT`).
        ///  * `buffer_strides` - An array of buffer strides; each stride is the size of an element
        ///                       for that buffer.
        ///  * `num_strides` - The number of strides (or buffers) in `buffer_strides` (ranges from
        ///                    0 to [`D3D11_SO_BUFFER_SLOT_COUNT`]).
        ///  * `rasterized_stream` - The index number of the stream to be sent to the rasterizer
        ///                          stage (ranges from 0 to `D3D11_SO_STREAM_COUNT - 1`). Set to
        ///                          [`D3D11_SO_NO_RASTERIZED_STREAM`] if no stream is to be
        ///                          rasterized.
        ///  * `class_linkage` - A pointer to a class linkage interface (see
        ///                      [`ID3D11ClassLinkage`]); the value can be [`null_mut`].
        ///  * `geometry_shader` - Address of a pointer to an [`ID3D11GeometryShader`] interface,
        ///                        representing the geometry shader that was created. Set this to
        ///                        [`null_mut`] to validate the other parameters; if validation
        ///                        passes, the method will return [`S_FALSE`] instead of [`S_OK`].
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// The Direct3D 11.1 runtime, which is available starting with Windows 8, provides the
        /// following new functionality for
        /// [`ID3D11Device::create_geometry_shader_with_stream_output`].
        ///
        /// The following shader model 5.0 instructions are available to just pixel shaders and
        /// compute shaders in the Direct3D 11.0 runtime. For the Direct3D 11.1 runtime, because
        /// unordered access views (UAV) are available at all shader stages, you can use these
        /// instructions in all shader stages.
        ///
        /// Therefore, if you use the following shader model 5.0 instructions in a geometry shader,
        /// you can successfully pass the compiled geometry shader to `shader_bytecode`. That is,
        /// the call to [`ID3D11Device::create_geometry_shader_with_stream_output`] succeeds.
        ///
        /// If you pass a compiled shader to `shader_bytecode` that uses any of the following
        /// instructions on a device that doesn’t support UAVs at every shader stage (including
        /// existing drivers that are not implemented to support UAVs at every shader stage),
        /// [`ID3D11Device::create_geometry_shader_with_stream_output`] fails.
        /// [`ID3D11Device::create_geometry_shader_with_stream_output`] also fails if the shader
        /// tries to use a UAV slot beyond the set of UAV slots that the hardware supports.
        ///
        ///  - `dcl_uav_typed`
        ///  - `dcl_uav_raw`
        ///  - `dcl_uav_structured`
        ///  - `ld_raw`
        ///  - `ld_structured`
        ///  - `ld_uav_typed`
        ///  - `store_raw`
        ///  - `store_structured`
        ///  - `store_uav_typed`
        ///  - `sync_uglobal`
        ///  - All atomics and immediate atomics (for example, atomic_and and imm_atomic_and)
        fn create_geometry_shader_with_stream_output(
            &self,
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            so_declaration: *const D3D11_SO_DECLARATION_ENTRY,
            num_entries: UINT,
            buffer_strides: *const UINT,
            num_strides: UINT,
            rasterized_stream: UINT,
            class_linkage: *mut ID3D11ClassLinkage,
            geometry_shader: *mut *mut ID3D11GeometryShader
        ) -> HRESULT;

        /// Create a pixel shader.
        ///
        /// # Parameters
        ///  * `shader_bytecode` - A pointer to the compiled shader.
        ///  * `bytecode_length` - Size of the compiled pixel shader.
        ///  * `class_linkage` - A pointer to a class linkage interface (see
        ///                      [`ID3D11ClassLinkage`]); the value can be [`null_mut`].
        ///  * `pixel_shader` - Address of a pointer to a [`ID3D11PixelShader`] interface. If this
        ///                     is [`null_mut`], all other parameters will be validated, and if all
        ///                     parameters pass validation this API will return [`S_FALSE`] instead
        ///                     of [`S_OK`].
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// After creating the pixel shader, you can set it to the device using
        /// [`ID3D11DeviceContext::ps_set_shader`].
        fn create_pixel_shader(
            &self,
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *mut ID3D11ClassLinkage,
            pixel_shader: *mut *mut ID3D11PixelShader
        ) -> HRESULT;

        /// Create a hull shader.
        ///
        /// # Parameters
        ///  * `shader_bytecode` - A pointer to a compiled shader.
        ///  * `bytecode_length` - Size of the compiled shader.
        ///  * `class_linkage` - A pointer to a class linkage interface (see
        ///                      [`ID3D11ClassLinkage`]); the value can be [`null_mut`].
        ///  * `hull_shader` - Address of a pointer to a [`ID3D11HullShader`] interface.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// The Direct3D 11.1 runtime, which is available starting with Windows 8, provides the
        /// following new functionality for [`ID3D11Device::create_hull_shader`].
        ///
        /// The following shader model 5.0 instructions are available to just pixel shaders and
        /// compute shaders in the Direct3D 11.0 runtime. For the Direct3D 11.1 runtime, because
        /// unordered access views (UAV) are available at all shader stages, you can use these
        /// instructions in all shader stages.
        ///
        /// Therefore, if you use the following shader model 5.0 instructions in a hull shader, you
        /// can successfully pass the compiled hull shader to `shader_bytecode`. That is, the call
        /// to [`ID3D11Device::create_hull_shader`] succeeds.
        ///
        /// If you pass a compiled shader to `shader_bytecode` that uses any of the following
        /// instructions on a device that doesn’t support UAVs at every shader stage (including
        /// existing drivers that are not implemented to support UAVs at every shader stage),
        /// [`ID3D11Device::create_hull_shader`] fails. [`ID3D11Device::create_hull_shader`] also
        /// fails if the shader tries to use a UAV slot beyond the set of UAV slots that the
        /// hardware supports.
        ///
        ///  - `dcl_uav_typed`
        ///  - `dcl_uav_raw`
        ///  - `dcl_uav_structured`
        ///  - `ld_raw`
        ///  - `ld_structured`
        ///  - `ld_uav_typed`
        ///  - `store_raw`
        ///  - `store_structured`
        ///  - `store_uav_typed`
        ///  - `sync_uglobal`
        ///  - All atomics and immediate atomics (for example, `atomic_and` and `imm_atomic_and`)
        fn create_hull_shader(
            &self,
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *mut ID3D11ClassLinkage,
            hull_shader: *mut *mut ID3D11HullShader
        ) -> HRESULT;

        /// Create a domain shader.
        ///
        /// # Parameters
        ///  * `shader_bytecode` - A pointer to a compiled shader.
        ///  * `bytecode_length` - Size of the compiled shader.
        ///  * `class_linkage` - A pointer to a class linkage interface (see
        ///                      [`ID3D11ClassLinkage`]); the value can be [`null_mut`].
        ///  * `domain_shader` - Address of a pointer to a [`ID3D11DomainShader`] interface. If
        ///                      this is [`null_mut`], all other parameters will be validated, and
        ///                      if all parameters pass validation this API will return [`S_FALSE`]
        ///                      instead of [`S_OK`].
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// The Direct3D 11.1 runtime, which is available starting with Windows 8, provides the
        /// following new functionality for [`ID3D11Device::create_domain_shader`].
        ///
        /// The following shader model 5.0 instructions are available to just pixel shaders and
        /// compute shaders in the Direct3D 11.0 runtime. For the Direct3D 11.1 runtime, because
        /// unordered access views (UAV) are available at all shader stages, you can use these
        /// instructions in all shader stages.
        ///
        /// Therefore, if you use the following shader model 5.0 instructions in a domain shader,
        /// you can successfully pass the compiled domain shader to `shader_bytecode`. That is, the
        /// call to [`ID3D11Device::create_domain_shader`] succeeds.
        ///
        /// If you pass a compiled shader to `shader_bytecode` that uses any of the following
        /// instructions on a device that doesn’t support UAVs at every shader stage (including
        /// existing drivers that are not implemented to support UAVs at every shader stage),
        /// [`ID3D11Device::create_domain_shader`] fails. [`ID3D11Device::create_domain_shader`]
        /// also fails if the shader tries to use a UAV slot beyond the set of UAV slots that the
        /// hardware supports.
        ///
        ///  - `dcl_uav_typed`
        ///  - `dcl_uav_raw`
        ///  - `dcl_uav_structured`
        ///  - `ld_raw`
        ///  - `ld_structured`
        ///  - `ld_uav_typed`
        ///  - `store_raw`
        ///  - `store_structured`
        ///  - `store_uav_typed`
        ///  - `sync_uglobal`
        ///  - All atomics and immediate atomics (for example, `atomic_and` and `imm_atomic_and`)
        fn create_domain_shader(
            &self,
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *mut ID3D11ClassLinkage,
            domain_shader: *mut *mut ID3D11DomainShader
        ) -> HRESULT;

        /// Create a compute shader.
        ///
        /// # Parameters
        ///  * `shder_bytecode` - A pointer to a compiled shader.
        ///  * `bytecode_length` - Size of the compiled shader in `shader_bytecode`.
        ///  * `class_linkage` - A pointer to a [`ID3D11ClassLinkage`], which represents class
        ///                      linkage interface; the value can be [`null_mut`].
        ///  * `compute_shader` - Address of a pointer to an [`ID3D11ComputeShader`] interface. If
        ///                       this is [`null_mut`], all other parameters will be validated; if
        ///                       validation passes, [`ID3D11Device::create_compute_shader`]
        ///                       returns [`S_FALSE`] instead of [`S_OK`].
        ///
        /// # Return Value
        /// This method returns [`E_OUTOFMEMORY`] if there is insufficient memory to create the
        /// compute shader.
        fn create_compute_shader(
            &self,
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *mut ID3D11ClassLinkage,
            compute_shader: *mut *mut ID3D11ComputeShader
        ) -> HRESULT;

        /// Creates class linkage libraries to enable dynamic shader linkage.
        ///
        /// # Parameters
        ///  * `linkage` - A pointer to a class-linkage interface pointer (see
        ///                [`ID3D11ClassLinkage`]).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// The [`ID3D11ClassLinkage`] interface returned in `linkage` is associated with a shader
        /// by passing it as a parameter to one of the [`ID3D11Device`] create shader methods such
        /// as [`ID3D11Device::create_pixel_shader`].
        fn create_class_linkage(&self, linkage: *mut *mut ID3D11ClassLinkage) -> HRESULT;

        /// Create a blend-state object that encapsulates blend state for the output-merger stage.
        ///
        /// # Parameters
        ///  * `blend_state_desc` - Pointer to a blend-state description (see
        ///                         [`D3D11_BLEND_DESC`]).
        ///  * `blend_state` - Address of a pointer to the blend-state object created (see
        ///                    [`ID3D11BlendState`]).
        ///
        /// # Return Value
        /// This method returns [`E_OUTOFMEMORY`] if there is insufficient memory to create the
        /// blend-state object.
        ///
        /// # Remarks
        /// An application can create up to 4096 unique blend-state objects. For each object
        /// created, the runtime checks to see if a previous object has the same state. If such a
        /// previous object exists, the runtime will return a pointer to previous instance instead
        /// of creating a duplicate object.
        fn create_blend_state(
            &self,
            blend_state_desc: *const D3D11_BLEND_DESC,
            blend_state: *mut *mut ID3D11BlendState
        ) -> HRESULT;

        /// Create a depth-stencil state object that encapsulates depth-stencil test information
        /// for the output-merger stage.
        ///
        /// # Parameters
        ///  * `depth_stencil_desc` - Pointer to a depth-stencil state description (see
        ///                           [`D3D11_DEPTH_STENCIL_DESC`]).
        ///  * `depth_stencil_state` - Address of a pointer to the depth-stencil state object
        ///                            created (see [`ID3D11DepthStencilState`]).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// 4096 unique depth-stencil state objects can be created on a device at a time.
        ///
        /// If an application attempts to create a depth-stencil-state interface with the same
        /// state as an existing interface, the same interface will be returned and the total
        /// number of unique depth-stencil state objects will stay the same.
        fn create_depth_stencil_state(
            &self,
            depth_stencil_desc: *const D3D11_DEPTH_STENCIL_DESC,
            depth_stencil_state: *mut *mut ID3D11DepthStencilState
        ) -> HRESULT;

        /// Create a rasterizer state object that tells the rasterizer stage how to behave.
        ///
        /// # Parameters
        ///  * `rasterizer_desc` - Pointer to a rasterizer state description (see
        ///                        [`D3D11_RASTERIZER_DESC`]).
        ///  * `rasterizer_state` - Address of a pointer to the rasterizer state object created
        ///                         (see [`ID3D11RasterizerState`]).
        ///
        /// # Return Value
        /// This method returns [`E_OUTOFMEMORY`] if there is insufficient memory to create the
        /// rasterizer-state object.
        ///
        /// # Remarks
        /// 4096 unique rasterizer state objects can be created on a device at a time.
        ///
        /// If an application attempts to create a rasterizer-state interface with the same state
        /// as an existing interface, the same interface will be returned and the total number of
        /// unique rasterizer state objects will stay the same.
        fn create_rasterizer_state(
            &self,
            rasterizer_desc: *const D3D11_RASTERIZER_DESC,
            rasterizer_state: *mut *mut ID3D11RasterizerState
        ) -> HRESULT;

        /// Create a sampler-state object that encapsulates sampling information for a texture.
        ///
        /// # Parameters
        ///  * `sampler_desc` - Pointer to a sampler state description (see
        ///                     [`D3D11_SAMPLER_DESC`]).
        ///  * `sampler_state` - Address of a pointer to the sampler state object created (see
        ///                      [`ID3D11SamplerState`]).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// 4096 unique sampler state objects can be created on a device at a time.
        ///
        /// If an application attempts to create a sampler-state interface with the same state as
        /// an existing interface, the same interface will be returned and the total number of
        /// unique sampler state objects will stay the same.
        fn create_sampler_state(
            &self,
            sampler_desc: *const D3D11_SAMPLER_DESC,
            sampler_state: *mut *mut ID3D11SamplerState
        ) -> HRESULT;
    }
);
