use crate::{
    com_interface,
    d3d11::{
        ID3D11DeviceChild, ID3D11DeviceChildTrait, ID3D11View, ID3D11ViewTrait,
        D3D11_RENDER_TARGET_VIEW_DESC,
    },
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// A render-target-view interface identifies the render-target subresources that can be
    /// accessed during rendering.
    ///
    /// # Remarks
    /// To create a render-target view, call [`ID3D11Device::create_render_target_view`]. To bind a
    /// render-target view to the pipeline, call [`ID3D11DeviceContext::om_set_render_targets`].
    ///
    /// A render target is a resource that can be written by the output-merger stage at the end of
    /// a render pass. Each render-target should also have a corresponding depth-stencil view.
    pub abstract ID3D11RenderTargetView(ID3D11RenderTargetViewVTable/ID3D11RenderTargetViewTrait):
        ID3D11View/ID3D11ViewTrait(view) +
        ID3D11DeviceChild/ID3D11DeviceChildTrait(view.device_child) +
        IUnknown/IUnknownTrait(view.device_child.unknown) {
        const IID = 0xDFDBA067-0x0B8D-0x4865-0x875B-0xD7B4516CC164;

        /// Get the properties of a render target view.
        ///
        /// # Parameters
        ///  * `desc` - Pointer to the description of a render target view (see
        ///             [`D3D11_RENDER_TARGET_VIEW_DESC`]).
        fn get_desc(&mut self, desc: *const D3D11_RENDER_TARGET_VIEW_DESC);
    }
);
