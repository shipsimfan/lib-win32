// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_4::{IDXGIOutput4, IDXGISwapChain3};

/// Specifies color space types.
///
/// # Remarks
/// This enum is used within DXGI in the [`IDXGISwapChain3::check_color_space_support`],
/// [`IDXGISwapChain3::set_color_space1`] and [`IDXGIOutput4::check_overlay_color_space_support`]
/// methods. It is also referenced in D3D11 video methods such as
/// [`ID3D11VideoContext1::video_processor_set_output_color_space1`], and D2D methods such as
/// [`ID2D1DeviceContext2::create_image_source_from_dxgi`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_COLOR_SPACE_TYPE {
    /// This is the standard definition for sRGB.
    RgbFullG22NoneP709 = 0,

    /// This is the standard definition for scRGB, and is usually used with 16 bit integer, 16 bit
    /// floating point, or 32 bit floating point color channels.
    RgbFullG10NoneP709 = 1,

    /// This is the standard definition for ITU-R Recommendation BT.709. Note that due to the
    /// inclusion of a linear segment, the transfer curve looks similar to a pure exponential gamma
    /// of 1.9.
    ///
    /// This is usually used with 8 or 10 bit color channels.
    RgbStudioG22NoneP709 = 2,

    /// This is usually used with 10 or 12 bit color channels.
    RgbStudioG22NoneP2020 = 3,

    /// Reserved.
    Reserved = 4,

    /// This definition is commonly used for JPG, and is usually used with 8, 10, or 12 bit color
    /// channels.
    YcbcrFullG22NoneP709X601 = 5,

    /// This definition is commonly used for MPEG2, and is usually used with 8, 10, or 12 bit color
    /// channels.
    YcbcrStudioG22LeftP601 = 6,

    /// This is sometimes used for H.264 camera capture, and is usually used with 8, 10, or 12 bit
    /// color channels.
    YcbcrFullG22LeftP601 = 7,

    /// This definition is commonly used for H.264 and HEVC, and is usually used with 8, 10, or 12
    /// bit color channels.
    YcbcrStudioG22LeftP709 = 8,

    /// This is sometimes used for H.264 camera capture, and is usually used with 8, 10, or 12 bit
    /// color channels.
    YcbcrFullG22LeftP709 = 9,

    /// This definition may be used by HEVC, and is usually used with 10 or 12 bit color channels.
    YcbcrStudioG22LeftP2020 = 10,

    /// This is usually used with 10 or 12 bit color channels.
    YcbcrFullG22LeftP2020 = 11,

    /// This is usually used with 10 or 12 bit color channels.
    RgbFullG2084NoneP2020 = 12,

    /// This is usually used with 10 or 12 bit color channels.
    YcbcrStudioG2084LeftP2020 = 13,

    /// This is usually used with 10 or 12 bit color channels.
    RgbStudioG2084NoneP2020 = 14,

    /// This is usually used with 10 or 12 bit color channels.
    YcbcrStudioG22TopleftP2020 = 15,

    /// This is usually used with 10 or 12 bit color channels.
    YcbcrStudioG2084TopleftP2020 = 16,

    /// This is usually used with 10 or 12 bit color channels.
    RgbFullG22NoneP2020 = 17,

    /// This is usually used with 10 or 12 bit color channels.
    YcbcrStudioGhlgTopleftP2020 = 18,

    /// This is usually used with 10 or 12 bit color channels.
    YcbcrFullGhlgTopleftP2020 = 19,

    /// This is usually used with 8, 10, or 12 bit color channels.
    RgbStudioG24NoneP709 = 20,

    /// This is usually used with 10 or 12 bit color channels.
    RgbStudioG24NoneP2020 = 21,

    /// This is usually used with 8, 10, or 12 bit color channels.
    YcbcrStudioG24LeftP709 = 22,

    /// This is usually used with 10 or 12 bit color channels.
    YcbcrStudioG24LeftP2020 = 23,

    /// This is usually used with 10 or 12 bit color channels.
    YcbcrStudioG24TopleftP2020 = 24,

    /// A custom color definition is used.
    Custom = 0xFfffffff,
}
