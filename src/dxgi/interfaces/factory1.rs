use crate::{
    com_interface,
    dxgi::{IDXGIAdapter1, IDXGIFactory, IDXGIObject},
    unknwn::{IUnknown},
    BOOL, HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{CreateDXGIFactory1, IDXGIDevice, IDXGIDevice1},
    DXGI_ERROR_INVALID_CALL, DXGI_ERROR_NOT_FOUND, FALSE, S_OK, TRUE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// The [`IDXGIFactory1`] interface implements methods for generating DXGI objects.
    ///
    /// # Remarks
    /// This interface is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
    /// Server 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows Server
    /// 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows Server
    /// 2008.
    ///
    /// To create a factory, call the [`CreateDXGIFactory1`] function.
    ///
    /// Because you can create a Direct3D device without creating a swap chain, you might need to
    /// retrieve the factory that is used to create the device in order to create a swap chain. You
    /// can request the [`IDXGIDevice`] or [`IDXGIDevice1`] interface from the Direct3D device and
    /// then use the [`IDXGIObject::get_parent`] method to locate the factory.
    pub abstract IDXGIFactory1(IDXGIFactory1VTable):
        IDXGIFactory(factory) +
        IDXGIObject +
        IUnknown {
        const IID = 0x770AAE78-0xF26F-0x4DBA-0xA829-0x253C83D1B387;

        /// Enumerates both adapters (video cards) with or without outputs.
        ///
        /// # Parameters
        ///  * `adapter` - The index of the adapter to enumerate.
        ///  * `p_adapter` - The address of a pointer to an [`IDXGIAdapter1`] interface at the
        ///                  position specified by the Adapter parameter. This parameter must not
        ///                  be [`null_mut`].
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns [`DXGI_ERROR_NOT_FOUND`] if the
        /// index is greater than or equal to the number of adapters in the local system, or
        /// [`DXGI_ERROR_INVALID_CALL`] if `p_adapter` parameter is [`null_mut`].
        ///
        /// # Remarks
        /// This method is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
        /// sServer 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows
        /// Server 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows
        /// Server 2008.
        ///
        /// When you create a factory, the factory enumerates the set of adapters that are
        /// available in the system. Therefore, if you change the adapters in a system, you must
        /// destroy and recreate the [`IDXGIFactory1`] object. The number of adapters in a system
        /// changes when you add or remove a display card, or dock or undock a laptop.
        ///
        /// When the [`IDXGIFactory1::enum_adapters1`] method succeeds and fills the `p_adapter`
        /// parameter with the address of the pointer to the adapter interface,
        /// [`IDXGIFactory1::enum_adapters1`] increments the adapter interface's reference count.
        /// When you finish using the adapter interface, call the [`IDXGIFactory1::release`] method
        /// to decrement the reference count before you destroy the pointer.
        ///
        /// [`IDXGIFactory1::enum_adapters1`] first returns the adapter with the output on which
        /// the desktop primary is displayed. This adapter corresponds with an index of zero.
        /// [`IDXGIFactory1::enum_adapters1`] next returns other adapters with outputs. 4
        /// [`IDXGIFactory1::enum_adapters1`] finally returns adapters without outputs.
        fn enum_adapters1(&mut self, adapter: UINT, p_adapter: *mut *mut IDXGIAdapter1) -> HRESULT;

        /// Informs an application of the possible need to re-create the factory and re-enumerate
        /// adapters.
        ///
        /// # Return Value
        /// [`FALSE`], if a new adapter is becoming available or the current adapter is going away.
        /// [`TRUE`], no adapter changes.
        ///
        /// [`IDXGIFactory1::is_current`] returns [`FALSE`] to inform the calling application to
        /// re-enumerate adapters.
        ///
        /// # Remarks
        /// This method is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
        /// Server 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows
        /// Server 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows
        /// Server 2008.
        fn is_current(&mut self) -> BOOL;
    }
);
