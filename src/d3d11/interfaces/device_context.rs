use crate::{
    com_interface,
    d3d11::{
        ID3D11Buffer, ID3D11ClassInstance, ID3D11DeviceChild, ID3D11DeviceChildTrait,
        ID3D11PixelShader, ID3D11ShaderResourceView,
    },
    unknwn::{IUnknown, IUnknownTrait},
    UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::ID3D11Device, d3d11_1::ID3D11DeviceContext1};
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
    }
);
