mod display_config_2d_region;
mod display_config_adapter_name;
mod display_config_device_info_header;
mod display_config_rational;
mod display_config_sdr_white_level;
mod display_config_set_target_persistence;
mod display_config_source_device_name;
mod display_config_support_virtual_resolution;
mod display_config_target_base_type;
mod display_config_target_device_name;
mod display_config_target_device_name_flags;
mod display_config_target_mode;
mod display_config_target_preferred_mode;
mod display_config_video_signal_info;
mod display_device_a;
mod display_device_w;
mod log_font_a;
mod log_font_w;

pub use display_config_2d_region::DISPLAYCONFIG_2DREGION;
pub use display_config_adapter_name::DISPLAYCONFIG_ADAPTER_NAME;
pub use display_config_device_info_header::DISPLAYCONFIG_DEVICE_INFO_HEADER;
pub use display_config_rational::DISPLAYCONFIG_RATIONAL;
pub use display_config_sdr_white_level::DISPLAYCONFIG_SDR_WHITE_LEVEL;
pub use display_config_set_target_persistence::{
    DISPLAYCONFIG_SET_TARGET_PERSISTENCE, DISPLAYCONFIG_SET_TARGET_PERSISTENCE_STRUCT,
    DISPLAYCONFIG_SET_TARGET_PERSISTENCE_UNION,
};
pub use display_config_source_device_name::DISPLAYCONFIG_SOURCE_DEVICE_NAME;
pub use display_config_support_virtual_resolution::{
    DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION, DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_STRUCT,
    DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_UNION,
};
pub use display_config_target_base_type::DISPLAYCONFIG_TARGET_BASE_TYPE;
pub use display_config_target_device_name::DISPLAYCONFIG_TARGET_DEVICE_NAME;
pub use display_config_target_device_name_flags::DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS;
pub use display_config_target_mode::DISPLAYCONFIG_TARGET_MODE;
pub use display_config_target_preferred_mode::DISPLAYCONFIG_TARGET_PREFERRED_MODE;
pub use display_config_video_signal_info::{
    DISPLAYCONFIG_VIDEO_SIGNAL_INFO, DISPLAYCONFIG_VIDEO_SIGNAL_INFO_STRUCT,
    DISPLAYCONFIG_VIDEO_SIGNAL_INFO_UNION,
};
pub use display_device_a::DISPLAY_DEVICEA;
pub use display_device_w::{DISPLAY_DEVICEW, DISPLAY_DEVICEW as DISPLAY_DEVICE};
pub use log_font_a::LOGFONTA;
pub use log_font_w::{LOGFONTW, LOGFONTW as LOGFONT};
