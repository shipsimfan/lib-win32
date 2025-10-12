use crate::{d3d11::D3D11_COUNTER, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Counter, ID3D11Device};

/// Describes a counter.
///
/// # Remarks
/// This structure is used by [`ID3D11Counter::get_desc`], [`ID3D11Device::check_counter`] and
/// [`ID3D11Device::create_counter`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_COUNTER_DESC {
    /// Type of counter (see [`D3D11_COUNTER`]).
    pub counter: D3D11_COUNTER,

    /// Reserved.
    pub misc_flags: UINT,
}

impl Default for D3D11_COUNTER_DESC {
    fn default() -> Self {
        D3D11_COUNTER_DESC {
            counter: D3D11_COUNTER::DeviceDependent0,
            misc_flags: 0,
        }
    }
}
