use crate::{d3d11::D3D11_COUNTER, UINT, UINT8};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Device;

/// Information about the video card's performance counter capabilities.
///
/// # Remarks
/// This structure is returned by [`ID3D11Device::check_counter_info`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_COUNTER_INFO {
    /// Largest device-dependent counter ID that the device supports. If none are supported, this
    /// value will be 0. Otherwise it will be greater than or equal to
    /// [`D3D11_COUNTER::DeviceDependent0`]. See [`D3D11_COUNTER`].
    pub last_device_dependent_counter: D3D11_COUNTER,

    /// Number of counters that can be simultaneously supported.
    pub num_simultaneous_counters: UINT,

    /// Number of detectable parallel units that the counter is able to discern. Values are 1 ~ 4.
    /// Use `num_detectable_parallel_units` to interpret the values of the `VERTEX_PROCESSING`,
    /// `GEOMETRY_PROCESSING`, `PIXEL_PROCESSING`, and `OTHER_GPU_PROCESSING` counters.
    pub num_detectable_parallel_units: UINT8,
}

impl Default for D3D11_COUNTER_INFO {
    fn default() -> Self {
        D3D11_COUNTER_INFO {
            last_device_dependent_counter: D3D11_COUNTER::DeviceDependent0,
            num_simultaneous_counters: 0,
            num_detectable_parallel_units: 0,
        }
    }
}
