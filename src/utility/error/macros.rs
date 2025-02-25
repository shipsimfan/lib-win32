// rustdoc imports
#[allow(unused_imports)]
use crate::{winsock2::SOCKET_ERROR, S_OK};
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

/// Convert an HRESULT result ([`S_OK`] on success) into a [`Result<()>`]
#[macro_export]
macro_rules! try_hresult {
    ($expr: expr) => {{
        #[allow(unused_unsafe)]
        let result = unsafe { $expr };
        if result == $crate::S_OK || result == $crate::S_FALSE {
            Ok(())
        } else {
            Err($crate::Error::new(result))
        }
    }};
}
