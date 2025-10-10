use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_FEATURE;

/// Describes feature data GPU virtual address support, including maximum address bits per resource
/// and per process.
///
/// # Remarks
/// See [`D3D11_FEATURE`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    /// The maximum GPU virtual address bits per resource.
    pub max_gpu_virtual_address_bits_per_resource: UINT,

    /// The maximum GPU virtual address bits per process.
    pub max_gpu_virtual_address_bits_per_process: UINT,
}

impl Default for D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn default() -> Self {
        D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
            max_gpu_virtual_address_bits_per_resource: 0,
            max_gpu_virtual_address_bits_per_process: 0,
        }
    }
}
