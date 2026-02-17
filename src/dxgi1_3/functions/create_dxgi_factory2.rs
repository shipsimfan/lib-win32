use crate::{HRESULT, REFIID, UINT};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::CreateDXGIFactory1, dxgi1_2::IDXGIFactory2, dxgi1_3::DXGI_CREATE_FACTORY_DEBUG, S_OK,
};

#[link(name = "DXGI")]
unsafe extern "system" {
    /// Creates a DXGI 1.3 factory that you can use to generate other DXGI objects.
    ///
    /// In Windows 8, any DXGI factory created while `DXGIDebug.dll` was present on the system
    /// would load and use it. Starting in Windows 8.1, apps explicitly request that
    /// `DXGIDebug.dll` be loaded instead. Use [`CreateDXGIFactory2`] and specify the
    /// [`DXGI_CREATE_FACTORY_DEBUG`] flag to request `DXGIDebug.dll`; the DLL will be loaded if it
    /// is present on the system.
    ///
    /// # Parameters
    ///  * `flags` - Valid values include the [`DXGI_CREATE_FACTORY_DEBUG`] flag, and zero.
    ///  * `iid` - The globally unique identifier (GUID) of the [`IDXGIFactory2`] object referenced
    ///            by the `factory` parameter.
    ///  * `factory` - Address of a pointer to an [`IDXGIFactory2`] interface.
    ///
    /// # Return Value
    /// Returns [`S_OK`] if successful; an error code otherwise.
    ///
    /// # Remarks
    /// This function accepts a flag indicating whether `DXGIDebug.dll` is loaded. The function
    /// otherwise behaves identically to [`CreateDXGIFactory1`].
    pub fn CreateDXGIFactory2(flags: UINT, iid: REFIID, factory: *mut *mut c_void) -> HRESULT;
}
