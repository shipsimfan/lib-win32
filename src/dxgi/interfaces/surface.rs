use crate::{
    com_interface,
    dxgi::{IDXGIDeviceSubObject, IDXGIDeviceSubObjectTrait, IDXGIObject, IDXGIObjectTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

com_interface!(
    /// The [`IDXGISurface`] interface implements methods for image-data objects.
    pub abstract IDXGISurface(IDXGISurfaceVTable/IDXGISurfaceTrait):
        IDXGIDeviceSubObject/IDXGIDeviceSubObjectTrait(device_sub_object) +
        IDXGIObject/IDXGIObjectTrait(device_sub_object.object) +
        IUnknown/IUnknownTrait(device_sub_object.object.unknown) {
        const IID = 0xCAFCB56C-0x6AC3-0x4889-0xBF47-0x9E23BBD260EC;
    }
);
