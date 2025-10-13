use crate::{
    com_interface,
    d3d11::{
        ID3D11Asynchronous, ID3D11BlendState, ID3D11Buffer, ID3D11ClassInstance,
        ID3D11DepthStencilView, ID3D11DeviceChild, ID3D11DeviceChildTrait, ID3D11GeometryShader,
        ID3D11InputLayout, ID3D11PixelShader, ID3D11Predicate, ID3D11RenderTargetView,
        ID3D11Resource, ID3D11SamplerState, ID3D11ShaderResourceView, ID3D11UnorderedAccessView,
        ID3D11VertexShader, D3D11_MAP, D3D11_MAPPED_SUBRESOURCE, D3D11_PRIMITIVE_TOPOLOGY,
    },
    dxgi::DXGI_FORMAT,
    unknwn::{IUnknown, IUnknownTrait},
    BOOL, FLOAT, HRESULT, INT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{
        ID3D11Device, D3D11_ASYNC_GETDATA_FLAG, D3D11_BIND_FLAG, D3D11_BLEND,
        D3D11_BUFFER_UAV_FLAG, D3D11_COUNTER, D3D11_INPUT_CLASSIFICATION,
        D3D11_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL, D3D11_KEEP_UNORDERED_ACCESS_VIEWS,
        D3D11_MAP_FLAG, D3D11_QUERY, D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT,
    },
    d3d11_1::{ID3D11DeviceContext1, D3D11_1_UAV_SLOT_COUNT},
    d3d11shader::{
        ID3D11ShaderReflection, ID3D11ShaderReflectionConstantBuffer,
        ID3D11ShaderReflectionVariable,
    },
    d3dcompiler::D3D11Reflect,
    DXGI_ERROR_DEVICE_REMOVED, DXGI_ERROR_WAS_STILL_DRAWING, FALSE, S_FALSE, S_OK, TRUE,
};
use std::ffi::c_void;
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

