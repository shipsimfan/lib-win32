use crate::{LONG, SYSTEMTIME, WCHAR};

// rustdoc imports
#[allow(unused_imports)]
use crate::GetTimeZoneInformation;

/// Specifies settings for a time zone.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TIME_ZONE_INFORMATION {
    /// The current bias for local time translation on this computer, in minutes. The bias is the
    /// difference, in minutes, between Coordinated Universal Time (UTC) and local time. All
    /// translations between UTC and local time are based on the following formula:
    /// `UTC = local time + bias`
    ///
    /// This member is required.
    pub bias: LONG,

    /// A description for standard time. For example, "EST" could indicate Eastern Standard Time.
    /// The string will be returned unchanged by the [`GetTimeZoneInformation`] function. This
    /// string can be empty.
    pub standard_name: [WCHAR; 32],

    /// A [`SYSTEMTIME`] structure that contains a date and local time when the transition from
    /// daylight saving time to standard time occurs on this operating system. If the time zone
    /// does not support daylight saving time or if the caller needs to disable daylight saving
    /// time, the wMonth member in the [`SYSTEMTIME`] structure must be zero. If this date is
    /// specified, the `daylight_date` member of this structure must also be specified.
    ///
    /// Otherwise, the system assumes the time zone data is invalid and no changes will be applied.
    ///
    /// To select the correct day in the month, set the `year` member to zero, the `hour` and
    /// `minute` members to the transition time, the `day_of_week` member to the appropriate
    /// weekday, and the `day` member to indicate the occurrence of the day of the week within the
    /// month (1 to 5, where 5 indicates the final occurrence during the month if that day of the
    /// week does not occur 5 times).
    ///
    /// Using this notation, specify `02:00` on the first Sunday in April as follows: `hour` = 2,
    /// `hour` = 4, `day_of_week` = 0, `day` = 1. Specify `02:00` on the last Thursday in October
    /// as follows: `hour` = 2, `month` = 10, `day_of_week` = 4, `day` = 5.
    ///
    /// If the `year` member is not zero, the transition date is absolute; it will only occur one
    /// time. Otherwise, it is a relative date that occurs yearly.
    pub standard_date: SYSTEMTIME,

    /// The bias value to be used during local time translations that occur during standard time.
    /// This member is ignored if a value for the `standard_date` member is not supplied.
    ///
    /// This value is added to the value of the `bias` member to form the bias used during standard
    /// time. In most time zones, the value of this member is zero.
    pub standard_bias: LONG,

    /// A description for daylight saving time. For example, "PDT" could indicate Pacific Daylight
    /// Time. The string will be returned unchanged by the [`GetTimeZoneInformation`] function.
    /// This string can be empty.
    pub daylight_name: [WCHAR; 32],

    /// A [`SYSTEMTIME`] structure that contains a date and local time when the transition from
    /// standard time to daylight saving time occurs on this operating system. If the time zone
    /// does not support daylight saving time or if the caller needs to disable daylight saving
    /// time, the `month` member in the [`SYSTEMTIME`] structure must be zero. If this date is
    /// specified, the `standard_date` member in this structure must also be specified.
    ///
    /// Otherwise, the system assumes the time zone data is invalid and no changes will be applied.
    ///
    /// To select the correct day in the month, set the `year` member to zero, the `hour` and
    /// `minute` members to the transition time, the `day_of_week` member to the appropriate
    /// weekday, and the `day` member to indicate the occurrence of the day of the week within the
    /// month (1 to 5, where 5 indicates the final occurrence during the month if that day of the
    /// week does not occur 5 times).
    ///
    /// If the `year` member is not zero, the transition date is absolute; it will only occur one
    /// time. Otherwise, it is a relative date that occurs yearly.
    pub daylight_date: SYSTEMTIME,

    /// The bias value to be used during local time translations that occur during daylight saving
    /// time. This member is ignored if a value for the `daylight_date` member is not supplied.
    ///
    /// This value is added to the value of the `bias` member to form the bias used during daylight
    /// saving time. In most time zones, the value of this member is `–60`.
    pub daylight_bias: LONG,
}

impl Default for TIME_ZONE_INFORMATION {
    fn default() -> Self {
        TIME_ZONE_INFORMATION {
            bias: 0,
            standard_name: [0; 32],
            standard_date: SYSTEMTIME::default(),
            standard_bias: 0,
            daylight_name: [0; 32],
            daylight_date: SYSTEMTIME::default(),
            daylight_bias: 0,
        }
    }
}
