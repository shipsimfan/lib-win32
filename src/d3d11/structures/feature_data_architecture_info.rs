use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{FALSE, TRUE};

/// Describes information about Direct3D 11.1 adapter architecture.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    /// Specifies whether a rendering device batches rendering commands and performs multipass
    /// rendering into tiles or bins over a render area. Certain API usage patterns that are fine
    /// for Tile Based Defferred Renderers (TBDRs) can perform worse on non-TBDRs and vice versa.
    /// Applications that are careful about rendering can be friendly to both TBDR and non-TBDR
    /// architectures. [`TRUE`] if the rendering device batches rendering commands and [`FALSE`]
    /// otherwise.
    pub tile_based_deferred_renderer: BOOL,
}

impl Default for D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    fn default() -> Self {
        D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
            tile_based_deferred_renderer: 0,
        }
    }
}
