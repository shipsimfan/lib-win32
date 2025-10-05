use crate::{
    com_interface,
    d3d11::{
        ID3D11DeviceChild, ID3D11DeviceChildTrait, ID3D11Resource, ID3D11ResourceTrait,
        D3D11_TEXTURE1D_DESC,
    },
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Device;

com_interface!(
    /// A 1D texture interface accesses texel data, which is structured memory.
    ///
    /// # Remarks
    /// To create an empty 1D texture, call [`ID3D11Device::create_texture_1d`].
    ///
    /// Textures cannot be bound directly to the pipeline; instead, a view must be created and
    /// bound. Using a view, texture data can be interpreted at run time within certain
    /// restrictions. To use the texture as a render target or depth-stencil resource, call
    /// [`ID3D11Device::create_render_target_view`], and
    /// [`ID3D11Device::create_depth_stencil_view`], respectively. To use the texture as an input
    /// to a shader, create a by calling [`ID3D11Device::create_shader_resource_view`].
    pub abstract ID3D11Texture1D(ID3D11Texture1DVTable/ID3D11Texture1DTrait):
        ID3D11Resource/ID3D11ResourceTrait(resource) +
        ID3D11DeviceChild/ID3D11DeviceChildTrait(resource.device_child) +
        IUnknown/IUnknownTrait(resource.device_child.unknown) {
        const IID = 0xF8FB5C27-0xC6B3-0x4F75-0xA4C8-0x439AF2EF564C;

        /// Get the properties of the texture resource.
        ///
        /// # Parameters
        ///  * `desc` - Pointer to a resource description (see [`D3D11_TEXTURE1D_DESC`]).
        fn get_desc(&mut self, desc: *mut D3D11_TEXTURE1D_DESC);
    }
);
