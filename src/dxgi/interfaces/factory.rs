use crate::{
    com_interface,
    dxgi::{IDXGIObject, IDXGIObjectTrait},
};

com_interface!(
    /// An [`IDXGIFactory`] interface implements methods for generating DXGI objects (which handle
    /// full screen transitions).
    ///
    /// Create a factory by calling [`CreateDXGIFactory`].
    ///
    /// Because you can create a Direct3D device without creating a swap chain, you might need to
    /// retrieve the factory that is used to create the device in order to create a swap chain. You
    /// can request the [`IDXGIDevice`] interface from the Direct3D device and then use the
    /// [`IDXGIObject::get_parent`] method to locate the factory.
    pub abstract IDXGIFactory(IDXGIFactoryVTable/IDXGIFactoryTrait): IDXGIObject/IDXGIObjectTrait(object) {
        const IID = 0x7B7166EC-0x21C7-0x44AE-0xB21A-0xC9AE321AE369;
    }
);
