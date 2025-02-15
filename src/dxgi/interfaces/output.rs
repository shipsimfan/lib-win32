use crate::{
    com_interface,
    dxgi::{IDXGIObject, IDXGIObjectTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

com_interface!(
    /// An [`IDXGIOutput`] interface represents an adapter output (such as a monitor).
    pub abstract IDXGIOutput(IDXGIOutputVTable/IDXGIOutputTrait):
        IDXGIObject/IDXGIObjectTrait(object) +
        IUnknown/IUnknownTrait(object.unknown) {
        const IID = 0xAE02EEDB-0xC735-0x4690-0x8D52-0x5A8DC20213AA;
    }
);
