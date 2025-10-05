use crate::{
    com_interface,
    d3d11::{
        ID3D11DeviceChild, ID3D11DeviceChildTrait, ID3D11Resource, ID3D11ResourceTrait,
        D3D11_TEXTURE3D_DESC,
    },
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Device;

com_interface!(
    /// A 3D texture interface accesses texel data, which is structured memory.
    ///
    /// # Remarks
    /// To create an empty Texture 3D resource, call [`ID3D11Device::create_texture_3d`].
    ///
    /// Textures cannot be bound directly to the pipeline; instead, a view must be created and
    /// bound. Using a view, texture data can be interpreted at run time within certain
    /// restrictions. To use the texture as a render target or depth-stencil resource, call
    /// [`ID3D11Device::create_render_target_view`], and
    /// [`ID3D11Device::create_depth_stencil_view`], respectively. To use the texture as an input
    /// to a shader, create a by calling [`ID3D11Device::create_shader_resource_view`].
    pub abstract ID3D11Texture3D(ID3D11Texture3DVTable/ID3D11Texture3DTrait):
        ID3D11Resource/ID3D11ResourceTrait(resource) +
        ID3D11DeviceChild/ID3D11DeviceChildTrait(resource.device_child) +
        IUnknown/IUnknownTrait(resource.device_child.unknown) {
        const IID = 0x037E866E-0xF56D-0x4357-0xA8AF-0x9DABBE6E250E;

        /// Get the properties of the texture resource.
        ///
        /// # Parameters
        ///  * `desc` - Pointer to a resource description (see [`D3D11_TEXTURE3D_DESC`]).
        fn get_desc(&mut self, desc: *const D3D11_TEXTURE3D_DESC);
    }
);
