use crate::{DISPLAYCONFIG_PATH_SOURCE_INFO, DISPLAYCONFIG_PATH_TARGET_INFO, UINT32};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    QueryDisplayConfig, DISPLAYCONFIG_PATH_ACTIVE, DISPLAYCONFIG_PATH_BOOST_REFRESH_RATE,
    DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE, DISPLAYCONFIG_TARGET_MODE,
};

/// The [`DISPLAYCONFIG_PATH_INFO`] structure is used to describe a single path from a target to a
/// source.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DISPLAYCONFIG_PATH_INFO {
    /// A [`DISPLAYCONFIG_PATH_SOURCE_INFO`] structure that contains the source information for the
    /// path.
    pub source_info: DISPLAYCONFIG_PATH_SOURCE_INFO,

    /// A [`DISPLAYCONFIG_PATH_TARGET_INFO`] structure that contains the target information for the
    /// path.
    pub target_info: DISPLAYCONFIG_PATH_TARGET_INFO,

    /// A bitwise OR of flag values that indicates the state of the path. The following values are
    /// supported:
    ///  * [`DISPLAYCONFIG_PATH_ACTIVE`] - Set by [`QueryDisplayConfig`] to indicate that the path
    ///                                    is active and part of the desktop. If this flag value is
    ///                                    set, [`SetDisplayConfig`] attempts to enable this path.
    ///  * [`DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE`] - Set by [`QueryDisplayConfig`] to indicate
    ///                                                  that the path supports virtual modes. This
    ///                                                  flag is for reporting only and cannot be
    ///                                                  modified. Supported starting in Windows
    ///                                                  10.
    ///  * [`DISPLAYCONFIG_PATH_BOOST_REFRESH_RATE`] - Set by [`QueryDisplayConfig`] to indicate
    ///                                                that the path is configured to automatically
    ///                                                boost the refresh rate between the virtual
    ///                                                refresh rate and the physical refresh rate
    ///                                                (this is known as "Dynamic refresh rate").
    ///                                                This value can be set or removed for a path
    ///                                                by [`SetDisplayConfig`]. The virtual refresh
    ///                                                rate is set by
    ///                                            [`DISPLAYCONFIG_PATH_TARGET_INFO::refresh_rate`]
    ///                                                and the physical refresh rate is selected by
    ///                                    [`DISPLAYCONFIG_TARGET_MODE::target_video_signal_info`].
    ///                                                Supported starting in Windows 11.
    pub flags: UINT32,
}

impl Default for DISPLAYCONFIG_PATH_INFO {
    fn default() -> Self {
        DISPLAYCONFIG_PATH_INFO {
            source_info: DISPLAYCONFIG_PATH_SOURCE_INFO::default(),
            target_info: DISPLAYCONFIG_PATH_TARGET_INFO::default(),
            flags: 0,
        }
    }
}
