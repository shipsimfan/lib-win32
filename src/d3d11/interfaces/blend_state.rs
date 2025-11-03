use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, D3D11_BLEND_DESC},
    unknwn::IUnknown,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// The blend-state interface holds a description for blending state that you can bind to the
    /// output-merger stage.
    ///
    /// # Remarks
    /// Blending applies a simple function to combine output values from a pixel shader with data
    /// in a render target. You have control over how the pixels are blended by using a predefined
    /// set of blending operations and preblending operations.
    ///
    /// To create a blend-state object, call [`ID3D11Device::create_blend_state`]. To bind the
    /// blend-state object to the output-merger stage, call
    /// [`ID3D11DeviceContext::om_set_blend_state`].
    pub abstract ID3D11BlendState(ID3D11BlendStateVTable):
        ID3D11DeviceChild(device_child) +
        IUnknown {
        const IID = 0x75B68FAA-0x347D-0x4159-0x8F45-0xA0640F01CD9A;

        /// Gets the description for blending state that you used to create the blend-state object.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_BLEND_DESC`] structure that receives a description
        ///             of the blend state.
        ///
        /// # Remarks
        /// You use the description for blending state in a call to the
        /// [`ID3D11Device::create_blend_state`] method to create the blend-state object.
        fn get_desc(&mut self, desc: *mut D3D11_BLEND_DESC);
    }
);
