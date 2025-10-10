// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_FEATURE_DATA_D3D11_OPTIONS2;

/// Specifies texture layout options.
///
/// # Remarks
/// This enumeration controls the swizzle pattern of default textures and enable map support on
/// default textures. Callers must query [`D3D11_FEATURE_DATA_D3D11_OPTIONS2`] to ensure that each
/// option is supported.
///
/// The standard swizzle formats applies within each page-sized chunk, and pages are laid out in
/// linear order with respect to one another. A 16-bit interleave pattern defines the conversion
/// from pre-swizzled intra-page location to the post-swizzled location.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum D3D11_TEXTURE_LAYOUT {
    /// The texture layout is undefined, and is selected by the driver.
    Undefined = 0,

    /// Data for the texture is stored in row major (sometimes called pitch-linear) order.
    RowMajor = 1,

    /// A default texture uses the standardized swizzle pattern.
    _64KStandardSwizzle = 2,
}
