use crate::{UINT, UINT16};

/// Describes the metadata for HDR10, used when video is compressed using High Efficiency Video
/// Coding (HEVC). This is used to describe the capabilities of the display used to master the
/// content and the luminance values of the content.
///
/// # Remarks
/// This structure represents the definition of HDR10 metadata used with HEVC, not HDR10 metadata
/// for ST.2086. These are closely related but defined differently.
///
/// This structure is used in conjunction with the [`IDXGISwapChain4::set_hdr_metadata`] method.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_HDR_METADATA_HDR10 {
    /// The chromaticity coordinates of the red value in the CIE1931 color space. Index 0 contains
    /// the X coordinate and index 1 contains the Y coordinate. The values are normalized to
    /// 50,000.
    pub red_primary: [UINT16; 2],

    /// The chromaticity coordinates of the green value in the CIE1931 color space. Index 0
    /// contains the X coordinate and index 1 contains the Y coordinate. The values are normalized
    /// to 50,000.
    pub green_primary: [UINT16; 2],

    /// The chromaticity coordinates of the blue value in the CIE1931 color space. Index 0 contains
    /// the X coordinate and index 1 contains the Y coordinate. The values are normalized to
    /// 50,000.
    pub blue_primary: [UINT16; 2],

    /// The chromaticity coordinates of the white point in the CIE1931 color space. Index 0
    /// contains the X coordinate and index 1 contains the Y coordinate. The values are normalized
    /// to 50,000.
    pub white_point: [UINT16; 2],

    /// The maximum number of nits of the display used to master the content. Values are in whole
    /// nits.
    pub max_mastering_luminance: UINT,

    /// The minimum number of nits of the display used to master the content. Values are 1/10000th
    /// of a nit (0.0001 nit).
    pub min_mastering_luminance: UINT,

    /// The maximum content light level (MaxCLL). This is the nit value corresponding to the
    /// brightest pixel used anywhere in the content.
    pub max_content_light_level: UINT16,

    /// The maximum frame average light level (MaxFALL). This is the nit value corresponding to the
    /// average luminance of the frame which has the brightest average luminance anywhere in the
    /// content.
    pub max_frame_average_light_level: UINT16,
}

impl Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        DXGI_HDR_METADATA_HDR10 {
            red_primary: [0; 2],
            green_primary: [0; 2],
            blue_primary: [0; 2],
            white_point: [0; 2],
            max_mastering_luminance: 0,
            min_mastering_luminance: 0,
            max_content_light_level: 0,
            max_frame_average_light_level: 0,
        }
    }
}
