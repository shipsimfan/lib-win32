use crate::{
    DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_TOPOLOGY_ID, LONG, UINT32,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, DISPLAYCONFIG_2DREGION,
    DISPLAYCONFIG_PATH_SOURCE_INFO, DISPLAYCONFIG_PATH_TARGET_INFO,
    DISPLAYCONFIG_VIDEO_SIGNAL_INFO, ERROR_ACCESS_DENIED, ERROR_GEN_FAILURE,
    ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_PARAMETER, ERROR_NOT_SUPPORTED, ERROR_SUCCESS,
    QDC_ALL_PATHS, QDC_DATABASE_CURRENT, QDC_ONLY_ACTIVE_PATHS,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// The [`QueryDisplayConfig`] function retrieves information about all possible display paths
    /// for all display devices, or views, in the current setting.
    ///
    /// # Parameters
    ///  * `flags` - The type of information to retrieve. The value for the `flags` parameter must
    ///              use one of the following values:
    ///    * [`QDC_ALL_PATHS`] - Returns all the possible path combinations of sources to targets.
    ///    * [`QDC_ONLY_ACTIVE_PATHS`] - Returns currently active paths only.
    ///    * [`QDC_DATABASE_CURRENT`] -  Returns active paths as defined in the CCD database for
    ///                                  the currently connected displays.
    ///  * `num_path_array_elements` - Pointer to a variable that contains the number of elements
    ///                                in `path_array`. This parameter cannot be [`null_mut`]. If
    ///                                [`QueryDisplayConfig`] returns [`ERROR_SUCCESS`],
    ///                                `num_path_array_elements` is updated with the number of
    ///                                valid entries in `path_array`.
    ///  * `path_array` - Pointer to a variable that contains an array of
    ///                   [`DISPLAYCONFIG_PATH_INFO`] elements. Each element in `path_array`
    ///                   describes a single path from a source to a target. The source and target
    ///                   mode information indexes are only valid in combination with the
    ///                   `mode_info_array` tables that are returned for the API at the same time.
    ///                   This parameter cannot be [`null_mut`]. The `path_array` is always
    ///                   returned in path priority order.
    ///  * `num_mode_info_array_elements` - Pointer to a variable that specifies the number in
    ///                                     element of the mode information table. This parameter
    ///                                     cannot be [`null_mut`]. If [`QueryDisplayConfig`]
    ///                                     returns [`ERROR_SUCCESS`],
    ///                                     `num_mode_info_array_elements` is updated with the
    ///                                     number of valid entries in `mode_info_array`.
    ///  * `mode_info_array` - Pointer to a variable that contains an array of
    ///                        [`DISPLAYCONFIG_MODE_INFO`] elements. This parameter cannot be
    ///                        [`null_mut`].
    ///  * `current_topology_id` - Pointer to a variable that receives the identifier of the
    ///                            currently active topology in the CCD database. For a list of
    ///                            possible values, see the [`DISPLAYCONFIG_TOPOLOGY_ID`]
    ///                            enumerated type. The `current_topology_id` parameter is only set
    ///                            when the `flags` parameter value is [`QDC_DATABASE_CURRENT`]. If
    ///                            the `flags` parameter value is set to [`QDC_DATABASE_CURRENT`],
    ///                            the `current_topology_id` parameter must not be [`null_mut`]. If
    ///                            the `flags` parameter value is not set to
    ///                            [`QDC_DATABASE_CURRENT`], the `current_topology_id` parameter
    ///                            value must be [`null_mut`].
    ///
    /// # Return Value
    /// The function returns one of the following return codes:
    ///  * [`ERROR_SUCCESS`] - The function succeeded.
    ///  * [`ERROR_INVALID_PARAMETER`] - The combination of parameters and flags that are specified
    ///                                  is invalid.
    ///  * [`ERROR_NOT_SUPPORTED`] - The system is not running a graphics driver that was written
    ///                              according to the Windows Display Driver Model (WDDM). The
    ///                              function is only supported on a system with a WDDM driver
    ///                              running.
    ///  * [`ERROR_ACCESS_DENIED`] - The caller does not have access to the console session. This
    ///                              error occurs if the calling process does not have access to
    ///                              the current desktop or is running on a remote session.
    ///  * [`ERROR_GEN_FAILURE`] - An unspecified error occurred.
    ///  * [`ERROR_INSUFFICIENT_BUFFER`] - The supplied path and mode buffer are too small.
    ///
    /// # Remarks
    /// As the [`GetDisplayConfigBufferSizes`] function can only determine the required array size
    /// at a particular moment in time, it is possible that between calls to
    /// [`GetDisplayConfigBufferSizes`] and [`QueryDisplayConfig`] the system configuration will
    /// change and the provided array sizes will no longer be sufficient to store the new path
    /// data. In this situation, [`QueryDisplayConfig`] fails with [`ERROR_INSUFFICIENT_BUFFER`],
    /// and the caller should call [`GetDisplayConfigBufferSizes`] again to get the new array
    /// sizes. The caller should then allocate the correct amount of memory.
    ///
    /// [`QueryDisplayConfig`] returns paths in the path array that the `path_array` parameter
    /// specifies and the source and target modes in the mode array that the `mode_info_array`
    /// parameter specifies. [`QueryDisplayConfig`] always returns paths in path priority order. If
    /// [`QDC_ALL_PATHS`] is set in the `flags` parameter, [`QueryDisplayConfig`] returns all the
    /// inactive paths after the active paths.
    ///
    /// Full path, source mode, and target mode information is available for all active paths. The
    /// `mode_info_idx` members in the [`DISPLAYCONFIG_PATH_SOURCE_INFO`] and
    /// [`DISPLAYCONFIG_PATH_TARGET_INFO`] structures for the source and target are set up for
    /// these active paths. For inactive paths, returned source and target mode information is not
    /// available; therefore, the target information in the path structure is set to default
    /// values, and the source and target mode indexes are marked as invalid. For database queries,
    /// if the current connect monitors have an entry, [`QueryDisplayConfig`] returns full path,
    /// source mode, and target mode information (same as for active paths). However, if the
    /// database does not have a entry, [`QueryDisplayConfig`] returns just the path information
    /// with the default target details (same as for inactive paths).
    ///
    /// The caller can use [`DisplayConfigGetDeviceInfo`] to obtain additional information about
    /// the source or target device, for example, the monitor names and monitor preferred mode and
    /// source device name.
    ///
    /// If a target is currently being force projected, the `status_flags` member of the
    /// [`DISPLAYCONFIG_PATH_TARGET_INFO`] structure has one of the
    /// `DISPLAYCONFIG_TARGET_FORCED_XXX` flags set.
    ///
    /// If the [`QDC_DATABASE_CURRENT`] flag is set in the `flags` parameter,
    /// [`QueryDisplayConfig`] returns the topology identifier of the active database topology in
    /// the variable that the `current_topology_id` parameter points to. If the [`QDC_ALL_PATHS`]
    /// or [`QDC_ONLY_ACTIVE_PATHS`] flag is set in the `flags` parameter, the
    /// `current_topology_id` parameter must be set to [`null_mut`]; otherwise,
    /// [`QueryDisplayConfig`] returns [`ERROR_INVALID_PARAMETER`].
    ///
    /// If a caller calls [`QueryDisplayConfig`] with the [`QDC_DATABASE_CURRENT`] flag set in the
    /// `flags` parameter, [`QueryDisplayConfig`] initializes the [`DISPLAYCONFIG_2DREGION`]
    /// structure that is specified in the `total_size` member of the
    /// [`DISPLAYCONFIG_VIDEO_SIGNAL_INFO`] structure to zeros and does not complete
    /// [`DISPLAYCONFIG_2DREGION`].
    ///
    /// The [`DEVMODE`] structure that is returned by the [`EnumDisplaySettings`] Win32 function
    /// (described in the Windows SDK documentation) contains information that relates to both the
    /// source and target modes. However, the CCD APIs explicitly separate the source and target
    /// mode components.
    pub fn QueryDisplayConfig(
        flags: UINT32,
        num_path_array_elements: *mut UINT32,
        path_array: *mut DISPLAYCONFIG_PATH_INFO,
        num_mode_info_array_elements: *mut UINT32,
        mode_info_array: *mut DISPLAYCONFIG_MODE_INFO,
        current_topology_id: *mut DISPLAYCONFIG_TOPOLOGY_ID,
    ) -> LONG;
}
