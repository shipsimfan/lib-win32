use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, D3D11_DEPTH_STENCIL_DESC},
    unknwn::{IUnknown},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// The depth-stencil-state interface holds a description for depth-stencil state that you can
    /// bind to the output-merger stage.
    ///
    /// # Remarks
    /// To create a depth-stencil-state object, call [`ID3D11Device::create_depth_stencil_state`].
    /// To bind the depth-stencil-state object to the output-merger stage, call
    /// [`ID3D11DeviceContext::om_set_depth_stencil_state`].
    pub abstract ID3D11DepthStencilState(ID3D11DepthStencilStateVTable):
        ID3D11DeviceChild(device_child) +
        IUnknown {
        const IID = 0x03823EFB-0x8D8F-0x4E1C-0x9AA2-0xF64BB2CBFDF1;

        /// Gets the description for depth-stencil state that you used to create the
        /// depth-stencil-state object.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_DEPTH_STENCIL_DESC`] structure that receives a
        ///             description of the depth-stencil state.
        ///
        /// # Remarks
        /// You use the description for depth-stencil state in a call to the
        /// [`ID3D11Device::create_depth_stencil_state`] method to create the depth-stencil-state
        /// object.
        fn get_desc(&mut self, desc: *mut D3D11_DEPTH_STENCIL_DESC);
    }
);
