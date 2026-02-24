use crate::POINT;

/// Contains information about a window's maximized size and position and its minimum and maximum
/// tracking size.
///
/// # Remarks
/// For systems with multiple monitors, the `max_size` and `max_position` members describe the
/// maximized size and position of the window on the primary monitor, even if the window ultimately
/// maximizes onto a secondary monitor. In that case, the window manager adjusts these values to
/// compensate for differences between the primary monitor and the monitor that displays the
/// window. Thus, if the user leaves `max_size` untouched, a window on a monitor larger than the
/// primary monitor maximizes to the size of the larger monitor.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MINMAXINFO {
    /// Reserved; do not use.
    pub reserved: POINT,

    /// The maximized width (`x` member) and the maximized height (`y` member) of the window. For
    /// top-level windows, this value is based on the width of the primary monitor.
    pub max_size: POINT,

    /// The position of the left side of the maximized window (`x` member) and the position of the
    /// top of the maximized window (`y` member). For top-level windows, this value is based on the
    /// position of the primary monitor.
    pub max_position: POINT,

    /// The minimum tracking width (`x` member) and the minimum tracking height (`y` member) of the
    /// window. This value can be obtained programmatically from the system metrics
    /// [`SM_CXMINTRACK`] and [`SM_CYMINTRACK`] (see the [`GetSystemMetrics`] function).
    pub min_track_size: POINT,

    /// The maximum tracking width (`x` member) and the maximum tracking height (`y` member) of the
    /// window. This value is based on the size of the virtual screen and can be obtained
    /// programmatically from the system metrics [`SM_CXMAXTRACK`] and [`SM_CYMAXTRACK`] (see the
    /// [`GetSystemMetrics`] function).
    pub max_track_size: POINT,
}

impl Default for MINMAXINFO {
    fn default() -> Self {
        MINMAXINFO {
            reserved: POINT::default(),
            max_size: POINT::default(),
            max_position: POINT::default(),
            min_track_size: POINT::default(),
            max_track_size: POINT::default(),
        }
    }
}
