use crate::{BOOL, UINT64};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{ID3D11DeviceContext, D3D11_QUERY},
    FALSE, TRUE,
};

/// Query information about the reliability of a timestamp query.
///
/// # Remarks
/// For a list of query types see [`D3D11_QUERY`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    /// How frequently the GPU counter increments in Hz.
    pub frequency: UINT64,

    /// If this is [`TRUE`], something occurred in between the query's
    /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`] calls that caused the
    /// timestamp counter to become discontinuous or disjoint, such as unplugging the AC cord on a
    /// laptop, overheating, or throttling up/down due to laptop savings events. The timestamp
    /// returned by [`ID3D11DeviceContext::get_data`] for a timestamp query is only reliable if
    /// `disjoint` is [`FALSE`].
    pub disjoint: BOOL,
}

impl Default for D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn default() -> Self {
        D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
            frequency: 0,
            disjoint: 0,
        }
    }
}
