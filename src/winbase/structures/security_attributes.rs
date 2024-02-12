use crate::{BOOL, DWORD, LPVOID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{RegCreateKeyEx, RegSaveKeyEx, SECURITY_DESCRIPTOR, TRUE};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// The [`SECURITY_ATTRIBUTES`] structure contains the security descriptor for an object and
/// specifies whether the handle retrieved by specifying this structure is inheritable. This
/// structure provides security settings for objects created by various functions, such as
/// [`CreateFile`], [`CreatePipe`], [`CreateProcess`], [`RegCreateKeyEx`], or [`RegSaveKeyEx`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SECURITY_ATTRIBUTES {
    /// The size, in bytes, of this structure. Set this value to the size of the
    /// [`SECURITY_ATTRIBUTES`] structure.
    pub length: DWORD,

    /// A pointer to a [`SECURITY_DESCRIPTOR`] structure that controls access to the object. If the
    /// value of this member is [`null_mut`], the object is assigned the default security
    /// descriptor associated with the access token of the calling process. This is not the same as
    /// granting access to everyone by assigning a [`null_mut`] discretionary access control list
    /// (DACL). By default, the default DACL in the access token of a process allows access only to
    /// the user represented by the access token.
    pub security_descriptor: LPVOID,

    /// A boolean value that specifies whether the returned handle is inherited when a new process
    /// is created. If this member is [`TRUE`], the new process inherits the handle.
    pub inherit_handle: BOOL,
}
