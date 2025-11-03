use crate::{
    com_interface,
    dxgi::{IDXGIAdapter, IDXGIObject, DXGI_ADAPTER_DESC1},
    unknwn::{IUnknown},
    HRESULT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGIDevice, IDXGIFactory, IDXGIFactory1},
    E_INVALIDARG, S_OK,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// The [`IDXGIAdapter1`] interface represents a display sub-system (including one or more
    /// GPU's, DACs and video memory).
    ///
    /// # Remarks
    /// This interface is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
    /// Server 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows Server
    /// 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows Server
    /// 2008.
    ///
    /// A display sub-system is often referred to as a video card, however, on some machines the
    /// display sub-system is part of the mother board.
    ///
    /// To enumerate the display sub-systems, use [`IDXGIFactory1::enum_adapters1`]. To get an
    /// interface to the adapter for a particular device, use [`IDXGIDevice::get_adapter`]. To
    /// create a software adapter, use [`IDXGIFactory::create_software_adapter`].
    pub abstract IDXGIAdapter1(IDXGIAdapter1VTable):
        IDXGIAdapter(adapter) +
        IDXGIObject +
        IUnknown {
        const IID = 0x29038F61-0x3839-0x4626-0x91FD-0x086879011A05;

        /// Gets a DXGI 1.1 description of an adapter (or video card).
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`DXGI_ADAPTER_DESC1`] structure that describes the adapter.
        ///             This parameter must not be [`null_mut`]. On feature level 9 graphics
        ///             hardware, [`IDXGIAdapter1::get_desc1`] returns zeros for `vendor_id`,
        ///             `device_id`, `sub_sys_id`, and `revision` members of [`DXGI_ADAPTER_DESC1`]
        ///             and “Software Adapter” for the description string in the `description`
        ///             member.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns [`E_INVALIDARG`] if the `desc`
        /// parameter is [`null_mut`].
        ///
        /// # Remarks
        /// This method is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
        /// Server 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows
        /// Server 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows
        /// Server 2008.
        ///
        /// Use the [`IDXGIAdapter1::get_desc1`] method to get a DXGI 1.1 description of an
        /// adapter. To get a DXGI 1.0 description, use the [`IDXGIAdapter`] method.
        fn get_desc1(&mut self, desc: *mut DXGI_ADAPTER_DESC1) -> HRESULT;
    }
);
