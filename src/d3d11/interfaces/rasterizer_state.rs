use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, D3D11_RASTERIZER_DESC},
    unknwn::{IUnknown},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// The rasterizer-state interface holds a description for rasterizer state that you can bind
    /// to the rasterizer stage.
    ///
    /// # Remarks
    /// To create a rasterizer-state object, call [`ID3D11Device::create_rasterizer_state`]. To
    /// bind the rasterizer-state object to the rasterizer stage, call
    /// [`ID3D11DeviceContext::rs_set_state`].
    pub abstract ID3D11RasterizerState(ID3D11RasterizerStateVTable):
        ID3D11DeviceChild(device_child) +
        IUnknown {
        const IID = 0x9BB4AB81-0xAB1A-0x4D8F-0xB506-0xFC04200B6EE7;

        /// Gets the description for rasterizer state that you used to create the rasterizer-state
        /// object.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_RASTERIZER_DESC`] structure that receives a
        ///             description of the rasterizer state.
        ///
        /// # Remarks
        /// You use the description for rasterizer state in a call to the
        /// [`ID3D11Device::create_rasterizer_state`] method to create the rasterizer-state object.
        fn get_desc(&mut self, desc: *mut D3D11_RASTERIZER_DESC);
    }
);
