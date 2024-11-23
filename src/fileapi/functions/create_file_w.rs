use crate::{DWORD, HANDLE, LPCWSTR, LPSECURITY_ATTRIBUTES};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateFile, GetLastError, CREATE_ALWAYS, CREATE_NEW, ERROR_ALREADY_EXISTS, ERROR_FILE_EXISTS,
    ERROR_FILE_NOT_FOUND, ERROR_SHARING_VIOLATION, FILE_ATTRIBUTE_ARCHIVE,
    FILE_ATTRIBUTE_ENCRYPTED, FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_NORMAL, FILE_ATTRIBUTE_OFFLINE,
    FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_SYSTEM, FILE_ATTRIBUTE_TEMPORARY,
    FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_DELETE_ON_CLOSE, FILE_FLAG_NO_BUFFERING,
    FILE_FLAG_OPEN_NO_RECALL, FILE_FLAG_OPEN_REPARSE_POINT, FILE_FLAG_OVERLAPPED,
    FILE_FLAG_POSIX_SEMANTICS, FILE_FLAG_RANDOM_ACCESS, FILE_FLAG_SEQUENTIAL_SCAN,
    FILE_FLAG_SESSION_AWARE, FILE_FLAG_WRITE_THROUGH, FILE_SHARE_DELETE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE, OPEN_ALWAYS,
    OPEN_EXISTING, SECURITY_ANONYMOUS, SECURITY_ATTRIBUTES, SECURITY_CONTEXT_TRACKING,
    SECURITY_DELEGATION, SECURITY_DESCRIPTOR, SECURITY_EFFECTIVE_ONLY, SECURITY_IDENTIFICATION,
    SECURITY_IMPERSONATION, SECURITY_SQOS_PRESENT, TRUNCATE_EXISTING,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Creates or opens a file or I/O device. The most commonly used I/O devices are as follows:
    /// file, file stream, directory, physical disk, volume, console buffer, tape drive,
    /// communications resource, mailslot, and pipe. The function returns a handle that can be used
    /// to access the file or device for various types of I/O depending on the file or device and
    /// the flags and attributes specified.
    ///
    /// To perform this operation as a transacted operation, which results in a handle that can be
    /// used for transacted I/O, use the [`CreateFileTransacted`] function.
    ///
    /// # Parameters
    ///  * `file_name` - The name of the file or device to be created or opened. You may use either
    ///                  forward slashes (/) or backslashes (\) in this name. To create a file
    ///                  stream, specify the name of the file, a colon, and then the name of the
    ///                  stream. By default, the name is limited to [`MAX_PATH`] characters. To
    ///                  extend this limit to 32,767 wide characters, prepend "\\?\" to the path.
    ///  * `desired_access` - The requested access to the file or device, which can be summarized
    ///                       as read, write, both or neither zero). The most commonly used values
    ///                       are [`GENERIC_READ`], [`GENERIC_WRITE`], or both
    ///                       `(GENERIC_READ | GENERIC_WRITE)`. If this parameter is zero, the
    ///                       application can query certain metadata such as file, directory, or
    ///                       device attributes without accessing that file or device, even if
    ///                       [`GENERIC_READ`] access would have been denied. You cannot request an
    ///                       access mode that conflicts with the sharing mode that is specified by
    ///                       the `share_mode` parameter in an open request that already has an
    ///                       open handle.
    ///  * `share_mode` - The requested sharing mode of the file or device, which can be read,
    ///                   write, both, delete, all of these, or none (refer to the following
    ///                   table). Access requests to attributes or extended attributes are not
    ///                   affected by this flag. If this parameter is zero and [`CreateFile`]
    ///                   succeeds, the file or device cannot be shared and cannot be opened again
    ///                   until the handle to the file or device is closed. You cannot request a
    ///                   sharing mode that conflicts with the access mode that is specified in an
    ///                   existing request that has an open handle. [`CreateFile`] would fail and
    ///                   the [`GetLastError`] function would return [`ERROR_SHARING_VIOLATION`].
    ///                   To enable a process to share a file or device while another process has
    ///                   the file or device open, use a compatible combination of one or more of
    ///                   the following values.
    ///    * `0` - Prevents subsequent open operations on a file or device if they request delete,
    ///            read, or write access.
    ///    * [`FILE_SHARE_DELETE`] - Enables subsequent open operations on a file or device to
    ///                              request delete access. Otherwise, no process can open the file
    ///                              or device if it requests delete access. If this flag is not
    ///                              specified, but the file or device has been opened for delete
    ///                              access, the function fails.
    ///    * [`FILE_SHARE_READ`] - Enables subsequent open operations on a file or device to
    ///                            request read access. Otherwise, no process can open the file or
    ///                            device if it requests read access. If this flag is not
    ///                            specified, but the file or device has been opened for read
    ///                            access, the function fails.
    ///    * [`FILE_SHARE_WRITE`] - Enables subsequent open operations on a file or device to
    ///                             request write access. Otherwise, no process can open the file
    ///                             or device if it requests write access. If this flag is not
    ///                             specified, but the file or device has been opened for write
    ///                             access or has a file mapping with write access, the function
    ///                             fails.
    ///  * `security_attributes` - A pointer to a [`SECURITY_ATTRIBUTES`] structure that contains
    ///                            two separate but related data members: an optional security
    ///                            descriptor, and a Boolean value that determines whether the
    ///                            returned handle can be inherited by child processes. This
    ///                            parameter can be [`null_mut`]. If this parameter is
    ///                            [`null_mut`], the handle returned by [`CreateFile`] cannot be
    ///                            inherited by any child processes the application may create and
    ///                            the file or device associated with the returned handle gets a
    ///                            default security descriptor. The `security_descriptor` member of
    ///                            the structure specifies a [`SECURITY_DESCRIPTOR`] for a file or
    ///                            device. If this member is [`null_mut`], the file or device
    ///                            associated with the returned handle is assigned a default
    ///                            security descriptor. [`CreateFile`] ignores the
    ///                            `security_descriptor` member when opening an existing file or
    ///                            device, but continues to use the `inherit_handle` member. The
    ///                            `inherit_handle` member of the structure specifies whether the
    ///                            returned handle can be inherited.
    ///  * `creation_disposition` - An action to take on a file or device that exists or does not
    ///                             exist. For devices other than files, this parameter is usually
    ///                             set to [`OPEN_EXISTING`]. This parameter must be one of the
    ///                             following values, which cannot be combined:
    ///    * [`CREATE_ALWAYS`] - Creates a new file, always. If the specified file exists and is
    ///                          writable, the function truncates the file, the function succeeds,
    ///                          and last-error code is set to [`ERROR_ALREADY_EXISTS`] (183). If
    ///                          the specified file does not exist and is a valid path, a new file
    ///                          is created, the function succeeds, and the last-error code is set
    ///                          to zero.
    ///    * [`CREATE_NEW`] - 	Creates a new file, only if it does not already exist. If the
    ///                         specified file exists, the function fails and the last-error code
    ///                         is set to [`ERROR_FILE_EXISTS`] (80). If the specified file does
    ///                         not exist and is a valid path to a writable location, a new file is
    ///                         created.
    ///    * [`OPEN_ALWAYS`] - Opens a file, always. If the specified file exists, the function
    ///                        succeeds and the last-error code is set to [`ERROR_ALREADY_EXISTS`]
    ///                        (183). If the specified file does not exist and is a valid path to
    ///                        a writable location, the function creates a file and the last-error
    ///                        code is set to zero.
    ///    * [`OPEN_EXISTING`] - Opens a file or device, only if it exists. If the specified file
    ///                          or device does not exist, the function fails and the last-error
    ///                          code is set to [`ERROR_FILE_NOT_FOUND`] (2).
    ///    * [`TRUNCATE_EXISTING`] - Opens a file and truncates it so that its size is zero bytes,
    ///                              only if it exists. If the specified file does not exist, the
    ///                              function fails and the last-error code is set to
    ///                              [`ERROR_FILE_NOT_FOUND`] (2). The calling process must open
    ///                              the file with the [`GENERIC_WRITE`] bit set as part of the
    ///                              `desired_access` parameter.
    ///  * `flags_and_attributes` - The file or device attributes and flags,
    ///                             [`FILE_ATTRIBUTE_NORMAL`] being the most common default value
    ///                             for files. This parameter can include any combination of the
    ///                             available file attributes (`FILE_ATTRIBUTE_*`). All other file
    ///                             attributes override [`FILE_ATTRIBUTE_NORMAL`]. This parameter
    ///                             can also contain combinations of flags (`FILE_FLAG_`) for
    ///                             control of file or device caching behavior, access modes, and
    ///                             other special-purpose flags. These combine with any
    ///                             `FILE_ATTRIBUTE_` values. This parameter can also contain
    ///                             Security Quality of Service (SQOS) information by specifying
    ///                             the [`SECURITY_SQOS_PRESENT`] flag. Additional SQOS-related
    ///                             flags information is presented in the table following the
    ///                             attributes and flags tables. Some of the following file
    ///                             attributes and flags may only apply to files and not
    ///                             necessarily all other types of devices that [`CreateFile`] can
    ///                             open. The `flags_and_attributes` parameter can also specify
    ///                             SQOS information. When the calling application specifies the
    ///                             [`SECURITY_SQOS_PRESENT`] flag as part of
    ///                             `flags_and_attributes`, it can also contain one or more of the
    ///                             following values.
    ///    * [`FILE_ATTRIBUTE_ARCHIVE`] - The file should be archived. Applications use this
    ///                                   attribute to mark files for backup or removal.
    ///    * [`FILE_ATTRIBUTE_ENCRYPTED`] - The file or directory is encrypted. For a file, this
    ///                                     means that all data in the file is encrypted. For a
    ///                                     directory, this means that encryption is the default
    ///                                     for newly created files and subdirectories. This flag
    ///                                     has no effect if [`FILE_ATTRIBUTE_SYSTEM`] is also
    ///                                     specified. This flag is not supported on Home, Home
    ///                                     Premium, Starter, or ARM editions of Windows.
    ///    * [`FILE_ATTRIBUTE_HIDDEN`] - The file is hidden. Do not include it in an ordinary
    ///                                  directory listing.
    ///    * [`FILE_ATTRIBUTE_NORMAL`] - The file does not have other attributes set. This
    ///                                  attribute is valid only if used alone.
    ///    * [`FILE_ATTRIBUTE_OFFLINE`] - The data of a file is not immediately available. This
    ///                                   attribute indicates that file data is physically moved to
    ///                                   offline storage. This attribute is used by Remote
    ///                                   Storage, the hierarchical storage management software.
    ///                                   Applications should not arbitrarily change this
    ///                                   attribute.
    ///    * [`FILE_ATTRIBUTE_READONLY`] - The file is read only. Applications can read the file,
    ///                                    but cannot write to or delete it.
    ///    * [`FILE_ATTRIBUTE_SYSTEM`] - The file is part of or used exclusively by an operating
    ///                                  system.
    ///    * [`FILE_ATTRIBUTE_TEMPORARY`] - The file is being used for temporary storage.
    ///    * [`FILE_FLAG_BACKUP_SEMANTICS`] - The file is being opened or created for a backup or
    ///                                       restore operation. The system ensures that the
    ///                                       calling process overrides file security checks when
    ///                                       the process has [`SE_BACKUP_NAME`] and
    ///                                       [`SE_RESTORE_NAME`] privileges. You must set this
    ///                                       flag to obtain a handle to a directory. A directory
    ///                                       handle can be passed to some functions instead of a
    ///                                       file handle.
    ///    * [`FILE_FLAG_DELETE_ON_CLOSE`] - The file is to be deleted immediately after all of its
    ///                                      handles are closed, which includes the specified
    ///                                      handle and any other open or duplicated handles. If
    ///                                      there are existing open handles to a file, the call
    ///                                      fails unless they were all opened with the
    ///                                      [`FILE_SHARE_DELETE`] share mode. Subsequent open
    ///                                      requests for the file fail, unless the
    ///                                      [`FILE_SHARE_DELETE`] share mode is specified.
    ///    * [`FILE_FLAG_NO_BUFFERING`] - The file or device is being opened with no system caching
    ///                                   for data reads and writes. This flag does not affect hard
    ///                                   disk caching or memory mapped files. There are strict
    ///                                   requirements for successfully working with files opened
    ///                                   with [`CreateFile`] using the [`FILE_FLAG_NO_BUFFERING`]
    ///                                   flag.
    ///    * [`FILE_FLAG_OPEN_NO_RECALL`] - The file data is requested, but it should continue to
    ///                                     be located in remote storage. It should not be
    ///                                     transported back to local storage. This flag is for use
    ///                                     by remote storage systems.
    ///    * [`FILE_FLAG_OPEN_REPARSE_POINT`] - Normal reparse point processing will not occur;
    ///                                         [`CreateFile`] will attempt to open the reparse
    ///                                         point. When a file is opened, a file handle is
    ///                                         returned, whether or not the filter that controls
    ///                                         the reparse point is operational. This flag cannot
    ///                                         be used with the [`CREATE_ALWAYS`] flag. If the
    ///                                         file is not a reparse point, then this flag is
    ///                                         ignored.
    ///    * [`FILE_FLAG_OVERLAPPED`] - The file or device is being opened or created for
    ///                                 asynchronous I/O. When subsequent I/O operations are
    ///                                 completed on this handle, the event specified in the
    ///                                 [`OVERLAPPED`] structure will be set to the signaled state.
    ///                                 If this flag is specified, the file can be used for
    ///                                 simultaneous read and write operations. If this flag is not
    ///                                 specified, then I/O operations are serialized, even if the
    ///                                 calls to the read and write functions specify an
    ///                                 [`OVERLAPPED`] structure.
    ///    * [`FILE_FLAG_POSIX_SEMANTICS`] - Access will occur according to POSIX rules. This
    ///                                      includes allowing multiple files with names, differing
    ///                                      only in case, for file systems that support that
    ///                                      naming. Use care when using this option, because files
    ///                                      created with this flag may not be accessible by
    ///                                      applications that are written for MS-DOS or 16-bit
    ///                                      Windows.
    ///    * [`FILE_FLAG_RANDOM_ACCESS`] - Access is intended to be random. The system can use this
    ///                                    as a hint to optimize file caching. This flag has no
    ///                                    effect if the file system does not support cached I/O
    ///                                    and [`FILE_FLAG_NO_BUFFERING`].
    ///    * [`FILE_FLAG_SESSION_AWARE`] - The file or device is being opened with session
    ///                                    awareness. If this flag is not specified, then per-
    ///                                    session devices (such as a device using RemoteFX USB
    ///                                    Redirection) cannot be opened by processes running in
    ///                                    session 0. This flag has no effect for callers not in
    ///                                    session 0. This flag is supported only on server
    ///                                    editions of Windows.
    ///    * [`FILE_FLAG_SEQUENTIAL_SCAN`] - Access is intended to be sequential from beginning to
    ///                                      end. The system can use this as a hint to optimize
    ///                                      file caching. This flag should not be used if read-
    ///                                      behind (that is, reverse scans) will be used. This
    ///                                      flag has no effect if the file system does not support
    ///                                      cached I/O and [`FILE_FLAG_NO_BUFFERING`].
    ///    * [`FILE_FLAG_WRITE_THROUGH`] - Write operations will not go through any intermediate
    ///                                    cache, they will go directly to disk.
    ///    * [`SECURITY_ANONYMOUS`] - Impersonates a client at the Anonymous impersonation level.
    ///    * [`SECURITY_CONTEXT_TRACKING`] - The security tracking mode is dynamic. If this flag is
    ///                                      not specified, the security tracking mode is static.
    ///    * [`SECURITY_DELEGATION`] - Impersonates a client at the Delegation impersonation level.
    ///    * [`SECURITY_EFFECTIVE_ONLY`] - Only the enabled aspects of the client's security context
    ///                                    are available to the server. If you do not specify this
    ///                                    flag, all aspects of the client's security context are
    ///                                    available. This allows the client to limit the groups
    ///                                    and privileges that a server can use while impersonating
    ///                                    the client.
    ///    * [`SECURITY_IDENTIFICATION`] - Impersonates a client at the Identification
    ///                                    impersonation level.
    ///    * [`SECURITY_IMPERSONATION`] - Impersonate a client at the impersonation level. This is
    ///                                   the default behavior if no other flags are specified
    ///                                   along with the [`SECURITY_SQOS_PRESENT`] flag.
    ///  * `template_file` - A valid handle to a template file with the [`GENERIC_READ`] access
    ///                      right. The template file supplies file attributes and extended
    ///                      attributes for the file that is being created. This parameter can be
    ///                      [`null_mut`]. When opening an existing file, [`CreateFile`] ignores
    ///                      this parameter. When opening a new encrypted file, the file inherits
    ///                      the discretionary access control list from its parent directory.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is an open handle to the specified file, device,
    /// named pipe, or mail slot.
    ///
    /// If the function fails, the return value is [`INVALID_HANDLE_VALUE`]. To get extended error
    /// information, call [`GetLastError`].
    pub fn CreateFileW(
        file_name: LPCWSTR,
        desired_access: DWORD,
        share_mode: DWORD,
        security_attributes: LPSECURITY_ATTRIBUTES,
        creation_disposition: DWORD,
        flags_and_attributes: DWORD,
        template_file: HANDLE,
    ) -> HANDLE;
}
