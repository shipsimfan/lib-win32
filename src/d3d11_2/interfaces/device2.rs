use crate::{
    d3d11::{ID3D11Device},
    d3d11_1::{ID3D11Device1},
    immut_com_interface,
    unknwn::{IUnknown},
};

immut_com_interface!(
    /// The device interface represents a virtual adapter; it is used to create resources.
    /// [`ID3D11Device2`] adds new methods to those in [`ID3D11Device1`].
    pub abstract ID3D11Device2(ID3D11Device2VTable):
        ID3D11Device1(device1) +
        ID3D11Device +
        IUnknown {
        const IID = 0x9D06DFFA-0xD1E5-0x4D07-0x83A8-0x1BB123F2F841;
    }
);
