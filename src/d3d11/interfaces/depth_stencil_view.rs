use crate::{
    com_interface,
    d3d11::{
        ID3D11DeviceChild, ID3D11DeviceChildTrait, ID3D11View, ID3D11ViewTrait,
        D3D11_DEPTH_STENCIL_VIEW_DESC,
    },
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// A depth-stencil-view interface accesses a texture resource during depth-stencil testing.
    ///
    /// # Remarks
    /// To create a depth-stencil view, call [`ID3D11Device::create_depth_stencil_view`].
    ///
    /// To bind a depth-stencil view to the pipeline, call
    /// [`ID3D11DeviceContext::om_set_render_targets`].
    pub abstract ID3D11DepthStencilView(ID3D11DepthStencilViewVTable/ID3D11DepthStencilViewTrait):
        ID3D11View/ID3D11ViewTrait(view) +
        ID3D11DeviceChild/ID3D11DeviceChildTrait(view.device_child) +
        IUnknown/IUnknownTrait(view.device_child.unknown) {
        const IID = 0x9FDAC92A-0x1876-0x48C3-0xAFAD-0x25B94F84A9B6;

        /// Get the depth-stencil view.
        ///
        /// # Parameters
        ///  * `desc` - Pointer to a depth-stencil-view description (see
        ///             [`D3D11_DEPTH_STENCIL_VIEW_DESC`]).
        fn get_desc(&mut self, desc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC);
    }
);
