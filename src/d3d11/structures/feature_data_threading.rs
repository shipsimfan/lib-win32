use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::ID3D11Device, FALSE, TRUE};

/// Describes the multi-threading features that are supported by the current graphics driver.W
///
/// # Remarks
/// Use the [`D3D11_FEATURE_DATA_THREADING`] structure with the
/// [`ID3D11Device::check_feature_support`] method to determine multi-threading support.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_THREADING {
    /// [`TRUE`] means resources can be created concurrently on multiple threads while drawing;
    /// [`FALSE`] means that the presence of coarse synchronization will prevent concurrency.
    pub driver_concurrent_creates: BOOL,

    /// [`TRUE`] means command lists are supported by the current driver; [`FALSE`] means that the
    /// API will emulate deferred contexts and command lists with software.
    pub driver_command_lists: BOOL,
}

impl Default for D3D11_FEATURE_DATA_THREADING {
    fn default() -> Self {
        D3D11_FEATURE_DATA_THREADING {
            driver_concurrent_creates: 0,
            driver_command_lists: 0,
        }
    }
}
