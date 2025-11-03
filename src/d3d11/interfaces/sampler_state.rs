use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, D3D11_SAMPLER_DESC},
    unknwn::{IUnknown},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// The sampler-state interface holds a description for sampler state that you can bind to any
    /// shader stage of the pipeline for reference by texture sample operations.
    ///
    /// # Remarks
    /// To create a sampler-state object, call [`ID3D11Device::create_sampler_state`].
    ///
    /// To bind a sampler-state object to any pipeline shader stage, call the following methods:
    ///  - [`ID3D11DeviceContext::vs_set_samplers`]
    ///  - [`ID3D11DeviceContext::hs_set_samplers`]
    ///  - [`ID3D11DeviceContext::ds_set_samplers`]
    ///  - [`ID3D11DeviceContext::gs_set_samplers`]
    ///  - [`ID3D11DeviceContext::ps_set_samplers`]
    ///  - [`ID3D11DeviceContext::cs_set_samplers`]
    ///
    /// You can bind the same sampler-state object to multiple shader stages simultaneously.
    pub abstract ID3D11SamplerState(ID3D11SamplerStateVTable):
        ID3D11DeviceChild(device_child) +
        IUnknown {
        const IID = 0xDA6FEA51-0x564C-0x4487-0x9810-0xF0D0F9B4E3A5;

        /// Gets the description for sampler state that you used to create the sampler-state object.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_SAMPLER_DESC`] structure that receives a description
        ///             of the sampler state.
        ///
        /// # Remarks
        /// You use the description for sampler state in a call to the
        /// [`ID3D11Device::create_sampler_state`] method to create the sampler-state object.
        fn get_desc(&mut self, desc: *mut D3D11_SAMPLER_DESC);
    }
);
