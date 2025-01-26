use crate::{
    com_interface,
    dxgi::{IDXGIObject, IDXGIObjectTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::IDXGIFactory;

com_interface!(
    /// The [`IDXGIAdapter`] interface represents a display subsystem (including one or more GPUs,
    /// DACs and video memory).
    ///
    /// # Remarks
    /// A display subsystem is often referred to as a video card, however, on some machines the
    /// display subsystem is part of the motherboard.
    ///
    /// To enumerate the display subsystems, use [`IDXGIFactory::enum_adapters`].
    ///
    /// To get an interface to the adapter for a particular device, use
    /// [`IDXGIDevice::get_adapter`].
    ///
    /// To create a software adapter, use [`IDXGIFactory::create_software_adapter`].
    ///
    /// Windows Phone 8: This API is supported.
    pub abstract IDXGIAdapter(IDXGIAdapterVTable/IDXGIAdapterTrait):
        IDXGIObject/IDXGIObjectTrait(object) +
        IUnknown/IUnknownTrait(object.unknown) {
        const IID = 0x2411E7E1-0x12AC-0x4CCF-0xBD14-0x9798E8534DC0;
    }
);
