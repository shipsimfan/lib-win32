use crate::SIZE_T;

#[link(name = "Kernel32")]
extern "system" {
    /// Retrieves the minimum size of a large page.
    ///
    /// # Return Value
    /// If the processor supports large pages, the return value is the minimum size of a large
    /// page.
    ///
    /// If the processor does not support large pages, the return value is zero.
    ///
    /// # Remarks
    /// The minimum large page size varies, but it is typically 2 MB or greater.
    pub fn GetLargePageMinimum() -> SIZE_T;
}
