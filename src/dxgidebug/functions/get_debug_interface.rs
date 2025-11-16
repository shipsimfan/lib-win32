use crate::{HRESULT, REFIID};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgidebug::IDXGIInfoQueue, GetModuleHandle, GetProcAddress, GUID, S_OK};

#[link(name = "Dxgidebug")]
extern "system" {
    /// Retrieves a debugging interface.
    ///
    /// # Parameters
    ///  * `iid` - The globally unique identifier ([`GUID`]) of the requested interface type.
    ///  * `debug` - A pointer to a buffer that receives a pointer to the debugging interface.
    ///
    /// # Return Value
    /// Returns [`S_OK`] if successful; an error code otherwise.
    ///
    /// # Remarks
    /// [`IDXGIDebug`] and [`IDXGIInfoQueue`] are debugging interfaces.
    ///
    /// To access [`DXGIGetDebugInterface`], call the [`GetModuleHandle`] function to get
    /// `Dxgidebug.dll` and the [``] function to get the address of [`DXGIGetDebugInterface`].
    ///
    /// Windows 8.1:  Starting in Windows 8.1, Windows Store apps call the
    /// [`DXGIGetDebugInterface1`] function to get an [`IDXGIDebug1`] interface.
    pub fn DXGIGetDebugInterface(iid: REFIID, debug: *mut *mut c_void) -> HRESULT;
}
