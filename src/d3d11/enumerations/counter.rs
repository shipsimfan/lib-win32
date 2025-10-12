// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{D3D11_COUNTER_DESC, D3D11_COUNTER_INFO};

/// Options for performance counters.
///
/// # Remarks
/// Independent hardware vendors may define their own set of performance counters for their
/// devices, by giving the enumeration value a number that is greater than the value for
/// [`D3D11_COUNTER::DeviceDependent0`].
///
/// This enumeration is used by [`D3D11_COUNTER_DESC`] and [`D3D11_COUNTER_INFO`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_COUNTER {
    /// Define a performance counter that is dependent on the hardware device.
    DeviceDependent0 = 0x40000000,
}
