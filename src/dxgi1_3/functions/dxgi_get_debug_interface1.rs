use crate::{HRESULT, REFIID, UINT};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "dxgidebug")]
use crate::dxgidebug::IDXGIInfoQueue;
#[allow(unused_imports)]
use crate::{E_NOINTERFACE, GUID, S_OK};

#[link(name = "DXGI")]
extern "system" {
    /// Retrieves an interface that Windows Store apps use for debugging the Microsoft DirectX
    /// Graphics Infrastructure (DXGI).
    ///
    /// # Parameters
    ///  * `flags` - Not used.
    ///  * `iid` - The globally unique identifier ([`GUID`]) of the requested interface type, which
    ///            can be the identifier for the [`IDXGIDebug`], [`IDXGIDebug1`], or
    ///            [`IDXGIInfoQueue`] interfaces.
    ///  * `debug` - A pointer to a buffer that receives a pointer to the debugging interface.
    ///
    /// # Return Value
    /// If this function succeeds, it returns [`S_OK`]. Otherwise, it returns an [`HRESULT`] error
    /// code.
    ///
    /// # Remarks
    /// The [`DXGIGetDebugInterface1`] function returns [`E_NOINTERFACE`] on systems without the
    /// Windows Software Development Kit (SDK) installed, because it's a development-time aid.
    pub fn DXGIGetDebugInterface1(flags: UINT, iid: REFIID, debug: *mut *mut c_void) -> HRESULT;
}
