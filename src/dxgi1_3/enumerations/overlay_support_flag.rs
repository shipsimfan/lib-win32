// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_3::IDXGIOutput3;

/// Specifies overlay support to check for in a call to [`IDXGIOutput3::check_overlay_support`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DXGI_OVERLAY_SUPPORT_FLAG {
    /// Direct overlay support.
    Direct = 0x1,

    /// Scaling overlay support.
    Scaling = 0x2,
}
