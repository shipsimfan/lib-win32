// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi1_2::IDXGIDevice2, dxgi1_5::IDXGIDevice4};

/// Specifies result flags for the [`IDXGIDevice4::reclaim_resources1`] method.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_RECLAIM_RESOURCE_RESULTS {
    /// The surface was successfully reclaimed and has valid content. This result is identical to
    /// the false value returned by the older [`IDXGIDevice2::reclaim_resources`] API.
    Ok = 0,

    /// The surface was reclaimed, but the old content was lost and must be regenerated. This
    /// result is identical to the true value returned by the older
    /// [`IDXGIDevice2::reclaim_resources`] API.
    Discarded = 1,

    /// Both the surface and its contents are lost and invalid. The surface must be recreated and
    /// the content regenerated in order to be used. All future use of that resource is invalid.
    /// Attempts to bind it to the pipeline or map a resource which returns this value will never
    /// succeed, and the resource cannot be reclaimed again.
    NotCommitted = 2,
}
