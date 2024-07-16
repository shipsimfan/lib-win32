use crate::{DWORD, LPTIME_ZONE_INFORMATION};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, TIME_ZONE_INFORMATION};

#[link(name = "Kernel32")]
extern "system" {
    /// Retrieves the current time zone settings. These settings control the translations between
    /// Coordinated Universal Time (UTC) and local time.
    ///
    /// To support boundaries for daylight saving time that change from year to year, use the
    /// [`GetDynamicTimeZoneInformation`] or [`GetTimeZoneInformationForYear`] function.
    ///
    /// # Parameters
    ///  * `time_zone_information` - A pointer to a [`TIME_ZONE_INFORMATION`] structure to receive
    ///                              the current settings.
    ///
    /// # Return Value
    /// If the function succeeds, it returns one of the following values:
    ///  * [`TIME_ZONE_ID_UNKNOWN`] - Daylight saving time is not used in the current time zone,
    ///                               because there are no transition dates or automatic adjustment
    ///                               for daylight saving time is disabled.
    ///  * [`TIME_ZONE_ID_STANDARD`] - The system is operating in the range covered by the
    ///                                `standard_date` member of the [`TIME_ZONE_INFORMATION`]
    ///                                structure.
    ///  * [`TIME_ZONE_ID_DAYLIGHT`] - The system is operating in the range covered by the
    ///                                `daylight_date` member of the [`TIME_ZONE_INFORMATION`]
    ///                                structure.
    ///
    /// If the function fails for other reasons, such as an out of memory error, it returns
    /// [`TIME_ZONE_ID_INVALID`]. To get extended error information, call [`GetLastError`].
    ///
    /// # Remarks
    /// All translations between UTC time and local time are based on the following formula:
    /// `UTC = local time + bias`
    ///
    /// The bias is the difference, in minutes, between UTC time and local time.
    ///
    /// The `standard_name` and `daylight_name` members of the resultant [`TIME_ZONE_INFORMATION`]
    /// structure are localized according to the current user default UI language.
    pub fn GetTimeZoneInformation(time_zone_information: LPTIME_ZONE_INFORMATION) -> DWORD;
}
