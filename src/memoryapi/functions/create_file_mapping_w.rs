use crate::{DWORD, HANDLE, LPCWSTR, LPSECURITY_ATTRIBUTES};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateFileMapping, GetLargePageMinimum, GetLastError, VirtualAlloc, VirtualFree, ACL,
    ERROR_ALREADY_EXISTS, ERROR_DISK_FULL, ERROR_FILE_INVALID, ERROR_INVALID_HANDLE,
    GENERIC_EXECUTE, GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE, PAGE_EXECUTE_READ,
    PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY, PAGE_READONLY, PAGE_READWRITE, PAGE_WRITECOPY,
    SECURITY_ATTRIBUTES, SEC_COMMIT, SEC_IMAGE, SEC_IMAGE_NO_EXECUTE, SEC_LARGE_PAGES, SEC_NOCACHE,
    SEC_RESERVE, SEC_WRITECOMBINE,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "Kernel32")]
extern "system" {
    /// Creates or opens a named or unnamed file mapping object for a specified file.
    ///
    /// To specify the NUMA node for the physical memory, see [`CreateFileMappingNuma`].
    ///
    /// # Parameters
    ///  * `file` - A handle to the file from which to create a file mapping object. The file must
    ///             be opened with access rights that are compatible with the protection flags that
    ///             the `protect` parameter specifies. It is not required, but it is recommended
    ///             that files you intend to map be opened for exclusive access. If `file` is
    ///             [`INVALID_HANDLE_VALUE`], the calling process must also specify a size for the
    ///             file mapping object in the `maximum_size_high` and `maximum_size_low`
    ///             parameters. In this scenario, [`CreateFileMapping`] creates a file mapping
    ///             object of a specified size that is backed by the system paging file instead of
    ///             by a file in the file system.
    ///  * `file_mapping_attributes` - A pointer to a [`SECURITY_ATTRIBUTES`] structure that
    ///                                determines whether a returned handle can be inherited by
    ///                                child processes. The `security_descriptor` member of the
    ///                                [`SECURITY_ATTRIBUTES`] structure specifies a security
    ///                                descriptor for a new file mapping object. If `attributes` is
    ///                                [`null_mut`], the handle cannot be inherited and the file
    ///                                mapping object gets a default security descriptor. The
    ///                                access control lists ([`ACL`]) in the default security
    ///                                descriptor for a file mapping object come from the primary
    ///                                or impersonation token of the creator.
    ///  * `protect` - Specifies the page protection of the file mapping object. All mapped views
    ///                of the object must be compatible with this protection. This parameter can
    ///                contain the following values:
    ///    * [`PAGE_EXECUTE_READ`] - Allows views to be mapped for read-only, copy-on-write, or
    ///                              execute access. The file handle specified by the `file`
    ///                              parameter must be created with the [`GENERIC_READ`] and
    ///                              [`GENERIC_EXECUTE`] access rights. Windows Server 2003 and
    ///                              Windows XP:  This value is not available until Windows XP with
    ///                              SP2 and Windows Server 2003 with SP1.
    ///    * [`PAGE_EXECUTE_READWRITE`] - Allows views to be mapped for read-only, copy-on-write,
    ///                                   read/write, or execute access. The file handle that the
    ///                                   `file` parameter specifies must be created with the
    ///                                   [`GENERIC_READ`], [`GENERIC_WRITE`], and
    ///                                   [`GENERIC_EXECUTE`] access rights. Windows Server 2003
    ///                                   and Windows XP: This value is not available until Windows
    ///                                   XP with SP2 and Windows Server 2003 with SP1.
    ///    * [`PAGE_EXECUTE_WRITECOPY`] - Allows views to be mapped for read-only, copy-on-write,
    ///                                   or execute access. This value is equivalent to
    ///                                   [`PAGE_EXECUTE_READ`]. The file handle that the `file`
    ///                                   parameter specifies must be created with the
    ///                                   [`GENERIC_READ`] and [`GENERIC_EXECUTE`] access rights.
    ///                                   Windows Vista: This value is not available until Windows
    ///                                   Vista with SP1. Windows Server 2003 and Windows XP: This
    ///                                   value is not supported.
    ///    * [`PAGE_READONLY`] - Allows views to be mapped for read-only or copy-on-write access.
    ///                          An attempt to write to a specific region results in an access
    ///                          violation. The file handle that the `file` parameter specifies
    ///                          must be created with the [`GENERIC_READ`] access right.
    ///    * [`PAGE_READWRITE`] - Allows views to be mapped for read-only, copy-on-write, or
    ///                           read/write access. The file handle that the `file` parameter
    ///                           specifies must be created with the [`GENERIC_READ`] and
    ///                           [`GENERIC_WRITE`] access rights.
    ///    * [`PAGE_WRITECOPY`] - Allows views to be mapped for read-only or copy-on-write access.
    ///                           This value is equivalent to [`PAGE_READONLY`]. The file handle
    ///                           that the hFile parameter specifies must be created with the
    ///                           [`GENERIC_READ`] access right.
    ///    * [`SEC_COMMIT`] - If the file mapping object is backed by the operating system paging
    ///                       file (the `file` parameter is [`INVALID_HANDLE_VALUE`]), specifies
    ///                       that when a view of the file is mapped into a process address space,
    ///                       the entire range of pages is committed rather than reserved. The
    ///                       system must have enough committable pages to hold the entire mapping.
    ///                       Otherwise, [`CreateFileMapping`] fails. This attribute has no effect
    ///                       for file mapping objects that are backed by executable image files or
    ///                       data files (the `file` parameter is a handle to a file).
    ///                       [`SEC_COMMIT`] cannot be combined with [`SEC_RESERVE`]. If no
    ///                       attribute is specified, [`SEC_COMMIT`] is assumed. However,
    ///                       [`SEC_COMMIT`] must be explicitly specified when combining it with
    ///                       another `SEC_` attribute that requires it.
    ///    * [`SEC_IMAGE`] - Specifies that the file that the `file` parameter specifies is an
    ///                      executable image file. The [`SEC_IMAGE`] attribute must be combined
    ///                      with a page protection value such as [`PAGE_READONLY`]. However, this
    ///                      page protection value has no effect on views of the executable image
    ///                      file. Page protection for views of an executable image file is
    ///                      determined by the executable file itself. No other attributes are
    ///                      valid with [`SEC_IMAGE`].
    ///    * [`SEC_IMAGE_NO_EXECUTE`] - Specifies that the file that the `file` parameter specifies
    ///                                 is an executable image file that will not be executed and
    ///                                 the loaded image file will have no forced integrity checks
    ///                                 run. Additionally, mapping a view of a file mapping object
    ///                                 created with the [`SEC_IMAGE_NO_EXECUTE`] attribute will
    ///                                 not invoke driver callbacks registered using the
    ///                                 `PsSetLoadImageNotifyRoutine` kernel API. The
    ///                                 [`SEC_IMAGE_NO_EXECUTE`] attribute must be combined with
    ///                                 the [`PAGE_READONLY`] page protection value. No other
    ///                                 attributes are valid with [`SEC_IMAGE_NO_EXECUTE`]. Windows
    ///                                 Server 2008 R2, Windows 7, Windows Server 2008, Windows
    ///                                 Vista, Windows Server 2003 and Windows XP:  This value is
    ///                                 not supported before Windows Server 2012 and Windows 8.
    ///    * [`SEC_LARGE_PAGES`] - Enables large pages to be used for file mapping objects that are
    ///                            backed by the operating system paging file (the `file` parameter
    ///                            is [`INVALID_HANDLE_VALUE`]). This attribute is not supported
    ///                            for file mapping objects that are backed by executable image
    ///                            files or data files (the `file` parameter is a handle to an
    ///                            executable image or data file). The maximum size of the file
    ///                            mapping object must be a multiple of the minimum size of a large
    ///                            page returned by the [`GetLargePageMinimum`] function. If it is
    ///                            not, [`CreateFileMapping`] fails. When mapping a view of a file
    ///                            mapping object created with [`SEC_LARGE_PAGES`], the base
    ///                            address and view size must also be multiples of the minimum
    ///                            large page size. [`SEC_LARGE_PAGES`] requires the
    ///                            "SeLockMemoryPrivilege" privilege to be enabled in the caller's
    ///                            token. If [`SEC_LARGE_PAGES`] is specified, [`SEC_COMMIT`] must
    ///                            also be specified. Windows Server 2003: This value is not
    ///                            supported until Windows Server 2003 with SP1. Windows XP: This
    ///                            value is not supported.
    ///    * [`SEC_NOCACHE`] - Sets all pages to be non-cacheable. Applications should not use this
    ///                        attribute except when explicitly required for a device. Using the
    ///                        interlocked functions with memory that is mapped with
    ///                        [`SEC_NOCACHE`] can result in an `EXCEPTION_ILLEGAL_INSTRUCTION`
    ///                        exception. [`SEC_NOCACHE`] requires either the [`SEC_RESERVE`] or
    ///                        [`SEC_COMMIT`] attribute to be set.
    ///    * [`SEC_RESERVE`] - If the file mapping object is backed by the operating system paging
    ///                        file (the `file` parameter is [`INVALID_HANDLE_VALUE`]), specifies
    ///                        that when a view of the file is mapped into a process address space,
    ///                        the entire range of pages is reserved for later use by the process
    ///                        rather than committed. Reserved pages can be committed in subsequent
    ///                        calls to the [`VirtualAlloc`] function. After the pages are
    ///                        committed, they cannot be freed or decommitted with the
    ///                        [`VirtualFree`] function. This attribute has no effect for file
    ///                        mapping objects that are backed by executable image files or data
    ///                        files (the `file` parameter is a handle to a file). [`SEC_RESERVE`]
    ///                        cannot be combined with [`SEC_COMMIT`].
    ///    * [`SEC_WRITECOMBINE`] - Sets all pages to be write-combined. Applications should not
    ///                             use this attribute except when explicitly required for a
    ///                             device. Using the interlocked functions with memory that is
    ///                             mapped with [`SEC_WRITECOMBINE`] can result in an
    ///                             `EXCEPTION_ILLEGAL_INSTRUCTION` exception. [`SEC_WRITECOMBINE`]
    ///                             requires either the [`SEC_RESERVE`] or [`SEC_COMMIT`] attribute
    ///                             to be set. Windows Server 2003 and Windows XP: This flag is not
    ///                             supported until Windows Vista.
    ///  * `maximum_size_high` - The high-order [`DWORD`] of the maximum size of the file mapping
    ///                          object.
    ///  * `maximum_size_low` - The low-order [`DWORD`] of the maximum size of the file mapping
    ///                         object. If this parameter and `maximum_size_high` are 0 (zero), the
    ///                         maximum size of the file mapping object is equal to the current
    ///                         size of the file that `file` identifies. An attempt to map a file
    ///                         with a length of 0 (zero) fails with an error code of
    ///                         [`ERROR_FILE_INVALID`]. Applications should test for files with a
    ///                         length of 0 (zero) and reject those files.
    ///  * `name` - The name of the file mapping object. If this parameter matches the name of an
    ///             existing mapping object, the function requests access to the object with the
    ///             protection that `protect` specifies. If this parameter is [`null`], the file
    ///             mapping object is created without a name. If `name` matches the name of an
    ///             existing event, semaphore, mutex, waitable timer, or job object, the function
    ///             fails, and the [`GetLastError`] function returns [`ERROR_INVALID_HANDLE`]. This
    ///             occurs because these objects share the same namespace. The name can have a
    ///             "Global" or "Local" prefix to explicitly create the object in the global or
    ///             session namespace. The remainder of the name can contain any character except
    ///             the backslash character (\). Creating a file mapping object in the global
    ///             namespace from a session other than session zero requires the
    ///             "SeCreateGlobalPrivilege" privilege. Fast user switching is implemented by
    ///             using Terminal Services sessions. The first user to log on uses session 0
    ///             (zero), the next user to log on uses session 1 (one), and so on. Kernel object
    ///             names must follow the guidelines that are outlined for Terminal Services so
    ///             that applications can support multiple users.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the newly created file mapping
    /// object.
    ///
    /// If the object exists before the function call, the function returns a handle to the
    /// existing object (with its current size, not the specified size), and [`GetLastError`]
    /// returns [`ERROR_ALREADY_EXISTS`].
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// After a file mapping object is created, the size of the file must not exceed the size of
    /// the file mapping object; if it does, not all of the file contents are available for
    /// sharing.
    ///
    /// If an application specifies a size for the file mapping object that is larger than the size
    /// of the actual named file on disk and if the page protection allows write access (that is,
    /// the `protect` parameter specifies [`PAGE_READWRITE`] or [`PAGE_EXECUTE_READWRITE`]), then
    /// the file on disk is increased to match the specified size of the file mapping object. If
    /// the file is extended, the contents of the file between the old end of the file and the new
    /// end of the file are not guaranteed to be zero; the behavior is defined by the file system.
    /// If the file on disk cannot be increased, [`CreateFileMapping`] fails and [`GetLastError`]
    /// returns [`ERROR_DISK_FULL`].
    ///
    /// The initial contents of the pages in a file mapping object backed by the operating system
    /// paging file are 0 (zero).
    ///
    /// The handle that [`CreateFileMapping`] returns has full access to a new file mapping object,
    /// and can be used with any function that requires a handle to a file mapping object.
    ///
    /// Multiple processes can share a view of the same file by either using a single shared file
    /// mapping object or creating separate file mapping objects backed by the same file. A single
    /// file mapping object can be shared by multiple processes through inheriting the handle at
    /// process creation, duplicating the handle, or opening the file mapping object by name. For
    /// more information, see the [`CreateProcess`], [`DuplicateHandle`] and [`OpenFileMapping`]
    /// functions.
    ///
    /// Creating a file mapping object does not actually map the view into a process address space.
    /// The [`MapViewOfFile`] and [`MapViewOfFileEx`] functions map a view of a file into a process
    /// address space.
    ///
    /// With one important exception, file views derived from any file mapping object that is
    /// backed by the same file are coherent or identical at a specific time. Coherency is
    /// guaranteed for views within a process and for views that are mapped by different processes.
    ///
    /// The exception is related to remote files. Although [`CreateFileMapping`] works with remote
    /// files, it does not keep them coherent. For example, if two computers both map a file as
    /// writable, and both change the same page, each computer only sees its own writes to the
    /// page. When the data gets updated on the disk, it is not merged.
    ///
    /// A mapped file and a file that is accessed by using the input and output (I/O) functions
    /// ([`ReadFile`] and [`WriteFile`]) are not necessarily coherent.
    ///
    /// Mapped views of a file mapping object maintain internal references to the object, and a
    /// file mapping object does not close until all references to it are released. Therefore, to
    /// fully close a file mapping object, an application must unmap all mapped views of the file
    /// mapping object by calling [`UnmapViewOfFile`] and close the file mapping object handle by
    /// calling [`CloseHandle`]. These functions can be called in any order.
    ///
    /// When modifying a file through a mapped view, the last modification timestamp may not be
    /// updated automatically. If required, the caller should use SetFileTime to set the timestamp.
    ///
    /// Creating a file mapping object in the global namespace from a session other than session
    /// zero requires the "SeCreateGlobalPrivilege" privilege. Note that this privilege check is
    /// limited to the creation of file mapping objects and does not apply to opening existing
    /// ones. For example, if a service or the system creates a file mapping object in the global
    /// namespace, any process running in any session can access that file mapping object provided
    /// that the caller has the required access rights.
    ///
    /// Windows XP: The requirement described in the previous paragraph was introduced with Windows
    /// Server 2003 and Windows XP with SP2
    ///
    /// Use structured exception handling to protect any code that writes to or reads from a file
    /// view.
    ///
    /// To have a mapping with executable permissions, an application must call
    /// [`CreateFileMapping`] with either [`PAGE_EXECUTE_READWRITE`] or [`PAGE_EXECUTE_READ`], and
    /// then call [`MapViewOfFile`] with `FILE_MAP_EXECUTE | FILE_MAP_WRITE` or
    /// `FILE_MAP_EXECUTE | FILE_MAP_READ`.
    pub fn CreateFileMappingW(
        file: HANDLE,
        file_mapping_attributes: LPSECURITY_ATTRIBUTES,
        protect: DWORD,
        maximum_size_high: DWORD,
        maximum_size_low: DWORD,
        name: LPCWSTR,
    ) -> HANDLE;
}
