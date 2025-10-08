// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_FEATURE_DATA_D3D11_OPTIONS1;

/// Filtering options during texture sampling.
///
/// # Remarks
/// During texture sampling, one or more texels are read and combined (this is calling filtering)
/// to produce a single value. Point sampling reads a single texel while linear sampling reads two
/// texels (endpoints) and linearly interpolates a third value between the endpoints.
///
/// HLSL texture-sampling functions also support comparison filtering during texture sampling.
/// Comparison filtering compares each sampled texel against a comparison value. The boolean result
/// is blended the same way that normal texture filtering is blended.
///
/// You can use HLSL intrinsic texture-sampling functions that implement texture filtering only or
/// companion functions that use texture filtering with comparison filtering.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_FILTER {
    /// Use point sampling for minification, magnification, and mip-level sampling.
    MinMagMipPoint = 0,

    /// Use point sampling for minification and magnification; use linear interpolation for
    /// mip-level sampling.
    MinMagPointMipLinear = 0x1,

    /// Use point sampling for minification; use linear interpolation for magnification; use point
    /// sampling for mip-level sampling.
    MinPointMagLinearMipPoint = 0x4,

    /// Use point sampling for minification; use linear interpolation for magnification and
    /// mip-level sampling.
    MinPointMagMipLinear = 0x5,

    /// Use linear interpolation for minification; use point sampling for magnification and
    /// mip-level sampling.
    MinLinearMagMipPoint = 0x10,

    /// Use linear interpolation for minification; use point sampling for magnification; use linear
    /// interpolation for mip-level sampling.
    MinLinearMagPointMipLinear = 0x11,

    /// Use linear interpolation for minification and magnification; use point sampling for
    /// mip-level sampling.
    MinMagLinearMipPoint = 0x14,

    /// Use linear interpolation for minification, magnification, and mip-level sampling.
    MinMagMipLinear = 0x15,

    /// Use anisotropic interpolation for minification, magnification, and mip-level sampling.
    Anisotropic = 0x55,

    /// Use point sampling for minification, magnification, and mip-level sampling. Compare the
    /// result to the comparison value.
    ComparisonMinMagMipPoint = 0x80,

    /// Use point sampling for minification and magnification; use linear interpolation for
    /// mip-level sampling. Compare the result to the comparison value.
    ComparisonMinMagPointMipLinear = 0x81,

    /// Use point sampling for minification; use linear interpolation for magnification; use point
    /// sampling for mip-level sampling. Compare the result to the comparison value.
    ComparisonMinPointMagLinearMipPoint = 0x84,

    /// Use point sampling for minification; use linear interpolation for magnification and
    /// mip-level sampling. Compare the result to the comparison value.
    ComparisonMinPointMagMipLinear = 0x85,

    /// Use linear interpolation for minification; use point sampling for magnification and
    /// mip-level sampling. Compare the result to the comparison value.
    ComparisonMinLinearMagMipPoint = 0x90,

    /// Use linear interpolation for minification; use point sampling for magnification; use linear
    /// interpolation for mip-level sampling. Compare the result to the comparison value.
    ComparisonMinLinearMagPointMipLinear = 0x91,

    /// Use linear interpolation for minification and magnification; use point sampling for
    /// mip-level sampling. Compare the result to the comparison value.
    ComparisonMinMagLinearMipPoint = 0x94,

    /// Use linear interpolation for minification, magnification, and mip-level sampling. Compare
    /// the result to the comparison value.
    ComparisonMinMagMipLinear = 0x95,

    /// Use anisotropic interpolation for minification, magnification, and mip-level sampling.
    /// Compare the result to the comparison value.
    ComparisonAnisotropic = 0xd5,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinMagMipPoint`] and instead of filtering
    /// them return the minimum of the texels. Texels that are weighted 0 during filtering aren't
    /// counted towards the minimum. You can query support for this filter type from the
    /// `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MinimumMinMagMipPoint = 0x100,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinMagPointMipLinear`] and instead of
    /// filtering them return the minimum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the minimum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MinimumMinMagPointMipLinear = 0x101,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinPointMagLinearMipPoint`] and instead of
    /// filtering them return the minimum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the minimum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MinimumMinPointMagLinearMipPoint = 0x104,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinPointMagMipLinear`] and instead of
    /// filtering them return the minimum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the minimum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MinimumMinPointMagMipLinear = 0x105,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinLinearMagMipPoint`] and instead of
    /// filtering them return the minimum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the minimum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MinimumMinLinearMagMipPoint = 0x110,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinLinearMagPointMipLinear`] and instead of
    /// filtering them return the minimum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the minimum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MinimumMinLinearMagPointMipLinear = 0x111,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinMagLinearMipPoint`] and instead of
    /// filtering them return the minimum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the minimum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MinimumMinMagLinearMipPoint = 0x114,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinMagMipLinear`] and instead of filtering
    /// them return the minimum of the texels. Texels that are weighted 0 during filtering aren't
    /// counted towards the minimum. You can query support for this filter type from the
    /// `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MinimumMinMagMipLinear = 0x115,

    /// Fetch the same set of texels as [`D3D11_FILTER::Anisotropic`] and instead of filtering them
    /// return the minimum of the texels. Texels that are weighted 0 during filtering aren't
    /// counted towards the minimum. You can query support for this filter type from the
    /// `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MinimumAnisotropic = 0x155,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinMagMipPoint`] and instead of filtering
    /// them return the maximum of the texels. Texels that are weighted 0 during filtering aren't
    /// counted towards the maximum. You can query support for this filter type from the
    /// `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MaximumMinMagMipPoint = 0x180,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinMagPointMipLinear`] and instead of
    /// filtering them return the maximum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the maximum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MaximumMinMagPointMipLinear = 0x181,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinPointMagLinearMipPoint`] and instead of
    /// filtering them return the maximum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the maximum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MaximumMinPointMagLinearMipPoint = 0x184,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinPointMagMipLinear`] and instead of
    /// filtering them return the maximum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the maximum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MaximumMinPointMagMipLinear = 0x185,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinLinearMagMipPoint`] and instead of
    /// filtering them return the maximum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the maximum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MaximumMinLinearMagMipPoint = 0x190,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinLinearMagPointMipLinear`] and instead of
    /// filtering them return the maximum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the maximum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MaximumMinLinearMagPointMipLinear = 0x191,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinMagLinearMipPoint`] and instead of
    /// filtering them return the maximum of the texels. Texels that are weighted 0 during
    /// filtering aren't counted towards the maximum. You can query support for this filter type
    /// from the `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MaximumMinMagLinearMipPoint = 0x194,

    /// Fetch the same set of texels as [`D3D11_FILTER::MinMagMipLinear`] and instead of filtering
    /// them return the maximum of the texels. Texels that are weighted 0 during filtering aren't
    /// counted towards the maximum. You can query support for this filter type from the
    /// `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MaximumMinMagMipLinear = 0x195,

    /// Fetch the same set of texels as [`D3D11_FILTER::Anisotropic`] and instead of filtering them
    /// return the maximum of the texels. Texels that are weighted 0 during filtering aren't
    /// counted towards the maximum. You can query support for this filter type from the
    /// `min_max_filtering` member in the [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] structure.
    MaximumAnisotropic = 0x1d5,
}
