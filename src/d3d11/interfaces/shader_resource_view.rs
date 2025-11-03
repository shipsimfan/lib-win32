use crate::{
    com_interface,
    d3d11::{
        ID3D11DeviceChild, ID3D11View,
        D3D11_SHADER_RESOURCE_VIEW_DESC,
    },
    unknwn::{IUnknown},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// A shader-resource-view interface specifies the subresources a shader can access during
    /// rendering. Examples of shader resources include a constant buffer, a texture buffer, and a
    /// texture.
    ///
    /// # Remarks
    /// To create a shader-resource view, call [`ID3D11Device::create_shader_resource_view`].
    ///
    /// A shader-resource view is required when binding a resource to a shader stage; the binding
    /// occurs by calling [`ID3D11DeviceContext::gs_set_shader_resources`],
    /// [`ID3D11DeviceContext::vs_set_shader_resources`] or
    /// [`ID3D11DeviceContext::ps_set_shader_resources`].
    pub abstract ID3D11ShaderResourceView(ID3D11ShaderResourceViewVTable):
        ID3D11View(view) +
        ID3D11DeviceChild +
        IUnknown {
        const IID = 0xB0E06FE0-0x8192-0x4E1A-0xB1CA-0x36D7414710B2;

        /// Get the shader resource view's description.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_SHADER_RESOURCE_VIEW_DESC`] structure to be filled
        ///             with data about the shader resource view.
        fn get_desc(&mut self, desc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC);
    }
);
