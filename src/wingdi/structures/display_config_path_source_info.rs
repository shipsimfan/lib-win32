use crate::{LUID, UINT32};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID, DISPLAYCONFIG_PATH_INFO,
    DISPLAYCONFIG_PATH_MODE_IDX_INVALID, DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID,
    DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE, DISPLAYCONFIG_SOURCE_IN_USE,
    DISPLAYCONFIG_SOURCE_MODE,
};

/// The [`DISPLAYCONFIG_PATH_SOURCE_INFO`] structure contains source information for a single path.
///
/// # Remarks
/// A [`DISPLAYCONFIG_PATH_SOURCE_INFO`] structure is specified in the `source_info` member of a
/// [`DISPLAYCONFIG_PATH_INFO`] structure.
///
/// A source corresponds to a surface on which the display adapter can render pixels. Each display
/// adapter is capable of rendering to x number of sources. What this means is how many desktops
/// can be rendered for extend mode. This is typically 2. For example, source 0 might be rendering
/// pixels from 0,0 to 1024,768, and source 1 might be rendering pixels from 1025,0 to 2048, 768.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO {
    /// The identifier of the adapter that this source information relates to.
    pub adapter_id: LUID,

    /// The source identifier on the specified adapter that this path relates to.
    pub id: UINT32,

    /// A valid index into the mode information table that contains the source mode information for
    /// this path only when [`DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE`] is not set. If source mode
    /// information is not available, the value of modeInfoIdx is
    /// [`DISPLAYCONFIG_PATH_MODE_IDX_INVALID`].
    ///
    /// # Bits 0 - 15
    /// A valid identifier used to show which clone group the path is a member of only when
    /// [`DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE`] is set. If this value is invalid, then it must
    /// be set to [`DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID`].
    ///
    /// `clone_group_id` is only used when the source mode index is not specified. Two such
    /// scenarios are when the source mode info must be invalid because [`SDC_TOPOLOGY_SUPPLIED`]
    /// is used, and when [`SDC_USE_SUPPLIED_DISPLAY_CONFIG`] is used with paths that do not have
    /// source mode info. The cloneGroupId will be used to indicate which paths are in a clone
    /// group, all the paths with the same cloneGroupId value are considered in the same clone
    /// group. There is no requirement that the clone group id’s have to be zero based or
    /// contiguous. Supported starting in Windows 10.
    ///
    /// # Bits 16 - 31
    /// A valid index into the mode array of the [`DISPLAYCONFIG_SOURCE_MODE`] entry that contains
    /// the source mode information for this path only when
    /// [`DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE`] is set. If there is no entry for this in the
    /// mode array, the value of sourceModeInfoIdx is
    /// [`DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID`]. Supported starting in Windows 10.
    pub mode_info_idx: UINT32,

    /// A bitwise OR of flag values that indicates the status of the source. The following values
    /// are supported:
    ///  * [`DISPLAYCONFIG_SOURCE_IN_USE`] - This source is in use by at least one active path.
    pub status_flags: UINT32,
}

impl Default for DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn default() -> Self {
        DISPLAYCONFIG_PATH_SOURCE_INFO {
            adapter_id: LUID::default(),
            id: 0,
            mode_info_idx: 0,
            status_flags: 0,
        }
    }
}
