use crate::WORD;

/// Specifies a date and time, using individual members for the month, day, year, weekday, hour,
/// minute, second, and millisecond. The time is either in coordinated universal time (UTC) or
/// local time, depending on the function that is being called.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SYSTEMTIME {
    /// The year. The valid values for this member are 1601 through 30827.
    pub year: WORD,

    /// The month. This member can be one of the following values:
    ///  * 1 - January
    ///  * 2 - February
    ///  * 3 - March
    ///  * 4 - April
    ///  * 5 - May
    ///  * 6 - June
    ///  * 7 - July
    ///  * 8 - August
    ///  * 9 - September
    ///  * 10 - October
    ///  * 11 - November
    ///  * 12 - December
    pub month: WORD,

    /// The day of the week. This member can be one of the following values:
    ///  * 0 - Sunday
    ///  * 1 - Monday
    ///  * 2 - Tuesday
    ///  * 3 - Wednesday
    ///  * 4 - Thursday
    ///  * 5 - Friday
    ///  * 6 - Saturday
    pub day_of_week: WORD,

    /// The day of the month. The valid values for this member are 1 through 31.
    pub day: WORD,

    /// The hour. The valid values for this member are 0 through 23.
    pub hour: WORD,

    /// The minute. The valid values for this member are 0 through 59.
    pub minute: WORD,

    /// The second. The valid values for this member are 0 through 59.
    pub second: WORD,

    /// The millisecond. The valid values for this member are 0 through 999.
    pub milliseconds: WORD,
}

impl Default for SYSTEMTIME {
    fn default() -> Self {
        SYSTEMTIME {
            year: 1601,
            month: 1,
            day_of_week: 0,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            milliseconds: 0,
        }
    }
}
