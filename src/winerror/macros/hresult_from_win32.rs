// rustdoc imports
#[allow(unused_imports)]
use crate::HRESULT;

/// Maps a system error code to an [`HRESULT`] value.
#[macro_export]
macro_rules! HRESULT_FROM_WIN32 {
    ($x: expr) => {
        let x = $x as $crate::HRESULT;
        if x <= 0 {
            x
        } else {
            (x & 0x0000FFFF) | ($crate::FACILITY_WIN32 << 16) | 0x80000000
        }
    };
}
