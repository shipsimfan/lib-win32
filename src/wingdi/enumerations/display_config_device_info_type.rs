// rustdoc imports
#[allow(unused_imports)]
use crate::{
    DisplayConfigGetDeviceInfo, DisplayConfigSetDeviceInfo, DISPLAYCONFIG_ADAPTER_NAME,
    DISPLAYCONFIG_SDR_WHITE_LEVEL, DISPLAYCONFIG_SET_TARGET_PERSISTENCE,
    DISPLAYCONFIG_SOURCE_DEVICE_NAME, DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION,
    DISPLAYCONFIG_TARGET_BASE_TYPE, DISPLAYCONFIG_TARGET_DEVICE_NAME,
    DISPLAYCONFIG_TARGET_PREFERRED_MODE,
};

/// The [`DISPLAYCONFIG_DEVICE_INFO_TYPE`] enumeration specifies the type of display device info to
/// configure or obtain through the [`DisplayConfigSetDeviceInfo`] or
/// [`DisplayConfigGetDeviceInfo`] function.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DISPLAYCONFIG_DEVICE_INFO_TYPE {
    /// Specifies the source name of the display device. If the [`DisplayConfigGetDeviceInfo`]
    /// function is successful, [`DisplayConfigGetDeviceInfo`] returns the source name in the
    /// [`DISPLAYCONFIG_SOURCE_DEVICE_NAME`] structure.
    GetSourceName = 1,

    /// Specifies information about the monitor. If the [`DisplayConfigGetDeviceInfo`] function is
    /// successful, [`DisplayConfigGetDeviceInfo`] returns info about the monitor in the
    /// [`DISPLAYCONFIG_TARGET_DEVICE_NAME`] structure.
    GetTargetName = 2,

    /// Specifies information about the preferred mode of a monitor. If the
    /// [`DisplayConfigGetDeviceInfo`] function is successful, [`DisplayConfigGetDeviceInfo`]
    /// returns info about the preferred mode of a monitor in the
    /// [`DISPLAYCONFIG_TARGET_PREFERRED_MODE`] structure.
    GetTargetPreferredMode = 3,

    /// Specifies the graphics adapter name. If the [`DisplayConfigGetDeviceInfo`] function is
    /// successful, [`DisplayConfigGetDeviceInfo`] returns the adapter name in the
    /// [`DISPLAYCONFIG_ADAPTER_NAME`] structure.
    GetAdapterName = 4,

    /// Specifies how to set the monitor. If the [`DisplayConfigSetDeviceInfo`] function is
    /// successful, [`DisplayConfigSetDeviceInfo`] uses info in the
    /// [`DISPLAYCONFIG_SET_TARGET_PERSISTENCE`] structure to force the output in a boot-persistent
    /// manner.
    SetTargetPersistence = 5,

    /// Specifies how to set the base output technology for a given target ID. If the
    /// [`DisplayConfigGetDeviceInfo`] function is successful, [`DisplayConfigGetDeviceInfo`]
    /// returns base output technology info in the [`DISPLAYCONFIG_TARGET_BASE_TYPE`] structure.
    ///
    /// Supported by WDDM 1.3 and later user-mode display drivers running on Windows 8.1 and later.
    GetTargetBaseType = 6,

    /// Specifies the state of virtual mode support. If the [`DisplayConfigGetDeviceInfo`] function
    /// is successful, [`DisplayConfigGetDeviceInfo`] returns virtual mode support information in
    /// the [`DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION`] structure. Supported starting in Windows
    /// 10.
    GetSupportVirtualResolution = 7,

    /// Specifies how to set the state of virtual mode support. If the
    /// [`DisplayConfigSetDeviceInfo`] function is successful, [`DisplayConfigSetDeviceInfo`] uses
    /// info in the [`DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION`] structure to change the state of
    /// virtual mode support. Supported starting in Windows 10.
    SetSupportVirtualResolution = 8,

    #[allow(missing_docs)]
    GetAdvancedColorInfo = 9,

    #[allow(missing_docs)]
    SetAdvancedColorState = 10,

    /// Specifies the current SDR white level for an HDR monitor. If the
    /// [`DisplayConfigGetDeviceInfo`] function is successful, [`DisplayConfigGetDeviceInfo`]
    /// return SDR white level info in the [`DISPLAYCONFIG_SDR_WHITE_LEVEL`] structure.
    ///
    /// Supported starting in Windows 10 Fall Creators Update (Version 1709).
    GetSdrWhiteLevel = 11,

    #[allow(missing_docs)]
    GetMonitorSpecialization,

    #[allow(missing_docs)]
    SetMonitorSpecialization,

    #[allow(missing_docs)]
    SetReserved1,

    #[allow(missing_docs)]
    GetAdvancedColorInfo2,

    #[allow(missing_docs)]
    SetHdrState,

    #[allow(missing_docs)]
    SetWcgState,

    /// Forces this enumeration to compile to 32 bits in size. Without this value, some compilers
    /// would allow this enumeration to compile to a size other than 32 bits. You should not use
    /// this value.
    ForceUint32 = 0xFFFFFFFF,
}
