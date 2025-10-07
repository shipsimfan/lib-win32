use crate::{
    d3d11::{
        ID3D11Buffer, ID3D11RenderTargetView, ID3D11Resource, ID3D11ShaderResourceView,
        ID3D11Texture1D, ID3D11Texture2D, ID3D11Texture3D, ID3D11UnorderedAccessView,
        D3D11_BUFFER_DESC, D3D11_RENDER_TARGET_VIEW_DESC, D3D11_SHADER_RESOURCE_VIEW_DESC,
        D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE1D_DESC, D3D11_TEXTURE2D_DESC, D3D11_TEXTURE3D_DESC,
        D3D11_UNORDERED_ACCESS_VIEW_DESC,
    },
    immut_com_interface,
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{
        ID3D11DeviceContext, D3D11_BIND_FLAG, D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT,
        D3D11_RTV_DIMENSION, D3D11_SRV_DIMENSION, D3D11_UAV_DIMENSION, D3D11_USAGE,
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
    }
);
