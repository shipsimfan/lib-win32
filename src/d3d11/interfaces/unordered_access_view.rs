use crate::{
    com_interface,
    d3d11::{
        ID3D11DeviceChild, ID3D11View,
        D3D11_UNORDERED_ACCESS_VIEW_DESC,
    },
    unknwn::{IUnknown},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// A view interface specifies the parts of a resource the pipeline can access during rendering.
    ///
    /// # Remarks
    /// To create a view for an unordered access resource, call
    /// [`ID3D11Device::create_unordered_access_view`].
    ///
    /// All resources must be bound to the pipeline before they can be accessed. Call
    /// [`ID3D11DeviceContext::cs_set_unordered_access_views`] to bind an unordered access view to
    /// a compute shader; call
    /// [`ID3D11DeviceContext::om_set_render_targets_and_unordered_access_views`] to bind an
    /// unordered access view to a pixel shader.
    pub abstract ID3D11UnorderedAccessView(ID3D11UnorderedAccessViewVTable):
        ID3D11View(view) +
        ID3D11DeviceChild +
        IUnknown {
        const IID = 0x28ACF509-0x7F5C-0x48F6-0x8611-0xF316010A6380;

        /// Get a description of the resource.
        ///
        /// # Parameters
        ///  * `desc` - Pointer to a resource description (see
        ///             [`D3D11_UNORDERED_ACCESS_VIEW_DESC`].)
        fn get_desc(&mut self, desc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC);
    }
);
