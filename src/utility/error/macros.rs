// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::SOCKET_ERROR;
#[allow(unused_imports)]
use std::ptr::null;

/// Convert a Windows call result (0/[`null`] on error) into a [`Result<T>`]
#[macro_export]
macro_rules! try_get_last_error {
    ($expr: expr) => {{
        let result = unsafe { $expr };
        if result as usize == 0 {
            Err($crate::Error::get_last_error())
        } else {
            Ok(result)
        }
    }};
}

/// Convert a Windows Socket call result ([`SOCKET_ERROR`] on error) into a [`Result<T>`]
#[macro_export]
macro_rules! try_wsa_get_last_error {
    ($expr: expr) => {{
        let result = unsafe { $expr };
        if result == $crate::winsock2::SOCKET_ERROR {
            Err($crate::Error::wsa_get_last_error())
        } else {
            Ok(result)
        }
    }};
}
