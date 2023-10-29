use crate::{
    raw::{LPCWStr, LPWStr},
    String,
};
use std::{
    ops::Deref,
    ptr::{null, null_mut},
};

/// A borrowed Windows UTF-16 string
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Str([u16]);

impl Str {
    /// Creates a Windows string from a slice
    pub fn from_slice(slice: &[u16]) -> &Self {
        assert_ne!(slice.len(), 0, "Cannot create a zero length Win32 string");
        assert_eq!(
            *slice.last().unwrap(),
            0,
            "Win32 string must end with a NULL-terminator"
        );

        unsafe { Str::from_slice_unchecked(slice) }
    }

    /// Creates a Windows string from a mutable slice
    pub fn from_slice_mut(slice: &mut [u16]) -> &mut Self {
        assert_ne!(slice.len(), 0, "Cannot create a zero length Win32 string");
        assert_eq!(
            *slice.last().unwrap(),
            0,
            "Win32 string must end with a NULL-terminator"
        );

        unsafe { Str::from_slice_unchecked_mut(slice) }
    }

    /// Creates a Windows string from a slice without checking for a NULL-terminator
    pub const unsafe fn from_slice_unchecked(slice: &[u16]) -> &Self {
        std::mem::transmute(slice)
    }

    /// Creates a Windows string from a mutable slice without checking for a NULL-terminator
    pub unsafe fn from_slice_unchecked_mut(slice: &mut [u16]) -> &mut Self {
        std::mem::transmute(slice)
    }

    /// Creates a Windows string from a pointer to a NULL-terminated string
    pub unsafe fn from_ptr<'a>(ptr: *const u16) -> &'a Self {
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

    /// Creates a Windows string from a mutable pointer to a NULL-terminated string
    pub unsafe fn from_ptr_mut<'a>(ptr: *mut u16) -> &'a mut Self {
        assert_ne!(ptr, null_mut(), "Cannot create a Win32 from a null pointer");

        let mut p = ptr;
        let mut len = 1;
        while *p != 0 {
            p = p.add(1);
            len += 1;
        }

        let slice = std::slice::from_raw_parts_mut(ptr, len);

        Str::from_slice_unchecked_mut(slice)
    }

    /// Gets the underlying pointer
    pub fn as_ptr(&self) -> LPCWStr {
        self.0.as_ptr()
    }
    /// Gets the string as words without the null-terminator
    pub fn as_words(&self) -> &[u16] {
        &self.0[..self.0.len() - 1]
    }

    /// Gets the string as words with the null-terminator
    pub fn as_words_with_null(&self) -> &[u16] {
        &self.0
    }

    /// Gets the underlying pointer
    pub fn as_ptr_mut(&mut self) -> LPWStr {
        self.0.as_mut_ptr()
    }
}

impl Deref for Str {
    type Target = [u16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for char in std::char::decode_utf16(self.as_words().iter().map(|word| *word)) {
            char.map_err(|_| std::fmt::Error)?.fmt(f)?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_words().fmt(f)
    }
}

impl<'a> From<&'a String> for &'a Str {
    fn from(value: &'a String) -> Self {
        unsafe { Str::from_slice_unchecked(value.as_words_with_null()) }
    }
}

impl ToOwned for Str {
    type Owned = String;

    fn to_owned(&self) -> Self::Owned {
        String::from_iter(self.0.iter().map(|word| *word))
    }
}
