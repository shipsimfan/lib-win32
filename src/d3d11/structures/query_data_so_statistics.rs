use crate::UINT64;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11DeviceContext;

/// Query information about the amount of data streamed out to the stream-output buffers in between
/// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_QUERY_DATA_SO_STATISTICS {
    /// Number of primitives (that is, points, lines, and triangles) written to the stream-output
    /// buffers.
    pub num_primitives_written: UINT64,

    /// Number of primitives that would have been written to the stream-output buffers if there had
    /// been enough space for them all.
    pub primitives_storage_needed: UINT64,
}

impl Default for D3D11_QUERY_DATA_SO_STATISTICS {
    fn default() -> Self {
        D3D11_QUERY_DATA_SO_STATISTICS {
            num_primitives_written: 0,
            primitives_storage_needed: 0,
        }
    }
}
