// rustdoc imports
#[allow(unused_imports)]
use crate::shlobj_core::SHGetKnownFolderPath;

/// Defines constants that specify special retrieval options for known folders (for example, for
/// use when calling the [`SHGetKnownFolderIDList`] function to retrieve the path of a known
/// folder). These values supersede [`CSIDL`] values, which have parallel meanings.
///
/// # Remarks
/// These values, with the exception of [`KNOWN_FOLDER_FLAG::AliasOnly`], were defined in Windows
/// Vista as individual constants. They're defined as an enumeration only in Windows 7 and later.
/// However, all underlying numerical values are the same in either form.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum KNOWN_FOLDER_FLAG {
    /// Specifies no special retrieval options.
    Default = 0x00000000,

    /// Introduced in Windows 10, version 1709. When called from a packaged app, specifies that LocalAppData/RoamingAppData folders are redirected to private app locations that match the paths returned from Windows.Storage.ApplicationData.Current in the LocalFolder and RoamingFolder properties. Other folders are redirected to subdirectories of LocalAppData.
    ///
    /// This flag is used with FOLDERID_AppDataDesktop, FOLDERID_AppDataDocuments, FOLDERID_AppDataFavorites, and FOLDERID_AppDataProgramData. It's also intended for compatibility with .NET applications, and not meant to be used directly from an application.
    ForceAppDataRedirection = 0x00080000,

    /// Introduced in Windows 10, version 1703. When running in a packaged process, specifies that some file system locations are redirected to package-specific locations by the file system. This flag causes the target of the direction to be returned for those locations. This is useful in cases where the real location within the file system needs to be known.
    ReturnFilterRedirectionTarget = 0x00040000,

    /// Introduced in Windows 10, version 1703. When running inside an AppContainer process, or when providing an AppContainer token, specifies that some folders are redirected to AppContainer-specific locations within the package. This flag forces redirection (for folders that aren't normally redirected) for the purposes of packaged processes, and can be used for sharing files between UWP and packaged apps within the same package. This flag supersedes the deprecated KF_FLAG_FORCE_APPCONTAINER_REDIRECTION.
    ForcePackageRedirection = 0x00020000,

    /// Introduced in Windows 10, version 1703. When running inside a packaged process, or when providing a packaged process token, specifies that some folders are redirected to package-specific locations. This flag disables redirection on locations where it's applied, and instead returns the path that would be returned were it not running inside a packaged process. This flag supersedes the deprecated KF_FLAG_NO_APPCONTAINER_REDIRECTION.
    NoPackageRedirection = 0x00010000,

    /// Specifies to force the creation of the specified folder if that folder doesn't already exist. The security provisions predefined for that folder are applied. If the folder doesn't exist and can't be created, then the function returns a failure code, and no path is returned. This value can be used only with the following functions and methods:
    ///  * [`SHGetKnownFolderPath`]
    ///  * [`SHGetKnownFolderIDList`]
    ///  * [`IKnownFolder::get_id_list`]
    ///  * [`IKnownFolder::get_path`]
    ///  * [`IKnownFolder::get_shell_item`]
    Create = 0x00008000,

    /// Specifies not to verify the folder's existence before attempting to retrieve the path or IDList. If this flag isn't set, then an attempt is made to verify that the folder is truly present at the path. If that verification fails due to the folder being absent or inaccessible, then the function returns a failure code, and no path is returned.
    ///
    /// If the folder is located on a network, then the function might take longer to execute. So setting this flag can reduce that latency.
    DontVerify = 0x00004000,

    /// Specfies to store the full path in the registry without using environment strings. If this flag isn't set, then portions of the path might be represented by environment strings such as %USERPROFILE%. This flag can be used only with SHSetKnownFolderPath and IKnownFolder::SetPath.
    DontUnexpand = 0x00002000,

    /// Specifies to retrieve the true system path for the folder, free of any aliased placeholders such as %USERPROFILE%, returned by SHGetKnownFolderIDList and IKnownFolder::GetIDList. This flag has no effect on paths returned by SHGetKnownFolderPath and IKnownFolder::GetPath. By default, known folder retrieval functions and methods return the aliased path if an alias exists.
    NoAlias = 0x00001000,

    /// Specifies to initialize the folder using its Desktop.ini settings. If the folder can't be initialized, then the function returns a failure code, and no path is returned. This flag should always be combined with KF_FLAG_CREATE.
    ///
    /// If the folder is located on a network, then the function might take longer to execute.
    Init = 0x00000800,

    /// Specifies to retrieve the default path for a known folder. If this flag isn't set, then the function retrieves the current—and possibly redirected—path of the folder. The execution of this flag includes a verification of the folder's existence unless KF_FLAG_DONT_VERIFY is set.
    DefaultPath = 0x00000400,

    /// Specifies to retrieve the folder's default path independent of the current location of its parent. KF_FLAG_DEFAULT_PATH must also be set.
    NotParentRelative = 0x00000200,

    /// Specifies to build a simple IDList (PIDL). This value can be used when you want to retrieve the file system path. But don't specify this value if you're retrieving the localized display name of the folder, because it might not resolve correctly.
    SimpleIdlist = 0x00000100,

    /// Introduced in Windows 7. Specifies to retrieve only aliased PIDLs. Don't use the file system path.
    AliasOnly = 0x80000000,
}
