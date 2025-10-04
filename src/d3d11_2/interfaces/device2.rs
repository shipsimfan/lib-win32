use crate::{
    d3d11::{ID3D11Device, ID3D11DeviceTrait},
    d3d11_1::{ID3D11Device1, ID3D11Device1Trait},
    immut_com_interface,
    unknwn::{IUnknown, IUnknownTrait},
};

immut_com_interface!(
    /// The device interface represents a virtual adapter; it is used to create resources.
    /// [`ID3D11Device2`] adds new methods to those in [`ID3D11Device1`].
    pub abstract ID3D11Device2(ID3D11Device2VTable/ID3D11Device2Trait):
        ID3D11Device1/ID3D11Device1Trait(device1) +
        ID3D11Device/ID3D11DeviceTrait(device1.device) +
        IUnknown/IUnknownTrait(device1.device.unknown) {
        const IID = 0x9D06DFFA-0xD1E5-0x4D07-0x83A8-0x1BB123F2F841;
    }
);
