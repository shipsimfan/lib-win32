// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{D3D11_FEATURE_DATA_D3D11_OPTIONS, D3D11_FEATURE_DATA_D3D11_OPTIONS4},
    dxgi::DXGI_FORMAT,
    dxgi1_2::IDXGIResource1,
    FALSE, TRUE,
};

/// Defines constants that specify the level of support for shared resources in the current
/// graphics driver.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_SHARED_RESOURCE_TIER {
    /// Specifies the support available when
    /// [`D3D11_FEATURE_DATA_D3D11_OPTIONS::extended_resource_sharing`] is [`FALSE`] (only very old
    /// drivers have this value set to [`FALSE`]).
    _0 = 0,

    /// Specifies the support available when
    /// [`D3D11_FEATURE_DATA_D3D11_OPTIONS::extended_resource_sharing`] and
    /// [`D3D11_FEATURE_DATA_D3D11_OPTIONS4::extended_nv12_shared_texture_supported`] are [`TRUE`].
    ///
    /// You can share additional formats;.
    ///
    /// Only formats that are still shareable when
    /// [`D3D11_FEATURE_DATA_D3D11_OPTIONS::extended_resource_sharing`] == [`FALSE`] can be shared
    /// across APIs between Direct3D 11 and Direct3D 12.
    ///
    /// Resource formats added by [`D3D11_FEATURE_DATA_D3D11_OPTIONS::extended_resource_sharing`]
    /// == [`TRUE`] can't be shared across APIs.
    _1,

    /// Specifies the support available when
    /// [`D3D11_FEATURE_DATA_D3D11_OPTIONS4::extended_nv12_shared_texture_supported`] is [`TRUE`].
    ///
    /// Sharing across APIs between Direct3D 11 and Direct3D 12 is possible for the
    /// [`D3D11_FEATURE_DATA_D3D11_OPTIONS::extended_resource_sharing`] == [`TRUE`] format list.
    _2,

    /// Specifies that [`DXGI_FORMAT::R11G11B10Float`] supports NT handle sharing. Also see
    /// [`IDXGIResource1::create_shared_handle`].
    ///
    /// Sharing across APIs between Direct3D 11 and Direct3D 12 is possible for the
    /// [`DXGI_FORMAT::R11G11B10Float`] format.
    _3,
}
