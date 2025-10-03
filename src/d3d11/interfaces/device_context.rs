use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, ID3D11DeviceChildTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

com_interface!(
    /// The [`ID3D11DeviceContext`] interface represents a device context which generates rendering
    /// commands.
    pub abstract ID3D11DeviceContext(ID3D11DeviceContextVTable/ID3D11DeviceContextTrait):
        ID3D11DeviceChild/ID3D11DeviceChildTrait(device_child) +
        IUnknown/IUnknownTrait(device_child.unknown) {
        const IID = 0xC0BFA96C-0xE089-0x44FB-0x8EAF-0x26F8796190DA;
    }
);
