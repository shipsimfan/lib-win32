use crate::{
    string::wcslen, Error, FormatMessage, LocalFree, DWORD, FORMAT_MESSAGE_ALLOCATE_BUFFER,
    FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, LPWSTR,
};
use std::{
    fmt::Write,
    ptr::{null, null_mut},
};

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Create the message
        let mut buffer: LPWSTR = null_mut();
        unsafe {
            FormatMessage(
                FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS,
                null(),
                self.0 as DWORD,
                0,
                &mut buffer as *mut LPWSTR as _,
                0,
                null_mut(),
            )
        };

        // Verify there is a message
        if buffer == null_mut() {
            return write!(f, "unknown error {:#X}", self.0);
        }

        // Get the length and convert the buffer to a slice
        let length = unsafe { wcslen(buffer) };
        let slice = unsafe { std::slice::from_raw_parts_mut(buffer, length) };

        // Print the message
        let mut result = Ok(());
        for c in char::decode_utf16(slice.into_iter().map(|val| *val)) {
            let c = c.unwrap_or(char::REPLACEMENT_CHARACTER);
            if c == '\n' || c == '\r' {
                break;
            }

            result = f.write_char(c);
            if result.is_err() {
                break;
            }
        }

        // Free the message
        unsafe { LocalFree(buffer.cast()) };

        result
    }
}
