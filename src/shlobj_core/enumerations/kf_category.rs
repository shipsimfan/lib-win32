/// Value that represent a category by which a folder registered with the Known Folder system can
/// be classified.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum KF_CATEGORY {
    /// Virtual folders are not part of the file system, which is to say that they have no path.
    /// For example, Control Panel and Printers are virtual folders. A number of features such as
    /// folder path and redirection do not apply to this category.
    Virtual = 1,

    /// Fixed file system folders are not managed by the Shell and are usually given a permanent
    /// path when the system is installed. For example, the Windows and Program Files folders are
    /// fixed folders. A number of features such as redirection do not apply to this category.
    Fixed = 2,

    /// Common folders are those file system folders used for sharing data and settings, accessible
    /// by all users of a system. For example, all users share a common Documents folder as well as
    /// their per-user Documents folder.
    Common = 3,

    /// Per-user folders are those stored under each user's profile and accessible only by that
    /// user. For example, %USERPROFILE%\Pictures. This category of folder usually supports many
    /// features including aliasing, redirection and customization.
    PerUser = 4,
}
