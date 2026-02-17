use crate::{HRESULT, REFIID};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi::IDXGIFactory1, S_OK};

#[link(name = "DXGI")]
unsafe extern "system" {
    /// Creates a DXGI 1.1 factory that you can use to generate other DXGI objects.
    ///
    /// # Parameters
    ///  * `iid` - The globally unique identifier (GUID) of the [`IDXGIFactory1`] object referenced
    ///            by the `facotry` parameter.
    ///  * `factory` - Address of a pointer to an [`IDXGIFactory1`] object.
    ///
    /// # Return Value
    /// Returns [`S_OK`] if successful; an error code otherwise.
    ///
    /// # Remarks
    /// Use a DXGI 1.1 factory to generate objects that enumerate adapters, create swap chains, and
    /// associate a window with the alt+enter key sequence for toggling to and from the full-screen
    /// display mode.
    ///
    /// If the [`CreateDXGIFactory1`] function succeeds, the reference count on the
    /// [`IDXGIFactory1`] interface is incremented. To avoid a memory leak, when you finish using
    /// the interface, call the [`IDXGIFactory1::release`] method to release the interface.
    ///
    /// This entry point is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
    /// sServer 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows Server
    /// 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows Server
    /// 2008.
    pub fn CreateDXGIFactory1(iid: REFIID, factory: *mut *mut c_void) -> HRESULT;
}
