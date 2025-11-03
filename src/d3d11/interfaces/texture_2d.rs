use crate::{
    com_interface,
    d3d11::{
        ID3D11DeviceChild, ID3D11Resource,
        D3D11_TEXTURE2D_DESC,
    },
    unknwn::{IUnknown},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Device;

com_interface!(
    /// A 2D texture interface manages texel data, which is structured memory.
    ///
    /// # Remarks
    /// To create an empty Texture2D resource, call [`ID3D11Device::create_texture_2d`].
    ///
    /// Textures cannot be bound directly to the pipeline; instead, a view must be created and
    /// bound. Using a view, texture data can be interpreted at run time within certain
    /// restrictions. To use the texture as a render target or depth-stencil resource, call
    /// [`ID3D11Device::create_render_target_view`], and
    /// [`ID3D11Device::create_depth_stencil_view`], respectively. To use the texture as an input
    /// to a shader, create a by calling [`ID3D11Device::create_shader_resource_view`].
    pub abstract ID3D11Texture2D(ID3D11Texture2DVTable):
        ID3D11Resource(resource) +
        ID3D11DeviceChild +
        IUnknown {
        const IID = 0x6F15AAF2-0xD208-0x4E89-0x9AB4-0x489535D34F9C;

        /// Get the properties of the texture resource.
        ///
        /// # Parameters
        ///  * `desc` - Pointer to a resource description (see [`D3D11_TEXTURE2D_DESC`]).
        fn get_desc(&mut self, desc: *const D3D11_TEXTURE2D_DESC);
    }
);
