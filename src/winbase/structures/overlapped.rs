use crate::{DWORD, HANDLE, PVOID, ULONG_PTR};
use std::ops::{Deref, DerefMut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{ReadFile, WriteFile, ERROR_INVALID_PARAMETER, TRUE};

/// Contains information used in asynchronous (or overlapped) input and output (I/O).
///
/// # Remarks
/// Any unused members of this structure should always be initialized to zero before the structure
/// is used in a function call. Otherwise, the function may fail and return
/// [`ERROR_INVALID_PARAMETER`].
///
/// The `offset` and `offset_high` members together represent a 64-bit file position. It is a byte
/// offset from the start of the file or file-like device, and it is specified by the user; the
/// system will not modify these values. The calling process must set this member before passing
/// the [`OVERLAPPED`] structure to functions that use an offset, such as the [`ReadFile`] or
/// [`WriteFile`] (and related) functions.
///
/// You can use the [`HasOverlappedIoCompleted`] macro to check whether an asynchronous I/O
/// operation has completed if [`GetOverlappedResult`] is too cumbersome for your application.
///
/// You can use the [`CancelIo`] function to cancel an asynchronous I/O operation.
///
/// A common mistake is to reuse an [`OVERLAPPED`] structure before the previous asynchronous
/// operation has been completed. You should use a separate structure for each request. You should
/// also create an event object for each thread that processes data. If you store the event handles
/// in an array, you could easily wait for all events to be signaled using the
/// [`WaitForMultipleObjects`] function.
#[repr(C)]
#[derive(Clone)]
pub struct OVERLAPPED {
    /// The status code for the I/O request. When the request is issued, the system sets this
    /// member to [`STATUS_PENDING`] to indicate that the operation has not yet started. When the
    /// request is completed, the system sets this member to the status code for the completed
    /// request.
    ///
    /// The `internal` member was originally reserved for system use and its behavior may change.
    pub internal: ULONG_PTR,

    /// The number of bytes transferred for the I/O request. The system sets this member if the
    /// request is completed without errors.
    ///
    /// The `internal_high` member was originally reserved for system use and its behavior may
    /// change.
    pub internal_high: ULONG_PTR,

    #[allow(missing_docs)]
    pub dummy: OVERLAPPED_UNION,

    /// A handle to the event that will be set to a signaled state by the system when the operation
    /// has completed. The user must initialize this member either to zero or a valid event handle
    /// using the [`CreateEvent`] function before passing this structure to any overlapped
    /// functions. This event can then be used to synchronize simultaneous I/O requests for a
    /// device.
    ///
    /// Functions such as [`ReadFile`] and [`WriteFile`] set this handle to the nonsignaled state
    /// before they begin an I/O operation. When the operation has completed, the handle is set to
    /// the signaled state.
    ///
    /// Functions such as [`GetOverlappedResult`] and the synchronization wait functions reset
    /// auto-reset events to the nonsignaled state. Therefore, you should use a manual reset event;
    /// if you use an auto-reset event, your application can stop responding if you wait for the
    /// operation to complete and then call [`GetOverlappedResult`] with the `wait` parameter set
    /// to [`TRUE`].
    pub event: HANDLE,
}

impl Deref for OVERLAPPED {
    type Target = OVERLAPPED_UNION;

    fn deref(&self) -> &Self::Target {
        &self.dummy
    }
}

impl DerefMut for OVERLAPPED {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dummy
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub union OVERLAPPED_UNION {
    #[allow(missing_docs)]
    pub dummy: OVERLAPPED_STRUCT,

    /// Reserved for system use; do not use after initialization to zero.
    pub pointer: PVOID,
}

impl Deref for OVERLAPPED_UNION {
    type Target = OVERLAPPED_STRUCT;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.dummy }
    }
}

impl DerefMut for OVERLAPPED_UNION {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.dummy }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub struct OVERLAPPED_STRUCT {
    /// The low-order portion of the file position at which to start the I/O request, as specified
    /// by the user.
    ///
    /// This member is nonzero only when performing I/O requests on a seeking device that supports
    /// the concept of an offset (also referred to as a file pointer mechanism), such as a file.
    /// Otherwise, this member must be zero.
    pub offset: DWORD,

    /// The high-order portion of the file position at which to start the I/O request, as specified
    /// by the user.
    ///
    /// This member is nonzero only when performing I/O requests on a seeking device that supports
    /// the concept of an offset (also referred to as a file pointer mechanism), such as a file.
    /// Otherwise, this member must be zero.
    pub offset_high: DWORD,
}
