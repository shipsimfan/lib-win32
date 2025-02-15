use crate::UINT;

/// Describes multi-sampling parameters for a resource.
///
/// # Remarks
/// This structure is a member of the [`DXGI_SWAP_CHAIN_DESC1`] structure.
///
/// The default sampler mode, with no anti-aliasing, has a count of 1 and a quality level of 0.
///
/// If multi-sample antialiasing is being used, all bound render targets and depth buffers must
/// have the same sample counts and quality levels.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_SAMPLE_DESC {
    /// The number of multisamples per pixel.
    pub count: UINT,

    /// The image quality level. The higher the quality, the lower the performance. The valid range
    /// is between zero and one less than the level returned by
    /// [`ID3D10Device::check_multisample_quality_levels`] for Direct3D 10 or
    /// [`ID3D11Device::check_multisample_quality_levels`] for Direct3D 11.
    ///
    /// For Direct3D 10.1 and Direct3D 11, you can use two special quality level values. For more
    /// information about these quality level values, see Remarks.
    pub quality: UINT,
}

impl Default for DXGI_SAMPLE_DESC {
    fn default() -> Self {
        DXGI_SAMPLE_DESC {
            count: 1,
            quality: 0,
        }
    }
}
