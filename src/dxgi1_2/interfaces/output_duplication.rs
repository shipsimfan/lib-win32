use crate::{
    com_interface,
    dxgi::{IDXGIObject, IDXGIObjectTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

com_interface!(
    /// The [`IDXGIOutputDuplication`] interface accesses and manipulates the duplicated desktop
    /// image.
    pub abstract IDXGIOutputDuplication(IDXGIOutputDuplicationVTable/IDXGIOutputDuplicationTrait):
        IDXGIObject/IDXGIObjectTrait(object) +
        IUnknown/IUnknownTrait(object.unknown) {
        const IID = 0x191CFAC3-0xA341-0x470D-0xB26E-0xA864F428319C;
    }
);
