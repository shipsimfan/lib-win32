// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_5::IDXGIFactory5;

/// Specifies a range of hardware features, to be used when checking for feature support.
///
/// # Remarks
/// This enum is used by the [`IDXGIFactory5::check_feature_support`] method.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_FEATURE {
    /// The display supports tearing, a requirement of variable refresh rate displays.
    AllowTearing = 0,
}
