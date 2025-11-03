use crate::{
    com_interface,
    d3d11::{
        ID3D11DeviceChild, ID3D11DeviceContext,
    },
    unknwn::{IUnknown},
};

com_interface!(
    /// The device context interface represents a device context; it is used to render commands.
    /// [`ID3D11DeviceContext1`] adds new methods to those in [`ID3D11DeviceContext`].
    pub abstract ID3D11DeviceContext1(ID3D11DeviceContext1VTable):
        ID3D11DeviceContext(device_context) +
        ID3D11DeviceChild +
        IUnknown {
        const IID = 0xBB2C6FAA-0xB5FB-0x4082-0x8E6B-0x388B8CFA90E1;
    }
);
