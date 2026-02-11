use crate::{DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_DEVICE_INFO_TYPE, ULONG};

/// The [`DISPLAYCONFIG_SDR_WHITE_LEVEL`] structure contains information about a display's current
/// SDR white level. This is the brightness level that SDR "white" is rendered at within an HDR
/// monitor.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DISPLAYCONFIG_SDR_WHITE_LEVEL {
    /// A [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure that contains information for getting the
    /// SDR white level. The `r#type` member of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] is set to
    /// [`DISPLAYCONFIG_DEVICE_INFO_TYPE::GetSdrWhiteLevel`]. [`DISPLAYCONFIG_DEVICE_INFO_HEADER`]
    /// also contains the adapter and target identifiers of the target to get the SDR white level
    /// for. The `size` member of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] is set to at least the size
    /// of the [`DISPLAYCONFIG_SDR_WHITE_LEVEL`] structure.
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,

    /// The monitor's current SDR white level, specified as a multiplier of 80 nits, multiplied by
    /// 1000. E.g. a value of 1000 would indicate that the SDR white level is 80 nits, while a
    /// value of 2000 would indicate an SDR white level of 160 nits.
    ///
    /// ```no_run
    /// let sdr_white_level: DISPLAYCONFIG_SDR_WHITE_LEVEL;
    ///
    /// f32 sdr_white_level_in_nits = (sdr_white_level.sdr_white_level as f32) / 1000.0 * 80.0;
    /// ```
    pub sdr_white_level: ULONG,
}

impl Default for DISPLAYCONFIG_SDR_WHITE_LEVEL {
    fn default() -> Self {
        DISPLAYCONFIG_SDR_WHITE_LEVEL {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE::GetSdrWhiteLevel,
                size: std::mem::size_of::<DISPLAYCONFIG_SDR_WHITE_LEVEL>() as _,
                ..Default::default()
            },
            sdr_white_level: 0,
        }
    }
}
