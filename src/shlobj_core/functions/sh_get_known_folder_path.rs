use crate::{DWORD, HANDLE, HRESULT, PWSTR, shtypes::REFKNOWNFOLDERID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CoTaskMemFree, E_FAIL, E_INVALIDARG, S_OK, TOKEN_DUPLICATE, TOKEN_IMPERSONATE, TOKEN_QUERY,
    shlobj_core::{KF_CATEGORY, KNOWN_FOLDER_FLAG},
    shtypes::KNOWNFOLDERID,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Shell32")]
unsafe extern "system" {
    /// Retrieves the full path of a known folder identified by the folder's [`KNOWNFOLDERID`].
    ///
    /// # Parameters
    ///  * `id` - A reference to the [`KNOWNFOLDERID`] that identifies the folder.
    ///  * `flags` - Flags that specify special retrieval options. This value can be 0; otherwise,
    ///              one or more of the [`KNOWN_FOLDER_FLAG`] values.
    ///  * `token` - An access token that represents a particular user. If this parameter is
    ///              [`null_mut`], which is the most common usage, the function requests the known
    ///              folder for the current user. Request a specific user's folder by passing the
    ///              `token` of that user. This is typically done in the context of a service that
    ///              has sufficient privileges to retrieve the token of a given user. That token
    ///              must be opened with [`TOKEN_QUERY`] and [`TOKEN_IMPERSONATE`] rights. In some
    ///              cases, you also need to include [`TOKEN_DUPLICATE`]. In addition to passing
    ///              the user's `token`, the registry hive of that specific user must be mounted.
    ///              Assigning the `token` parameter a value of -1 indicates the Default User. This
    ///              allows clients of [`SHGetKnownFolderPath`] to find folder locations (such as
    ///              the Desktop folder) for the Default User. The Default User user profile is
    ///              duplicated when any new user account is created, and includes special folders
    ///              such as Documents and Desktop. Any items added to the Default User folder also
    ///              appear in any new user account. Note that access to the Default User folders
    ///              requires administrator privileges.
    ///  * `path` - When this method returns, contains the address of a pointer to a
    ///             null-terminated Unicode string that specifies the path of the known folder. The
    ///             calling process is responsible for freeing this resource once it is no longer
    ///             needed by calling [`CoTaskMemFree`], whether [`SHGetKnownFolderPath`] succeeds
    ///             or not. The returned path does not include a trailing backslash. For example,
    ///             "C:\Users" is returned rather than "C:\Users\".
    ///
    /// # Return Value
    /// Returns [`S_OK`] if successful, or an error value otherwise, including the following:
    ///  * [`E_FAIL`] -  Among other things, this value can indicate that the `id` parameter
    ///                  references a [`KNOWNFOLDERID`] which does not have a path (such as a
    ///                  folder marked as [`KF_CATEGORY::Virtual`]).
    ///  * [`E_INVALIDARG`] -  Among other things, this value can indicate that the `id` parameter
    ///                        references a [`KNOWNFOLDERID`] that is not present on the system.
    ///                        Not all [`KNOWNFOLDERID`] values are present on all systems. Use
    ///                        [`IKnownFolderManager::get_folder_ids`] to retrieve the set of
    ///                        [`KNOWNFOLDERID`] values for the current system.
    ///
    /// # Remarks
    /// This function replaces [`SHGetFolderPath`]. That older function is now simply a wrapper for
    /// [`SHGetKnownFolderPath`].
    pub fn SHGetKnownFolderPath(
        id: REFKNOWNFOLDERID,
        flags: DWORD,
        token: HANDLE,
        path: *mut PWSTR,
    ) -> HRESULT;
}
