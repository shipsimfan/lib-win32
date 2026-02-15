use crate::{
    BOOL, DISPLAYCONFIG_RATIONAL, DISPLAYCONFIG_ROTATION, DISPLAYCONFIG_SCALING,
    DISPLAYCONFIG_SCANLINE_ORDERING, DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY, LUID, UINT32,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    QueryDisplayConfig, DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID, DISPLAYCONFIG_PATH_INFO,
    DISPLAYCONFIG_PATH_MODE_IDX_INVALID, DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE,
    DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT, DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH,
    DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM, DISPLAYCONFIG_TARGET_FORCIBLE,
    DISPLAYCONFIG_TARGET_IN_USE, DISPLAYCONFIG_TARGET_IS_HMD, DISPLAYCONFIG_TARGET_MODE,
    DISPLAYCONFIG_VIDEO_SIGNAL_INFO, FALSE, QDC_INCLUDE_HMD, TRUE,
};

/// The [`DISPLAYCONFIG_PATH_TARGET_INFO`] structure contains target information for a single path.
///
/// # Remarks
/// A [`DISPLAYCONFIG_PATH_TARGET_INFO`] structure is specified in the `target_info` member of a
/// [`DISPLAYCONFIG_PATH_INFO`] structure.
///
/// A target corresponds to the number of possible video outputs on a display adapter. This number,
/// however, does not equate to the number of physical connectors on the display adapter. Each
/// connector exposes a number of targets that includes backward compatibility with older connector
/// technology. For example, a DVI connector exposes a DVI target, as well as a VGA target. A
/// DisplayPort connector, which was introduced in 2006, exposes DisplayPort, HDMI, DVI, legacy TV,
/// and VGA targets.
///
/// The `status_flags` member is set when you call the [`QueryDisplayConfig`] function.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO {
    /// The identifier of the adapter that the path is on.
    pub adapter_id: LUID,

    /// The target identifier on the specified adapter that this path relates to.
    pub id: UINT32,

    /// A valid index into the mode information table that contains the target mode information for
    /// this path only when [`DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE`] is not set. If target mode
    /// information is not available, the value of `mode_info_idx` is
    /// [`DISPLAYCONFIG_PATH_MODE_IDX_INVALID`].
    ///
    /// # Bits 0 - 15
    /// A valid index into the mode array of the [`DISPLAYCONFIG_DESKTOP_IMAGE_INFO`] entry that
    /// contains the desktop mode information for this path only when
    /// [`DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE`] is set. If there is no entry for this in the
    /// mode array, the value of desktopModeInfoIdx is
    /// [`DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID`]. Supported starting in Windows 10.
    ///
    /// # Bits 16 - 31
    /// A valid index into the mode array of the [`DISPLAYCONFIG_TARGET_MODE`] entry that contains
    /// the target mode information for this path only when
    /// [`DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE`] is set. If there is no entry for this in the
    /// mode array, the value of `target_mode_info_idx` is
    /// [`DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID`]. Supported starting in Windows 10.
    pub mode_info_idx: UINT32,

    /// The target's connector type. For a list of possible values, see the
    /// [`DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY`] enumerated type.
    pub output_technology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,

    /// A value that specifies the rotation of the target. For a list of possible values, see the
    /// [`DISPLAYCONFIG_ROTATION`] enumerated type.
    pub rotation: DISPLAYCONFIG_ROTATION,

    /// A value that specifies how the source image is scaled to the target. For a list of possible
    /// values, see the [`DISPLAYCONFIG_SCALING`] enumerated type. For more information about
    /// scaling, see Scaling the Desktop Image.
    pub scaling: DISPLAYCONFIG_SCALING,

    /// A [`DISPLAYCONFIG_RATIONAL`] structure that specifies the refresh rate of the target. If
    /// the caller specifies target mode information, the operating system will instead use the
    /// refresh rate that is stored in the `v_sync_freq` member of the
    /// [`DISPLAYCONFIG_VIDEO_SIGNAL_INFO`] structure. In this case, the caller specifies this
    /// value in the `target_video_signal_info` member of the [`DISPLAYCONFIG_TARGET_MODE`]
    /// structure. A refresh rate with both the numerator and denominator set to zero indicates
    /// that the caller does not specify a refresh rate and the operating system should use the
    /// most optimal refresh rate available. For this case, in a call to the [`SetDisplayConfig`]
    /// function, the caller must set the `scanline_ordering` member to the
    /// [`DISPLAYCONFIG_SCANLINE_ORDERING::Unspecified`] value; otherwise, [`SetDisplayConfig`]
    /// fails.
    pub refresh_rate: DISPLAYCONFIG_RATIONAL,

    /// A value that specifies the scan-line ordering of the output on the target. For a list of
    /// possible values, see the [`DISPLAYCONFIG_SCANLINE_ORDERING`] enumerated type. If the caller
    /// specifies target mode information, the operating system will instead use the scan-line
    /// ordering that is stored in the `scanline_ordering` member of the
    /// [`DISPLAYCONFIG_VIDEO_SIGNAL_INFO`] structure. In this case, the caller specifies this
    /// value in the `target_video_signal_info` member of the [`DISPLAYCONFIG_TARGET_MODE`]
    /// structure.
    pub scanline_ordering: DISPLAYCONFIG_SCANLINE_ORDERING,

    /// A Boolean value that specifies whether the target is available. [`TRUE`] indicates that the
    /// target is available.
    ///
    /// Because the asynchronous nature of display topology changes when a monitor is removed, a
    /// path might still be marked as active even though the monitor has been removed. In such a
    /// case, `target_available` could be [`FALSE`] for an active path. This is typically a
    /// transient situation that will change after the operating system takes action on the monitor
    /// removal.
    pub target_available: BOOL,

    /// A bitwise OR of flag values that indicates the status of the target. The following values
    /// are supported:
    ///  * [`DISPLAYCONFIG_TARGET_IN_USE`] - Target is in use on an active path.
    ///  * [`DISPLAYCONFIG_TARGET_FORCIBLE`] - The output can be forced on this target even if a
    ///                                        monitor is not detected.
    ///  * [`DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT`] - Output is currently being forced in
    ///                                                        a boot-persistent manner.
    ///  * [`DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH`] - Output is currently being forced in
    ///                                                        a path-persistent manner.
    ///  * [`DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM`] - Output is currently being forced
    ///                                                          in a nonpersistent manner.
    ///  * [`DISPLAYCONFIG_TARGET_IS_HMD`] - The output is a head-mounted display (HMD). Such a
    ///                                      path is only returned from QueryDisplayConfig using
    ///                                      the [`QDC_INCLUDE_HMD`] flag. Supported starting in
    ///                                      the Windows 10 Creators Update (Version 1703).
    pub status_flags: UINT32,
}

impl Default for DISPLAYCONFIG_PATH_TARGET_INFO {
    fn default() -> Self {
        DISPLAYCONFIG_PATH_TARGET_INFO {
            adapter_id: LUID::default(),
            id: 0,
            mode_info_idx: 0,
            output_technology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::Other,
            rotation: DISPLAYCONFIG_ROTATION::Identity,
            scaling: DISPLAYCONFIG_SCALING::Identity,
            refresh_rate: DISPLAYCONFIG_RATIONAL::default(),
            scanline_ordering: DISPLAYCONFIG_SCANLINE_ORDERING::Unspecified,
            target_available: 0,
            status_flags: 0,
        }
    }
}
