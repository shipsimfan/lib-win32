use crate::{BOOL, CHAR_INFO, COORD, HANDLE, PSMALL_RECT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetConsoleScreenBufferInfo, GetLastError, GENERIC_WRITE, SMALL_RECT};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Writes character and color attribute data to a specified rectangular block of character
    /// cells in a console screen buffer. The data to be written is taken from a correspondingly
    /// sized rectangular block at a specified location in the source buffer.
    ///
    /// # Parameters
    ///  * `console_output` - A handle to the console screen buffer. The handle must have the
    ///                       [`GENERIC_WRITE`] access right.
    ///  * `buffer` - The data to be written to the console screen buffer. This pointer is treated
    ///               as the origin of a two-dimensional array of [`CHAR_INFO`] structures whose
    ///               size is specified by the `buffer_size` parameter.
    ///  * `buffer_size` - The size of the buffer pointed to by the `buffer` parameter, in
    ///                    character cells. The `x` member of the [`COORD`] structure is the number
    ///                    of columns; the `y` member is the number of rows.
    ///  * `buffer_coord` - The coordinates of the upper-left cell in the buffer pointed to by the
    ///                     `buffer` parameter. The `x` member of the [`COORD`] structure is the
    ///                     column, and the `y` member is the row.
    ///  * `write_region` - A pointer to a [`SMALL_RECT`] structure. On input, the structure
    ///                     members specify the upper-left and lower-right coordinates of the
    ///                     console screen buffer rectangle to write to. On output, the structure
    ///                     members specify the actual rectangle that was used.
    ///
    /// # Return value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// [`WriteConsoleOutput`] treats the source buffer and the destination screen buffer as
    /// two-dimensional arrays (columns and rows of character cells). The rectangle pointed to by
    /// the `write_region` parameter specifies the size and location of the block to be written to
    /// in the console screen buffer. A rectangle of the same size is located with its upper-left
    /// cell at the coordinates of the `buffer_coord` parameter in the `buffer` array. Data from
    /// the cells that are in the intersection of this rectangle and the source buffer rectangle
    /// (whose dimensions are specified by the `buffer_size` parameter) is written to the
    /// destination rectangle.
    ///
    /// Cells in the destination rectangle whose corresponding source location are outside the
    /// boundaries of the source buffer rectangle are left unaffected by the write operation. In
    /// other words, these are the cells for which no data is available to be written.
    ///
    /// Before [`WriteConsoleOutput`] returns, it sets the members of `write_region` to the actual
    /// screen buffer rectangle affected by the write operation. This rectangle reflects the cells
    /// in the destination rectangle for which there existed a corresponding cell in the source
    /// buffer, because [`WriteConsoleOutput`] clips the dimensions of the destination rectangle to
    /// the boundaries of the console screen buffer.
    ///
    /// If the rectangle specified by `write_region` lies completely outside the boundaries of the
    /// console screen buffer, or if the corresponding rectangle is positioned completely outside
    /// the boundaries of the source buffer, no data is written. In this case, the function returns
    /// with the members of the structure pointed to by the `write_region` parameter set such that
    /// the `right` member is less than the `left`, or the `bottom` member is less than the `top`.
    /// To determine the size of the console screen buffer, use the [`GetConsoleScreenBufferInfo`]
    /// function.
    ///
    /// [`WriteConsoleOutput`] has no effect on the cursor position.
    ///
    /// This function uses either Unicode characters or 8-bit characters from the console's current
    /// code page. The console's code page defaults initially to the system's OEM code page. To
    /// change the console's code page, use the [`SetConsoleCP`] or [`SetConsoleOutputCP`]
    /// functions. Legacy consumers may also use the chcp or mode con cp select= commands, but it
    /// is not recommended for new development.
    pub fn WriteConsoleOutput(
        console_output: HANDLE,
        buffer: *const CHAR_INFO,
        buffer_size: COORD,
        buffer_coord: COORD,
        write_region: PSMALL_RECT,
    ) -> BOOL;
}
