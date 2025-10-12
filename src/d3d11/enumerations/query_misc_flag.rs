// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11DeviceContext, D3D11_QUERY_DESC};

/// Flags that describe miscellaneous query behavior.
///
/// # Remarks
/// This flag is part of a query description (see [`D3D11_QUERY_DESC`]).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_QUERY_MISC_FLAG {
    /// Tell the hardware that if it is not yet sure if something is hidden or not to draw it
    /// anyway. This is only used with an occlusion predicate. Predication data cannot be returned
    /// to your application via [`ID3D11DeviceContext::get_data`] when using this flag.
    PredicateHint = 0x1,
}
