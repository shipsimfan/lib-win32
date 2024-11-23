use crate::DWORD;

/// The file should be archived. Applications use this attribute to mark files for backup or
/// removal.
pub const FILE_ATTRIBUTE_ARCHIVE: DWORD = 0x20;

/// The file or directory is encrypted. For a file, this means that all data in the file is
/// encrypted. For a directory, this means that encryption is the default for newly created files
/// and subdirectories.
///
/// This flag has no effect if [`FILE_ATTRIBUTE_SYSTEM`] is also specified.
///
/// This flag is not supported on Home, Home Premium, Starter, or ARM editions of Windows.
pub const FILE_ATTRIBUTE_ENCRYPTED: DWORD = 0x4000;

/// The file is hidden. Do not include it in an ordinary directory listing.
pub const FILE_ATTRIBUTE_HIDDEN: DWORD = 0x2;

/// The file does not have other attributes set. This attribute is valid only if used alone.
pub const FILE_ATTRIBUTE_NORMAL: DWORD = 0x80;

/// The data of a file is not immediately available. This attribute indicates that file data is
/// physically moved to offline storage. This attribute is used by Remote Storage, the hierarchical
/// storage management software. Applications should not arbitrarily change this attribute.
pub const FILE_ATTRIBUTE_OFFLINE: DWORD = 0x1000;

/// The file is read only. Applications can read the file, but cannot write to or delete it.
pub const FILE_ATTRIBUTE_READONLY: DWORD = 0x1;

/// The file is part of or used exclusively by an operating system.
pub const FILE_ATTRIBUTE_SYSTEM: DWORD = 0x4;

/// The file is being used for temporary storage.
pub const FILE_ATTRIBUTE_TEMPORARY: DWORD = 0x100;
