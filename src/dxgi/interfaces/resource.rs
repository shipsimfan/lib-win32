use crate::{
    com_interface,
    dxgi::{IDXGIDeviceSubObject, IDXGIDeviceSubObjectTrait, IDXGIObject, IDXGIObjectTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

com_interface!(
    /// An [`IDXGIResource`] interface allows resource sharing and identifies the memory that a
    /// resource resides in.
    pub abstract IDXGIResource(IDXGIResourceVTable/IDXGIResourceTrait):
        IDXGIDeviceSubObject/IDXGIDeviceSubObjectTrait(device_sub_object) +
        IDXGIObject/IDXGIObjectTrait(device_sub_object.object) +
        IUnknown/IUnknownTrait(device_sub_object.object.unknown) {
        const IID = ;
    }
);
