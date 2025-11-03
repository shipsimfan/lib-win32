use crate::{HRESULT, REFIID};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{CreateDXGIFactory1, IDXGIFactory},
    S_OK,
};

#[link(name = "DXGI")]
extern "system" {
    /// Creates a DXGI 1.0 factory that you can use to generate other DXGI objects.
    ///
    /// # Parameters
    ///  * `iid` - The globally unique identifier (GUID) of the [`IDXGIFactory`] object referenced
    ///            by the `factory` parameter.
    ///  * `factory` - Address of a pointer to an [`IDXGIFactory`] object.
    ///
    /// # Return Value
    /// Returns [`S_OK`] if successful; otherwise, returns a `DXGI_ERROR`.
    ///
    /// # Remarks
    /// Use a DXGI factory to generate objects that enumerate adapters, create swap chains, and
    /// associate a window with the `alt+enter` key sequence for toggling to and from the
    /// fullscreen display mode.
    ///
    /// If the [`CreateDXGIFactory`] function succeeds, the reference count on the [`IDXGIFactory`]
    /// interface is incremented. To avoid a memory leak, when you finish using the interface, call
    /// the [`IDXGIFactory::release`] method to release the interface.
    ///
    /// The [`CreateDXGIFactory`] function does not exist for Windows Store apps. Instead, Windows
    /// Store apps use the [`CreateDXGIFactory1`] function.
    pub fn CreateDXGIFactory(iid: REFIID, factory: *mut *mut c_void) -> HRESULT;
}
