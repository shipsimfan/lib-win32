use crate::HANDLE;
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::IDXGIDevice;

/// Represents a handle to a shared resource.
///
/// To create a shared surface, pass a shared-resource handle into the
/// [`IDXGIDevice::create_surface`] method.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_SHARED_RESOURCE {
    /// A handle to a shared resource.
    pub handle: HANDLE,
}

impl Default for DXGI_SHARED_RESOURCE {
    fn default() -> Self {
        DXGI_SHARED_RESOURCE { handle: null_mut() }
    }
}
