use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::TIME_ZONE_INFORMATION;

/// Daylight saving time is not used in the current time zone, because there are no transition 
/// dates or automatic adjustment for daylight saving time is disabled.
pub const TIME_ZONE_ID_UNKNOWN: DWORD = 0;

/// The system is operating in the range covered by the `standard_date` member of the 
/// [`TIME_ZONE_INFORMATION`] structure.
pub const TIME_ZONE_ID_STANDARD: DWORD = 1;

/// The system is operating in the range covered by the `daylight_date` member of the 
/// [`TIME_ZONE_INFORMATION`] structure.
pub const TIME_ZONE_ID_DAYLIGHT: DWORD = 2;

/// The system could not get information for the requested time zone.
pub const TIME_ZONE_ID_INVALID: DWORD = 0xFFFFFFFF;
