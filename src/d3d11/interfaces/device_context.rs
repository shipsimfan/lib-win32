use crate::{
    com_interface,
    d3d11::{
        ID3D11Buffer, ID3D11ClassInstance, ID3D11DeviceChild, ID3D11DeviceChildTrait,
        ID3D11InputLayout, ID3D11PixelShader, ID3D11Resource, ID3D11SamplerState,
        ID3D11ShaderResourceView, ID3D11VertexShader, D3D11_MAP, D3D11_MAPPED_SUBRESOURCE,
    },
    dxgi::DXGI_FORMAT,
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, INT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{ID3D11Device, D3D11_BIND_FLAG, D3D11_MAP_FLAG},
    d3d11_1::ID3D11DeviceContext1,
    DXGI_ERROR_DEVICE_REMOVED, DXGI_ERROR_WAS_STILL_DRAWING,
};
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
    }
);
