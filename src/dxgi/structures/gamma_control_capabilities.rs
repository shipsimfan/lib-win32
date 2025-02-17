use crate::{BOOL, UINT};
use std::ffi::c_float;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::IDXGIOutput;

/// Controls the gamma capabilities of an adapter.
///
/// To get a list of the capabilities for controlling gamma correction, call
/// [`IDXGIOutput::get_gamma_control_capabilities`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    /// True if scaling and offset operations are supported during gamma correction; otherwise,
    /// false.
    pub scale_and_offset_supported: BOOL,

    /// A value describing the maximum range of the control-point positions.
    pub max_converted_value: c_float,

    /// A value describing the minimum range of the control-point positions.
    pub min_converted_value: c_float,

    /// A value describing the number of control points in the array.
    pub num_gamma_control_points: UINT,

    /// An array of values describing control points; the maximum length of control points is 1025.
    pub control_point_positions: [c_float; 1025],
}

impl Default for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn default() -> Self {
        DXGI_GAMMA_CONTROL_CAPABILITIES {
            scale_and_offset_supported: 0,
            max_converted_value: 0.0,
            min_converted_value: 0.0,
            num_gamma_control_points: 0,
            control_point_positions: [0.0; 1025],
        }
    }
}
