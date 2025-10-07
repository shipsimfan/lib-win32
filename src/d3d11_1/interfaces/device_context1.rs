use crate::{
    com_interface,
    d3d11::{
        ID3D11DeviceChild, ID3D11DeviceChildTrait, ID3D11DeviceContext, ID3D11DeviceContextTrait,
    },
    unknwn::{IUnknown, IUnknownTrait},
};

com_interface!(
    /// The device context interface represents a device context; it is used to render commands.
    /// [`ID3D11DeviceContext1`] adds new methods to those in [`ID3D11DeviceContext`].
    pub abstract ID3D11DeviceContext1(ID3D11DeviceContext1VTable/ID3D11DeviceContext1Trait):
        ID3D11DeviceContext/ID3D11DeviceContextTrait(device_context) +
        ID3D11DeviceChild/ID3D11DeviceChildTrait(device_context.device_child) +
        IUnknown/IUnknownTrait(device_context.device_child.unknown) {
        const IID = 0xBB2C6FAA-0xB5FB-0x4082-0x8E6B-0x388B8CFA90E1;
    }
);