com_interface!(
    /// The [`ID3D11DeviceContext`] interface represents a device context which generates rendering
    /// commands.
    pub abstract ID3D11DeviceContext(ID3D11DeviceContextVTable/ID3D11DeviceContextTrait):
        ID3D11DeviceChild/ID3D11DeviceChildTrait(device_child) +
        IUnknown/IUnknownTrait(device_child.unknown) {
        const IID = 0xC0BFA96C-0xE089-0x44FB-0x8EAF-0x26F8796190DA;

        /// Sets the constant buffers used by the vertex shader pipeline stage.
        ///
        /// # Parameters
        ///  * `start_slot` - Index into the device's zero-based array to begin setting constant
        ///                   buffers to (ranges from 0 to
        ///                   `D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1`).
        ///  * `num_buffers` - Number of buffers to set (ranges from 0 to
        ///                    `D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - start_slot`).
        ///  * `constant_buffers` - Array of constant buffers (see [`ID3D11Buffer`]) being given to
        ///                         the device.
        ///
        /// # Remarks
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        ///
        /// The Direct3D 11.1 runtime, which is available starting with Windows 8, can bind a
        /// larger number of [`ID3D11Buffer`] resources to the shader than the maximum constant
        /// buffer size that is supported by shaders (4096 constants – 4*32-bit components each).
        /// When you bind such a large buffer, the shader can access only the first 4096 4*32-bit
        /// component constants in the buffer, as if 4096 constants is the full size of the buffer.
        ///
        /// If the application wants the shader to access other parts of the buffer, it must call
        /// the [`ID3D11DeviceContext1::vs_set_constant_buffers1`] method instead.
        fn vs_set_constant_buffers(
            &mut self,
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *mut ID3D11Buffer
        );

        /// Bind an array of shader resources to the pixel shader stage.
        ///
        /// # Parameters
        ///  * `start_slot` - Index into the device's zero-based array to begin setting shader
        ///                   resources to (ranges from 0 to
        ///                   `D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1`).
        ///  * `num_views` - Number of shader resources to set. Up to a maximum of 128 slots are
        ///                  available for shader resources (ranges from 0 to
        ///                  `D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - start_slot`).
        ///  * `shader_resource_views` - Array of shader resource view interfaces to set to the
        ///                              device.
        ///
        /// # Remarks
        /// If an overlapping resource view is already bound to an output slot, such as a render
        /// target, then this API will fill the destination shader resource slot with [`null_mut`].
        ///
        /// For information about creating shader-resource views, see
        /// [`ID3D11Device::create_shader_resource_view`].
        ///
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        fn ps_set_shader_resources(
            &mut self,
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *mut ID3D11ShaderResourceView
        );

        /// Sets a pixel shader to the device.
        ///
        /// # Parameters
        ///  * `pixel_shader` - Pointer to a pixel shader (see [`ID3D11PixelShader`]). Passing in
        ///                     [`null_mut`] disables the shader for this pipeline stage.
        ///  * `class_instances` - A pointer to an array of class-instance interfaces (see
        ///                        [`ID3D11ClassInstance`]). Each interface used by a shader must
        ///                        have a corresponding class instance or the shader will get
        ///                        disabled. Set `class_instances` to [`null`] if the shader does
        ///                        not use any interfaces.
        ///  * `num_class_instances` - The number of class-instance interfaces in the array.
        ///
        /// # Remarks
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        ///
        /// The maximum number of instances a shader can have is 256.
        ///
        /// Set `class_instances` to [`null`] if no interfaces are used in the shader. If it is not
        /// [`null`], the number of class instances must match the number of interfaces used in the
        /// shader. Furthermore, each interface pointer must have a corresponding class instance or
        /// the assigned shader will be disabled.
        fn ps_set_shader(
            &mut self,
            pixel_shader: *mut ID3D11PixelShader,
            class_instances: *const *mut ID3D11ClassInstance,
            num_class_instances: UINT
        );

        /// Set an array of sampler states to the pixel shader pipeline stage.
        ///
        /// # Parameters
        ///  * `start_slot` - Index into the device's zero-based array to begin setting samplers to
        ///                   (ranges from 0 to `D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1`).
        ///  * `num_samplers` - Number of samplers in the array. Each pipeline stage has a total of
        ///                     16 sampler slots available (ranges from 0 to
        ///                     `D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - start_slot`).
        ///  * `samplers` - Pointer to an array of sampler-state interfaces (see
        ///                 [`ID3D11SamplerState`]).
        ///
        /// # Remarks
        /// Any sampler may be set to [`null_mut`]; this invokes the default state.
        ///
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        fn ps_set_samplers(
            &mut self,
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *mut ID3D11SamplerState
        );

        /// Set a vertex shader to the device.
        ///
        /// # Parameters
        ///  * `vertex_shader` - Pointer to a vertex shader (see [`ID3D11VertexShader`]). Passing
        ///                      in [`null_mut`] disables the shader for this pipeline stage.
        ///  * `class_instances` - A pointer to an array of class-instance interfaces (see
        ///                        [`ID3D11ClassInstance`]). Each interface used by a shader must
        ///                        have a corresponding class instance or the shader will get
        ///                        disabled. Set `class_instances` to [`null`] if the shader does
        ///                        not use any interfaces.
        ///  * `num_class_instances` - The number of class-instance interfaces in the array.
        ///
        /// # Remarks
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        ///
        /// The maximum number of instances a shader can have is 256.
        fn vs_set_shader(
            &mut self,
            vertex_shader: *mut ID3D11VertexShader,
            class_instances: *const *mut ID3D11ClassInstance,
            num_class_instances: UINT
        );

        /// Draw indexed, non-instanced primitives.
        ///
        /// # Parameters
        ///  * `index_count` - Number of indices to draw.
        ///  * `start_index_location` - The location of the first index read by the GPU from the
        ///                             index buffer.
        ///  * `base_vertex_location` - A value added to each index before reading a vertex from
        ///                             the vertex buffer.
        ///
        /// # Remarks
        /// A draw API submits work to the rendering pipeline.
        ///
        /// If the sum of both indices is negative, the result of the function call is undefined.
        fn draw_indexed(
            &mut self,
            index_count: UINT,
            start_index_location: UINT,
            base_vertex_location: INT
        );

        /// Draw non-indexed, non-instanced primitives.
        ///
        /// # Parameters
        ///  * `vertex_count` - Number of vertices to draw.
        ///  * `start_vertex_location` - Index of the first vertex, which is usually an offset in a
        ///                              vertex buffer.
        ///
        /// # Remarks
        /// Draw submits work to the rendering pipeline.
        ///
        /// The vertex data for a draw call normally comes from a vertex buffer that is bound to
        /// the pipeline.
        ///
        /// Even without any vertex buffer bound to the pipeline, you can generate your own vertex
        /// data in your vertex shader by using the `SV_VertexID` system-value semantic to
        /// determine the current vertex that the runtime is processing.
        fn draw(&mut self, vertex_count: UINT, start_vertex_location: UINT);

        /// Gets a pointer to the data contained in a subresource, and denies the GPU access to
        /// that subresource.
        ///
        /// # Parameters
        ///  * `resource` - A pointer to a [`ID3D11Resource`] interface.
        ///  * `subresource` - Index number of the subresource.
        ///  * `map_type` - A [`D3D11_MAP`]-typed value that specifies the CPU's read and write
        ///                 permissions for a resource.
        ///  * `map_flags` - Flag that specifies what the CPU does when the GPU is busy. This flag
        ///                  is optional.
        ///  * `mapped_resource` - A pointer to the [`D3D11_MAPPED_SUBRESOURCE`] structure for the
        ///                        mapped subresource.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// This method also returns [`DXGI_ERROR_WAS_STILL_DRAWING`] if map_flags specifies
        /// [`D3D11_MAP_FLAG::DoNotWait`] and the GPU is not yet finished with the resource.
        ///
        /// This method also returns [`DXGI_ERROR_DEVICE_REMOVED`] if `map_type` allows any CPU
        /// read access and the video card has been removed.
        ///
        /// # Remarks
        /// If you call [`ID3D11DeviceContext::map`] on a deferred context, you can only pass
        /// [`D3D11_MAP::WriteDiscard`], [`D3D11_MAP::WriteNoOverwrite`], or both to the `map_type`
        /// parameter. Other [`D3D11_MAP`]-typed values are not supported for a deferred context.
        fn map(
            &mut self,
            resource: *mut ID3D11Resource,
            subresource: UINT,
            map_type: D3D11_MAP,
            map_flags: UINT,
            mapped_resource: *mut D3D11_MAPPED_SUBRESOURCE
        ) -> HRESULT;

        /// Invalidate the pointer to a resource and reenable the GPU's access to that resource.
        ///
        /// # Parameters
        ///  * `resource` - A pointer to a [`ID3D11Resource`] interface.
        ///  * `subresource` - A subresource to be unmapped.
        fn unmap(&mut self, resource: *mut ID3D11Resource, subresource: UINT);

        /// Sets the constant buffers used by the pixel shader pipeline stage.
        ///
        /// # Parameters
        ///  * `start_slot` - Index into the device's zero-based array to begin setting constant
        ///                   buffers to (ranges from 0 to
        ///                   `D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1`).
        ///  * `num_buffers` - Index into the device's zero-based array to begin setting constant
        ///                    buffers to (ranges from 0 to
        ///                    `D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1`).
        ///  * `constant_buffers` - Array of constant buffers (see [`ID3D11Buffer`]) being given to
        ///                         the device.
        ///
        /// # Remarks
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        ///
        /// The Direct3D 11.1 runtime, which is available on Windows 8 and later operating systems,
        /// can bind a larger number of [`ID3D11Buffer`] resources to the shader than the maximum
        /// constant buffer size that is supported by shaders (4096 constants – 432-bit components
        /// each). When you bind such a large buffer, the shader can access only the first 4096
        /// 432-bit component constants in the buffer, as if 4096 constants is the full size of the
        /// buffer.
        ///
        /// To enable the shader to access other parts of the buffer, call
        /// [`ID3D11DeviceContext1::ps_set_constant_buffers1`] instead of
        /// [`ID3D11DeviceContext::ps_set_constant_buffers`].
        /// [`ID3D11DeviceContext1::ps_set_constant_buffers1`] has additional parameters
        /// `first_constant` and `num_constants`.
        fn ps_set_constant_buffers(
            &mut self,
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *mut ID3D11Buffer
        );

        /// Bind an input-layout object to the input-assembler stage.
        ///
        /// # Parameters
        ///  * `input_layout` - A pointer to the input-layout object (see [`ID3D11InputLayout`]),
        ///                     which describes the input buffers that will be read by the IA
        ///                     stage.
        ///
        /// # Remarks
        /// Input-layout objects describe how vertex buffer data is streamed into the IA pipeline
        /// stage. To create an input-layout object, call [`ID3D11Device::create_input_layout`].
        ///
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        fn ia_set_input_layout(&mut self, input_layout: *mut ID3D11InputLayout);

        /// Bind an array of vertex buffers to the input-assembler stage.
        ///
        /// # Parameters
        ///  * `start_slot` - The first input slot for binding. The first vertex buffer is
        ///                   explicitly bound to the start slot; this causes each additional
        ///                   vertex buffer in the array to be implicitly bound to each subsequent
        ///                   input slot. The maximum of 16 or 32 input slots (ranges from 0 to
        ///                   `D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - 1`) are available; the
        ///                   maximum number of input slots depends on the feature level.
        ///  * `num_buffers` - The number of vertex buffers in the array. The number of buffers
        ///                    (plus the starting slot) can't exceed the total number of IA-stage
        ///                    input slots (ranges from 0 to
        ///                    `D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - start_slot`).
        ///  * `vertex_buffers` - A pointer to an array of vertex buffers (see [`ID3D11Buffer`]).
        ///                       The vertex buffers must have been created with the
        ///                       [`D3D11_BIND_FLAG::VertexBuffer`] flag.
        ///  * `strides` - Pointer to an array of stride values; one stride value for each buffer
        ///                in the vertex-buffer array. Each stride is the size (in bytes) of the
        ///                elements that are to be used from that vertex buffer.
        ///  * `offsets` - Pointer to an array of offset values; one offset value for each buffer
        ///                in the vertex-buffer array. Each offset is the number of bytes between
        ///                the first element of a vertex buffer and the first element that will be
        ///                used.
        ///
        /// # Remarks
        /// Calling this method using a buffer that is currently bound for writing (that is, bound
        /// to the stream output pipeline stage) will effectively bind [`null_mut`] instead because
        /// a buffer can't be bound as both an input and an output at the same time.
        ///
        /// The debug layer will generate a warning whenever a resource is prevented from being
        /// bound simultaneously as an input and an output, but this will not prevent invalid data
        /// from being used by the runtime.
        ///
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        fn ia_set_vertex_buffers(
            &mut self,
            start_slot: UINT,
            num_buffers: UINT,
            vertex_buffers: *const *mut ID3D11Buffer,
            strides: *const UINT,
            offsets: *const UINT
        );

        /// Bind an index buffer to the input-assembler stage.
        ///
        /// # Parameters
        ///  * `index_buffer` - A pointer to an [`ID3D11Buffer`] object, that contains indices. The
        ///                     index buffer must have been created with the
        ///                     [`D3D11_BIND_FLAG::IndexBuffer`] flag.
        ///  * `format` - A [`DXGI_FORMAT`] that specifies the format of the data in the index
        ///               buffer. The only formats allowed for index buffer data are 16-bit
        ///               ([`DXGI_FORMAT::R16UInt`]) and 32-bit ([`DXGI_FORMAT::R32UInt`])
        ///               integers.
        ///  * `offset` - Offset (in bytes) from the start of the index buffer to the first index
        ///               to use.
        ///
        /// # Remarks
        /// Calling this method using a buffer that is currently bound for writing (i.e. bound to
        /// the stream output pipeline stage) will effectively bind [`null_mut`] instead because a
        /// buffer cannot be bound as both an input and an output at the same time.
        ///
        /// The debug layer will generate a warning whenever a resource is prevented from being
        /// bound simultaneously as an input and an output, but this will not prevent invalid data
        /// from being used by the runtime.
        ///
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        fn ia_set_index_buffer(
            &mut self,
            index_buffer: *mut ID3D11Buffer,
            format: DXGI_FORMAT,
            offset: UINT
        );

        /// Draw indexed, instanced primitives.
        ///
        /// # Parameters
        ///  * `index_count_per_instance` - Number of indices read from the index buffer for each
        ///                                 instance.
        ///  * `instance_count` - Number of instances to draw.
        ///  * `start_index_location` - The location of the first index read by the GPU from the
        ///                             index buffer.
        ///  * `base_vertex_location` - A value added to each index before reading a vertex from
        ///                             the vertex buffer.
        ///  * `start_instance_location` - A value added to each index before reading per-instance
        ///                                data from a vertex buffer.
        ///
        /// # Remarks
        /// A draw API submits work to the rendering pipeline.
        ///
        /// Instancing may extend performance by reusing the same geometry to draw multiple objects
        /// in a scene. One example of instancing could be to draw the same object with different
        /// positions and colors. Instancing requires multiple vertex buffers: at least one for
        /// per-vertex data and a second buffer for per-instance data.
        ///
        /// The second buffer is needed only if the input layout that you use has elements that use
        /// [`D3D11_INPUT_CLASSIFICATION::PerInstanceData`] as the input element classification
        /// buffer for per-instance data.
        fn draw_indexed_instanced(
            &mut self,
            index_count_per_instance: UINT,
            instance_count: UINT,
            start_index_location: UINT,
            base_vertex_location: INT,
            start_instance_location: UINT
        );

        /// Draw non-indexed, instanced primitives.
        ///
        /// # Parameters
        ///  * `vertex_count_per_instance` - Number of vertices to draw.
        ///  * `instance_count` - Number of instances to draw.
        ///  * `start_vertex_location` - Index of the first vertex.
        ///  * `start_instance_location` - A value added to each index before reading per-instance
        ///                                data from a vertex buffer.
        ///
        /// # Remarks
        /// A draw API submits work to the rendering pipeline.
        ///
        /// Instancing may extend performance by reusing the same geometry to draw multiple objects
        /// in a scene. One example of instancing could be to draw the same object with different
        /// positions and colors.
        ///
        /// The vertex data for an instanced draw call normally comes from a vertex buffer that is
        /// bound to the pipeline. However, you could also provide the vertex data from a shader
        /// that has instanced data identified with a system-value semantic (`SV_InstanceID`).
        fn draw_instanced(
            &mut self,
            vertex_count_per_instance: UINT,
            instance_count: UINT,
            start_vertex_location: UINT,
            start_instance_location: UINT
        );

        /// Sets the constant buffers used by the geometry shader pipeline stage.
        ///
        /// # Parameters
        ///  * `start_slot` - Index into the device's zero-based array to begin setting constant
        ///                   buffers to (ranges from 0 to
        ///                   `D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1`).
        ///  * `num_buffers` - Number of buffers to set (ranges from 0 to
        ///                    `D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - start_slot`).
        ///  * `constant_buffers` - Array of constant buffers (see [`ID3D11Buffer`]) being given to
        ///                         the device.
        ///
        /// # Remarks
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        ///
        /// You can't use the [`ID3D11ShaderReflectionConstantBuffer`] interface to get information
        /// about what is currently bound to the pipeline in the device context. But you can use
        /// [`ID3D11ShaderReflectionConstantBuffer`] to get information from a compiled shader. For
        /// example, you can use [`ID3D11ShaderReflectionConstantBuffer`] and
        /// [`ID3D11ShaderReflectionVariable`] to determine the slot in which a geometry shader
        /// expects a constant buffer. You can then pass this slot number to
        /// [`ID3D11DeviceContext::gs_set_constant_buffers`] to set the constant buffer. You can
        /// call the [`D3D11Reflect`] function to retrieve the address of a pointer to the
        /// [`ID3D11ShaderReflection`] interface and then call
        /// [`ID3D11ShaderReflection::get_constant_buffer_by_name`] to get a pointer to
        /// [`ID3D11ShaderReflectionConstantBuffer`].
        ///
        /// The Direct3D 11.1 runtime, which is available starting with Windows 8, can bind a
        /// larger number of [`ID3D11Buffer`] resources to the shader than the maximum constant
        /// buffer size that is supported by shaders (4096 constants – 432-bit components each).
        /// When you bind such a large buffer, the shader can access only the first 4096 432-bit
        /// component constants in the buffer, as if 4096 constants is the full size of the buffer.
        ///
        /// If the application wants the shader to access other parts of the buffer, it must call
        /// the [`ID3D11DeviceContext1::gs_set_constant_buffers1`] method instead.
        fn gs_set_constant_buffers(
            &mut self,
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *mut ID3D11Buffer
        );

        /// Set a geometry shader to the device.
        ///
        /// # Parameters
        ///  * `shader` - Pointer to a geometry shader (see [`ID3D11GeometryShader`]). Passing in
        ///               [`null_mut`] disables the shader for this pipeline stage.
        ///  * `class_instances` - A pointer to an array of class-instance interfaces (see
        ///                        [`ID3D11ClassInstance`]). Each interface used by a shader must
        ///                        have a corresponding class instance or the shader will get
        ///                        disabled. Set `class_instances` to [`null`] if the shader does
        ///                        not use any interfaces.
        ///  * `num_class_instances` - The number of class-instance interfaces in the array.
        ///
        /// # Remarks
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        ///
        /// The maximum number of instances a shader can have is 256.
        fn gs_set_shader(
            &mut self,
            shader: *mut ID3D11GeometryShader,
            class_instances: *const *mut ID3D11ClassInstance,
            num_class_instances: UINT
        );

        /// Bind information about the primitive type, and data order that describes input data for
        /// the input assembler stage.
        ///
        /// # Parameters
        ///  * `topology` - The type of primitive and ordering of the primitive data (see
        ///                 [`D3D11_PRIMITIVE_TOPOLOGY`]).
        fn ia_set_primitive_topology(&mut self, topology: D3D11_PRIMITIVE_TOPOLOGY);

        /// Bind an array of shader resources to the vertex-shader stage.
        ///
        /// # Parameters
        ///  * `start_slot` - Index into the device's zero-based array to begin setting shader
        ///                   resources to (range is from 0 to
        ///                   `D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1`).
        ///  * `num_views` - Number of shader resources to set. Up to a maximum of 128 slots are
        ///                  available for shader resources (range is from 0 to
        ///                  `D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - start_slot`).
        ///  * `shader_resource_views` - Array of shader resource view interfaces to set to the
        ///                              device.
        ///
        /// # Remarks
        /// If an overlapping resource view is already bound to an output slot, such as a
        /// rendertarget, then this API will fill the destination shader resource slot with
        /// [`null_mut`].
        ///
        /// For information about creating shader-resource views, see
        /// [`ID3D11Device::create_shader_resource_view`].
        ///
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        fn vs_set_shader_resources(
            &mut self,
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *mut ID3D11ShaderResourceView
        );

        /// Set an array of sampler states to the vertex shader pipeline stage.
        ///
        /// # Parameters
        ///  * `start_slot` - Index into the device's zero-based array to begin setting samplers to
        ///                   (ranges from 0 to `D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1`).
        ///  * `num_samplers` - Number of samplers in the array. Each pipeline stage has a total of
        ///                     16 sampler slots available (ranges from 0 to
        ///                     `D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - start_slot`).
        ///  * `samplers` - Pointer to an array of sampler-state interfaces (see
        ///                 [`ID3D11SamplerState`]).
        ///
        /// # Remarks
        /// Any sampler may be set to [`null_mut`]; this invokes the default state.
        fn vs_set_samplers(
            &mut self,
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *mut ID3D11SamplerState
        );

        /// Mark the beginning of a series of commands.
        ///
        /// # Parameters
        ///  * `r#async` - A pointer to an [`ID3D11Asynchronous`] interface.
        ///
        /// # Remarks
        /// Use [`ID3D11DeviceContext::end`] to mark the ending of the series of commands.
        fn begin(&mut self, r#async: *mut ID3D11Asynchronous);

        /// Mark the end of a series of commands.
        ///
        /// # Parameters
        ///  * `r#async` - A pointer to an [`ID3D11Asynchronous`] interface.
        ///
        /// # Remarks
        /// Use [`ID3D11DeviceContext::begin`] to mark the beginning of the series of commands.
        fn end(&mut self, r#async: *mut ID3D11Asynchronous);

        /// Get data from the graphics processing unit (GPU) asynchronously.
        ///
        /// # Parameters
        ///  * `r#async` - A pointer to an [`ID3D11Asynchronous`] interface for the object about
        ///                which [`ID3D11DeviceContext::get_data`] retrieves data.
        ///  * `data` - Address of memory that will receive the data. If [`null_mut`],
        ///             [`ID3D11DeviceContext::get_data`] will be used only to check status. The
        ///             type of data output depends on the type of asynchronous interface.
        ///  * `data_size` - Size of the data to retrieve or 0. Must be 0 when `data` is
        ///                  [`null_mut`].
        ///  * `get_data_flags` - Optional flags. Can be 0 or any combination of the flags
        ///                       enumerated by [`D3D11_ASYNC_GETDATA_FLAG`].
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes. A return value of [`S_OK`]
        /// indicates that the data at pData is available for the calling application to access. A
        /// return value of [`S_FALSE`] indicates that the data is not yet available. If the data
        /// is not yet available, the application must call [`ID3D11DeviceContext::get_data`] until
        /// the data is available.
        ///
        /// # Remarks
        /// Queries in a deferred context are limited to predicated drawing. That is, you cannot
        /// call [`ID3D11DeviceContext::get_data`] on a deferred context to get data about a query;
        /// you can only call [`ID3D11DeviceContext::get_data`] on the immediate context to get
        /// data about a query. For predicated drawing, the results of a predication-type query are
        /// used by the GPU and not returned to an application. For more information about
        /// predication and predicated drawing, see [`ID3D11DeviceContext::set_predication`].
        ///
        /// [`ID3D11DeviceContext::get_data`] retrieves the data that the runtime collected between
        /// calls to [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`]. Certain
        /// queries only require a call to [`ID3D11DeviceContext::end`] in which case the data
        /// returned by [`ID3D11DeviceContext::get_data`] is accurate up to the last call to
        /// [`ID3D11DeviceContext::end`]. For information about the queries that only require a
        /// call to [`ID3D11DeviceContext::end`] and about the type of data that
        /// [`ID3D11DeviceContext::get_data`] retrieves for each query, see [`D3D11_QUERY`].
        ///
        /// If DataSize is 0, [`ID3D11DeviceContext::get_data`] is only used to check status.
        ///
        /// An application gathers counter data by calling [`ID3D11DeviceContext::begin`], issuing
        /// some graphics commands, calling [`ID3D11DeviceContext::end`], and then calling
        /// [`ID3D11DeviceContext::get_data`] to get data about what happened in between the
        /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`] calls. For information
        /// about performance counter types, see [`D3D11_COUNTER`].
        fn get_data(
            &mut self,
            r#async: *mut ID3D11Asynchronous,
            data: *mut c_void,
            data_size: UINT,
            get_data_flags: UINT
        ) -> HRESULT;

        /// Set a rendering predicate.
        ///
        /// # Parameters
        ///  * `predicate` - A pointer to the [`ID3D11Predicate`] interface that represents the
        ///                  rendering predicate. A [`null_mut`] value indicates "no" predication;
        ///                  in this case, the value of `predicate_value` is irrelevant but will be
        ///                  preserved for [`ID3D11DeviceContext::get_predication`].
        ///  * `predicate_value` - If [`TRUE`], rendering will be affected by when the predicate's
        ///                        conditions are met. If [`FALSE`], rendering will be affected
        ///                        when the conditions are not met.
        ///
        /// # Remarks
        /// The predicate must be in the "issued" or "signaled" state to be used for predication.
        /// While the predicate is set for predication, calls to [`ID3D11DeviceContext::begin`] and
        /// [`ID3D11DeviceContext::end`] are invalid.
        ///
        /// Use this method to denote that subsequent rendering and resource manipulation commands
        /// are not actually performed if the resulting predicate data of the predicate is equal to
        /// the `predicate_value`. However, some predicates are only hints, so they may not
        /// actually prevent operations from being performed.
        ///
        /// The primary usefulness of predication is to allow an application to issue rendering and
        /// resource manipulation commands without taking the performance hit of spinning, waiting
        /// for [`ID3D11DeviceContext::get_data`] to return. So, predication can occur while
        /// [`ID3D11DeviceContext::get_data`] returns [`S_FALSE`]. Another way to think of it: an
        /// application can also use predication as a fallback, if it is possible that
        /// [`ID3D11DeviceContext::get_data`] returns [`S_FALSE`]. If
        /// [`ID3D11DeviceContext::get_data`] returns [`S_OK`], the application can skip calling
        /// the rendering and resource manipulation commands manually with its own application
        /// logic.
        ///
        /// Rendering and resource manipulation commands for Direct3D 11 include these Draw,
        /// Dispatch, Copy, Update, Clear, Generate, and Resolve operations:
        ///  - [`ID3D11DeviceContext::draw`]
        ///  - [`ID3D11DeviceContext::draw_auto`]
        ///  - [`ID3D11DeviceContext::draw_indexed`]
        ///  - [`ID3D11DeviceContext::draw_indexed_instanced`]
        ///  - [`ID3D11DeviceContext::draw_indexed_instanced_indirect`]
        ///  - [`ID3D11DeviceContext::draw_instanced`]
        ///  - [`ID3D11DeviceContext::draw_instanced_indirect`]
        ///  - [`ID3D11DeviceContext::dispatch`]
        ///  - [`ID3D11DeviceContext::dispatch_indirect`]
        ///  - [`ID3D11DeviceContext::copy_resource`]
        ///  - [`ID3D11DeviceContext::copy_structure_count`]
        ///  - [`ID3D11DeviceContext::copy_subresource_region`]
        ///  - [`ID3D11DeviceContext1::copy_subresource_region1`]
        ///  - [`ID3D11DeviceContext2::copy_tiles`]
        ///  - [`ID3D11DeviceContext2::copy_tile_mappings`]
        ///  - [`ID3D11DeviceContext::update_subresource`]
        ///  - [`ID3D11DeviceContext1::update_subresource1`]
        ///  - [`ID3D11DeviceContext2::update_tiles`]
        ///  - [`ID3D11DeviceContext2::update_tile_mappings`]
        ///  - [`ID3D11DeviceContext::clear_render_target_view`]
        ///  - [`ID3D11DeviceContext::clear_unordered_access_view_float`]
        ///  - [`ID3D11DeviceContext::clear_unordered_access_view_uint`]
        ///  - [`ID3D11DeviceContext1::clear_view`]
        ///  - [`ID3D11DeviceContext::clear_depth_stencil_View`]
        ///  - [`ID3D11DeviceContext::generate_mips`]
        ///  - [`ID3D11DeviceContext::resolve_subresource`]
        ///
        /// You can set a rendering predicate on an immediate or a deferred context.
        fn set_predication(&mut self, predicate: *mut ID3D11Predicate, predicate_value: BOOL);

        /// Bind an array of shader resources to the geometry shader stage.
        ///
        /// # Parameters
        ///  * `start_slot` - Index into the device's zero-based array to begin setting shader
        ///                   resources to (ranges from 0 to
        ///                   `D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1`).
        ///  * `num_views` - Number of shader resources to set. Up to a maximum of 128 slots are
        ///                  available for shader resources(ranges from 0 to
        ///                  `D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - start_slot`).
        ///  * `shader_resources` - Array of shader resource view interfaces to set to the device.
        ///
        /// # Remarks
        /// If an overlapping resource view is already bound to an output slot, such as a render
        /// target, then the method will fill the destination shader resource slot with
        /// [`null_mut`].
        ///
        /// For information about creating shader-resource views, see
        /// [`ID3D11Device::create_shader_resource_view`].
        ///
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        fn gs_set_shader_resources(
            &mut self,
            start_slot: UINT,
            num_views: UINT,
            shader_resources: *const *mut ID3D11ShaderResourceView
        );

        /// Set an array of sampler states to the geometry shader pipeline stage.
        ///
        /// # Parameters
        ///  * `start_slot` - Index into the device's zero-based array to begin setting samplers to
        ///                   (ranges from 0 to `D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1`).
        ///  * `num_samplers` - Number of samplers in the array. Each pipeline stage has a total of
        ///                     16 sampler slots available (ranges from 0 to
        ///                     `D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - start_slot`).
        ///  * `samplers` - Pointer to an array of sampler-state interfaces (see
        ///                 [`ID3D11SamplerState`]).
        ///
        /// # Remarks
        /// Any sampler may be set to [`null_mut`]; this invokes the default state.
        ///
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        fn gs_set_samplers(
            &mut self,
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *mut ID3D11SamplerState
        );

        /// Bind one or more render targets atomically and the depth-stencil buffer to the
        /// output-merger stage.
        ///
        /// # Parameters
        ///  * `num_views` - Number of render targets to bind (ranges between 0 and
        ///                  [`D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT`]). If this parameter is
        ///                  nonzero, the number of entries in the array to which
        ///                  `render_target_views` points must equal the number in this parameter.
        ///  * `render_target_views` - Pointer to an array of [`ID3D11RenderTargetView`] that
        ///                            represent the render targets to bind to the device. If this
        ///                            parameter is [`null`] and `num_views` is 0, no render
        ///                            targets are bound.
        ///  * `depth_stencil_view` - Pointer to a [`ID3D11DepthStencilView`] that represents the
        ///                           depth-stencil view to bind to the device. If this parameter
        ///                           is [`null_mut`], the depth-stencil view is not bound.
        ///
        /// # Remarks
        /// The maximum number of active render targets a device can have active at any given time
        /// is set by [`D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT`]. It is invalid to try to set the
        /// same subresource to multiple render target slots. Any render targets not defined by
        /// this call are set to [`null_mut`].
        ///
        /// If any subresources are also currently bound for reading in a different stage or
        /// writing (perhaps in a different part of the pipeline), those bind points will be set to
        /// [`null_mut`], in order to prevent the same subresource from being read and written
        /// simultaneously in a single rendering operation.
        ///
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        ///
        /// If the render-target views were created from an array resource type, all of the
        /// render-target views must have the same array size. This restriction also applies to the
        /// depth-stencil view, its array size must match that of the render-target views being
        /// bound.
        ///
        /// The pixel shader must be able to simultaneously render to at least eight separate
        /// render targets. All of these render targets must access the same type of resource:
        /// Buffer, Texture 1D, Texture 1D Array, Texture 2D, Texture 2D Array, Texture 3D, or
        /// Texture Cube. All render targets must have the same size in all dimensions (width and
        /// height, and depth for 3D or array size for *Array types). If render targets use
        /// multisample anti-aliasing, all bound render targets and depth buffer must be the same
        /// form of multisample resource (that is, the sample counts must be the same). Each render
        /// target can have a different data format. These render target formats are not required
        /// to have identical bit-per-element counts.
        ///
        /// Any combination of the eight slots for render targets can have a render target set or
        /// not set.
        ///
        /// The same resource view cannot be bound to multiple render target slots simultaneously.
        /// However, you can set multiple non-overlapping resource views of a single resource as
        /// simultaneous multiple render targets.
        fn om_set_render_targets(
            &mut self,
            num_views: UINT,
            render_target_views: *const *mut ID3D11RenderTargetView,
            depth_stencil_view: *mut ID3D11DepthStencilView
        );

        /// Binds resources to the output-merger stage.
        ///
        /// # Parameters
        ///  * `num_rtvs` - Number of render targets to bind (ranges between 0 and
        ///                 [`D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT`]). If this parameter is
        ///                 nonzero, the number of entries in the array to which
        ///                 `render_target_views` points must equal the number in this parameter.
        ///                 If you set `num_rtvs` to
        ///                 [`D3D11_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL`], this method does not
        ///                 modify the currently bound render-target views (RTVs) and also does not
        ///                 modify depth-stencil view (DSV).
        ///  * `render_target_views` - Pointer to an array of [`ID3D11RenderTargetView`]s that
        ///                            represent the render targets to bind to the device. If this
        ///                            parameter is [`null`] and `num_rtvs` is 0, no render targets
        ///                            are bound.
        ///  * `depth_stencil_view` - Pointer to a [`ID3D11DepthStencilView`] that represents the
        ///                           depth-stencil view to bind to the device. If this parameter
        ///                           is [`null_mut`], the depth-stencil view is not bound.
        ///  * `uav_start_slot` - Index into a zero-based array to begin setting unordered-access
        ///                       views (ranges from 0 to `D3D11_PS_CS_UAV_REGISTER_COUNT - 1`).
        ///                       For the Direct3D 11.1 runtime, which is available starting with
        ///                       Windows 8, this value can range from 0 to
        ///                       `D3D11_1_UAV_SLOT_COUNT - 1`. [`D3D11_1_UAV_SLOT_COUNT`] is
        ///                       defined as 64. For pixel shaders, UAVStartSlot should be equal to
        ///                       the number of render-target views being bound.
        ///  * `num_uavs` - Number of unordered-access views (UAVs) in `unordered_access_views`. If
        ///                 you set `num_uavs` to [`D3D11_KEEP_UNORDERED_ACCESS_VIEWS`], this
        ///                 method does not modify the currently bound unordered-access views. For
        ///                 the Direct3D 11.1 runtime, which is available starting with Windows 8,
        ///                 this value can range from 0 to
        ///                 `D3D11_1_UAV_SLOT_COUNT - uav_start_slot`.
        ///  * `unordered_access_views` - Pointer to an array of [`ID3D11UnorderedAccessView`]s
        ///                               that represent the unordered-access views to bind to the
        ///                               device. If this parameter is [`null`] and `num_uavs` is
        ///                               0, no unordered-access views are bound.
        ///  * `uav_initial_counts` - An array of append and consume buffer offsets. A value of -1
        ///                           indicates to keep the current offset. Any other values set
        ///                           the hidden counter for that appendable and consumable UAV.
        ///                           `uav_initial_counts` is relevant only for UAVs that were
        ///                           created with either [`D3D11_BUFFER_UAV_FLAG::Append`] or
        ///                           [`D3D11_BUFFER_UAV_FLAG::Counter`] specified when the UAV was
        ///                           created; otherwise, the argument is ignored.
        ///
        /// # Remarks
        /// For pixel shaders, the render targets and unordered-access views share the same
        /// resource slots when being written out. This means that UAVs must be given an offset so
        /// that they are placed in the slots after the render target views that are being bound.
        ///
        /// Two RTVs conflict if they share a subresource (and therefore share the same resource).
        ///
        /// Two UAVs conflict if they share a subresource (and therefore share the same resource).
        ///
        /// An RTV conflicts with a UAV if they share a subresource or share a bind point.
        fn om_set_render_targets_and_unordered_access_views(
            &mut self,
            num_rtvs: UINT,
            render_target_views: *const *mut ID3D11RenderTargetView,
            depth_stencil_view: *mut ID3D11DepthStencilView,
            uav_start_slot: UINT,
            num_uavs: UINT,
            unordered_access_views: *const *mut ID3D11UnorderedAccessView,
            uav_initial_counts: *const UINT
        );

        /// Set the blend state of the output-merger stage.
        ///
        /// # Parameters
        ///  * `blend_state` - Pointer to a blend-state interface (see [`ID3D11BlendState`]). Pass
        ///                    [`null_mut`] for a default blend state.
        ///  * `blend_factor` - Array of blend factors, one for each RGBA component. The blend
        ///                     factors modulate values for the pixel shader, render target, or
        ///                     both. If you created the blend-state object with
        ///                     [`D3D11_BLEND::BlendFactor`] or [`D3D11_BLEND::InvBlendFactor`],
        ///                     the blending stage uses the non-[`null`] array of blend factors. If
        ///                     you didn't create the blend-state object with
        ///                     [`D3D11_BLEND::BlendFactor`] or [`D3D11_BLEND::InvBlendFactor`],
        ///                     the blending stage does not use the non-[`null`] array of blend
        ///                     factors; the runtime stores the blend factors, and you can later
        ///                     call [`ID3D11DeviceContext::om_get_blend_state`] to retrieve the
        ///                     blend factors. If you pass [`null`], the runtime uses or stores a
        ///                     blend factor equal to `[1., 1., 1., 1.]`.
        ///  * `sample_mask` - 32-bit sample coverage. The default value is 0xFFFFFFFF.
        ///
        /// # Remarks
        /// Blend state is used by the output-merger stage to determine how to blend together two
        /// RGB pixel values and two alpha values. The two RGB pixel values and two alpha values
        /// are the RGB pixel value and alpha value that the pixel shader outputs and the RGB pixel
        /// value and alpha value already in the output render target. The blend option controls
        /// the data source that the blending stage uses to modulate values for the pixel shader,
        /// render target, or both. The blend operation controls how the blending stage
        /// mathematically combines these modulated values.
        ///
        /// To create a blend-state interface, call [`ID3D11Device::create_blend_state`].
        ///
        /// Passing in [`null_mut`] for the blend-state interface indicates to the runtime to set a
        /// default blending state.
        ///
        /// A sample mask determines which samples get updated in all the active render targets.
        /// The mapping of bits in a sample mask to samples in a multisample render target is the
        /// responsibility of an individual application. A sample mask is always applied; it is
        /// independent of whether multisampling is enabled, and does not depend on whether an
        /// application uses multisample render targets.
        ///
        /// The method will hold a reference to the interfaces passed in. This differs from the
        /// device state behavior in Direct3D 10.
        fn om_set_blend_state(
            &mut self,
            blend_state: *mut ID3D11BlendState,
            blend_factor: [FLOAT; 4],
            sample_mask: UINT
        );
    }
);
