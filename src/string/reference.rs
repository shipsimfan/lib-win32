use crate::{raw::LPCWStr, String};
use std::ptr::null;

/// A borrowed Windows UTF-16 string
pub struct Str<'a>(&'a [u16]);

impl<'a> Str<'a> {
    /// Creates a Windows string from a slice
    pub fn from_slice(slice: &'a [u16]) -> Self {
        assert_ne!(slice.len(), 0, "Cannot create a zero length Win32 string");
        assert_eq!(
            *slice.last().unwrap(),
            0,
            "Win32 string must end with a NULL-terminator"
        );

        unsafe { Str::from_slice_unchecked(slice) }
    }

    /// Creates a Windows string from a slice without checking for a NULL-terminator
    pub const unsafe fn from_slice_unchecked(slice: &'a [u16]) -> Self {
        Str(slice)
    }

    /// Creates a Windows string from a pointer to a NULL-terminated string
    pub unsafe fn from_ptr(ptr: *const u16) -> Self {
        assert_ne!(ptr, null(), "Cannot create a Win32 from a null pointer");

        let mut p = ptr;
        let mut len = 1;
        while *p != 0 {
            p = p.add(1);
            len += 1;
        }

        let slice = std::slice::from_raw_parts(ptr, len);

        Str::from_slice_unchecked(slice)
    }

    /// Gets the underlying pointer
    pub fn as_ptr(&self) -> LPCWStr {
        self.0.as_ptr()
    }

    /// Gets the string as words without the null-terminator
    pub fn as_words(&self) -> &'a [u16] {
        &self.0[..self.0.len() - 1]
    }

    /// Gets the string as words with the null-terminator
    pub fn as_words_with_null(&self) -> &'a [u16] {
        self.0
    }
}

impl<'a> std::fmt::Display for Str<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for char in std::char::decode_utf16(self.as_words().iter().map(|word| *word)) {
            char.map_err(|_| std::fmt::Error)?.fmt(f)?;
        }

        Ok(())
    }
}

impl<'a> std::fmt::Debug for Str<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_words().fmt(f)
    }
}

impl<'a> From<&'a String> for Str<'a> {
    fn from(value: &'a String) -> Self {
        unsafe { Str::from_slice_unchecked(value.as_words_with_null()) }
    }
}
