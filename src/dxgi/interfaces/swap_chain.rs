use crate::{
    com_interface,
    dxgi::{IDXGIDeviceSubObject, IDXGIDeviceSubObjectTrait, IDXGIObject, IDXGIObjectTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

com_interface!(
    /// An [`IDXGISwapChain`] interface implements one or more surfaces for storing rendered data
    /// before presenting it to an output.
    ///
    /// # Remarks
    /// You can create a swap chain by calling [`IDXGIFactory2::create_swap_chain_for_hwnd`],
    /// [`IDXGIFactory2::create_swap_chain_for_core_window`], or
    /// [`IDXGIFactory2::create_swap_chain_for_composition`]. You can also create a swap chain when
    /// you call [`D3D11CreateDeviceAndSwapChain`]; however, you can then only access the sub-set
    /// of swap-chain functionality that the [`IDXGISwapChain`] interface provides.
    pub abstract IDXGISwapChain(IDXGIFactoryVTable/IDXGIFactoryTrait):
        IDXGIDeviceSubObject/IDXGIDeviceSubObjectTrait(device_sub_object) +
        IDXGIObject/IDXGIObjectTrait(device_sub_object.object) +
        IUnknown/IUnknownTrait(device_sub_object.object.unknown) {
        const IID = 0x310D36A0-0xD2E7-0x4C0A-0xAA04-0x6A9D23B8886A;
    }
);
