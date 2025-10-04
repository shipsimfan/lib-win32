use crate::{
    com_interface,
    dxgi::{
        IDXGIFactory, IDXGIFactory1, IDXGIFactory1Trait, IDXGIFactoryTrait, IDXGIObject,
        IDXGIObjectTrait,
    },
    dxgi1_2::{IDXGIFactory2, IDXGIFactory2Trait},
    unknwn::{IUnknown, IUnknownTrait},
    UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{D3D11CreateDevice, D3D11CreateDeviceAndSwapChain},
    dxgi::{CreateDXGIFactory, CreateDXGIFactory1},
    dxgi1_3::CreateDXGIFactory2,
};

com_interface!(
    /// Enables creating Microsoft DirectX Graphics Infrastructure (DXGI) objects.
    pub abstract IDXGIFactory3(IDXGIFactory3VTable/IDXGIFactory3Trait):
        IDXGIFactory2/IDXGIFactory2Trait(factory2) +
        IDXGIFactory1/IDXGIFactory1Trait(factory2.factory1) +
        IDXGIFactory/IDXGIFactoryTrait(factory2.factory1.factory) +
        IDXGIObject/IDXGIObjectTrait(factory2.factory1.factory.object) +
        IUnknown/IUnknownTrait(factory2.factory1.factory.object.unknown) {
        const IID = 0x25483823-0xCD46-0x4C7D-0x86CA-0x47AA95B837BD;

        /// Gets the flags that were used when a Microsoft DirectX Graphics Infrastructure (DXGI)
        /// object was created.
        ///
        /// # Return Value
        /// The creation flags.
        ///
        /// # Remarks
        /// The [`IDXGIFactory3::get_creation_flags`] method returns flags that were passed to the
        /// [`CreateDXGIFactory2`] function, or were implicitly constructed by
        /// [`CreateDXGIFactory`], [`CreateDXGIFactory1`], [`D3D11CreateDevice`], or
        /// [`D3D11CreateDeviceAndSwapChain`].
        fn get_creation_flags(&mut self) -> UINT;
    }
);
