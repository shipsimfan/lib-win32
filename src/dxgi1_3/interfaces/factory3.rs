use crate::{
    com_interface,
    dxgi::{IDXGIFactory, IDXGIFactory1, IDXGIObject},
    dxgi1_2::IDXGIFactory2,
    unknwn::IUnknown,
    UINT,
};

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11")]
use crate::d3d11::{D3D11CreateDevice, D3D11CreateDeviceAndSwapChain};
#[allow(unused_imports)]
use crate::{
    dxgi::{CreateDXGIFactory, CreateDXGIFactory1},
    dxgi1_3::CreateDXGIFactory2,
};

com_interface!(
    /// Enables creating Microsoft DirectX Graphics Infrastructure (DXGI) objects.
    pub abstract IDXGIFactory3(IDXGIFactory3VTable):
        IDXGIFactory2(factory2) +
        IDXGIFactory1 +
        IDXGIFactory +
        IDXGIObject +
        IUnknown {
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
