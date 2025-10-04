use crate::{
    d3d11::{ID3D11Device, ID3D11DeviceTrait},
    immut_com_interface,
    unknwn::{IUnknown, IUnknownTrait},
};

immut_com_interface!(
    /// The device interface represents a virtual adapter; it is used to create resources.
    /// [`ID3D11Device1`] adds new methods to those in [`ID3D11Device`].
    pub abstract ID3D11Device1(ID3D11Device1VTable/ID3D11Device1Trait):
        ID3D11Device/ID3D11DeviceTrait(device) +
        IUnknown/IUnknownTrait(device.unknown) {
        const IID = 0xA04BFB29-0x08EF-0x43D6-0xA49C-0xA9BDBDCBE686;
    }
);
