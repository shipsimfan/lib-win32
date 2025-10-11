use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, ID3D11DeviceChildTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// An input-layout interface holds a definition of how to feed vertex data that is laid out in
    /// memory into the input-assembler stage of the graphics pipeline.
    ///
    /// # Remarks
    /// To create an input-layout object, call [`ID3D11Device::create_input_layout`]. To bind the
    /// input-layout object to the input-assembler stage, call
    /// [`ID3D11DeviceContext::ia_set_input_layout`].
    pub abstract ID3D11InputLayout(ID3D11InputLayoutVTable/ID3D11InputLayoutTrait):
        ID3D11DeviceChild/ID3D11DeviceChildTrait(device_child) +
        IUnknown/IUnknownTrait(device_child.unknown) {
        const IID = 0xE4819DDC-0x4CF0-0x4025-0xBD26-0x5DE82A3E07B7;
    }
);
