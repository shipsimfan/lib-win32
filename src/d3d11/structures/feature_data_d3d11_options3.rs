use crate::BOOL;

/// Describes Direct3D 11.3 feature options in the current graphics driver.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    /// Whether to use the VP and RT array index from any shader feeding the rasterizer.
    pub vp_and_rt_array_index_from_any_shader_feeding_rasterizer: BOOL,
}

impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    fn default() -> Self {
        D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
            vp_and_rt_array_index_from_any_shader_feeding_rasterizer: 0,
        }
    }
}
