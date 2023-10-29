use crate::{
    raw::{GetLastError, LPCWStr, LPWStr, LocalAlloc, LocalFree, SizeT, WChar},
    Str,
};
use std::{
    borrow::{Borrow, Cow},
    ptr::{null_mut, NonNull},
};

/// A Windows UTF-16 string
pub struct String {
    ptr: NonNull<WChar>,
    length: usize,
    capacity: usize,
}

const INITIAL_CAPACITY: usize = 4;

/// (Re)allocates `initial_ptr` to be `new_capacity` [`WChar`]s long
fn alloc(initial_ptr: Option<(NonNull<WChar>, usize)>, new_capacity: usize) -> NonNull<u16> {
    assert_ne!(new_capacity, 0, "Cannot allocate a zero capacity array");

    let bytes = (new_capacity * 2) as SizeT;

    let new_ptr = match NonNull::new(unsafe { LocalAlloc(0, bytes) }) {
        Some(new_ptr) => new_ptr.cast(),
        None => panic!(
            "Call to Win32 function \"LocalAlloc()\" failed: {:#010X}",
            unsafe { GetLastError() }
        ),
    };

    if let Some((initial_ptr, old_capacity)) = initial_ptr {
        let old_slice = unsafe { std::slice::from_raw_parts(initial_ptr.as_ptr(), old_capacity) };
        let new_slice = unsafe { std::slice::from_raw_parts_mut(new_ptr.as_ptr(), old_capacity) };

        new_slice.copy_from_slice(old_slice);

        if unsafe { LocalFree(initial_ptr.cast().as_ptr()) } != null_mut() {
            panic!(
                "Call to Win32 function \"LocalFree()\" failed: {:#010X}",
                unsafe { GetLastError() }
            );
        }
    }

    new_ptr
}

impl String {
    /// Creates a new Windows string with a null-terminator
    pub fn new() -> Self {
        // Allocate the initial array
        let mut ptr = alloc(None, INITIAL_CAPACITY);

        // Set the first value to zero
        unsafe { *ptr.as_mut() = 0 };

        String {
            ptr,
            length: 1,
            capacity: INITIAL_CAPACITY,
        }
    }

    /// Creates a Windows string from a raw pointer, length, and capacity
    pub unsafe fn from_raw_parts(ptr: LPWStr, length: usize, capacity: usize) -> Self {
        assert!(length > 0);
        assert!(capacity >= length);
        assert_ne!(ptr, null_mut());
        assert!(unsafe { *ptr.add(length - 1) } == 0); // SAFTEY: `ptr` is checked for null above
        String {
            ptr: unsafe { NonNull::new_unchecked(ptr) }, // SAFTEY: `ptr` is checked for null above
            length,
            capacity,
        }
    }

    /// Gets the underlying pointer
    pub fn as_ptr(&self) -> LPCWStr {
        self.ptr.as_ptr()
    }

    /// Gets the string as words without the null-terminator
    pub fn as_words(&self) -> &[u16] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.length - 1) }
    }

    /// Gets the string as words with the null-terminator
    pub fn as_words_with_null(&self) -> &[u16] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.length) }
    }

    pub fn as_str(&self) -> &Str {
        unsafe { Str::from_slice_unchecked(self.as_words_with_null()) }
    }

    pub fn as_ptr_mut(&mut self) -> LPWStr {
        self.ptr.as_ptr()
    }

    pub fn as_str_mut(&mut self) -> &mut Str {
        unsafe { Str::from_slice_unchecked_mut(self.as_words_with_null_mut()) }
    }

    /// Pushes the passed word into the string maintaining the null-terminator
    pub fn push(&mut self, word: u16) {
        assert_ne!(word, 0);

        // Grow the underlying array if needed
        if self.length == self.capacity {
            let old_capacity = self.capacity;
            self.capacity *= 2;
            self.ptr = alloc(Some((self.ptr, old_capacity)), self.capacity);
        }

        // This must be done here due to the borrow from `words`
        let new_word_idx = self.length - 1;
        let zero_idx = self.length;
        self.length += 1;

        let words = unsafe { self.as_words_with_null_mut() };

        // Add the new word
        words[new_word_idx] = word;
        // Add the null-terminator
        words[zero_idx] = 0;
    }

    unsafe fn as_words_with_null_mut(&mut self) -> &mut [u16] {
        std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.length)
    }
}

impl Borrow<Str> for String {
    fn borrow(&self) -> &Str {
        self.as_str()
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for char in std::char::decode_utf16(self.as_words().iter().map(|word| *word)) {
            char.map_err(|_| std::fmt::Error)?.fmt(f)?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_words().fmt(f)
    }
}

impl Drop for String {
    fn drop(&mut self) {
        if unsafe { LocalFree(self.ptr.cast().as_ptr()) } != null_mut() {
            panic!("Call to Win32 function \"LocalFree()\" failed");
        }
    }
}

impl From<std::string::String> for String {
    fn from(value: std::string::String) -> Self {
        value.encode_utf16().collect()
    }
}

impl<'a> From<&'a str> for String {
    fn from(value: &'a str) -> Self {
        value.encode_utf16().collect()
    }
}

impl<'a> From<Cow<'a, str>> for String {
    fn from(value: Cow<'a, str>) -> Self {
        (*value).into()
    }
}

impl FromIterator<u16> for String {
    fn from_iter<T: IntoIterator<Item = u16>>(iter: T) -> Self {
        let mut this = String::new();

        for word in iter {
            this.push(word);
        }

        this
    }
}
