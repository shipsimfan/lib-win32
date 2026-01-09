use crate::{BOOL, DWORD, HANDLE, LPVOID, PDWORD, TOKEN_INFORMATION_CLASS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, TOKEN_QUERY, TOKEN_QUERY_SOURCE};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Advapi32")]
extern "system" {
    /// The [`GetTokenInformation`] function retrieves a specified type of information about an
    /// access token. The calling process must have appropriate access rights to obtain the
    /// information.
    ///
    /// To determine if a user is a member of a specific group, use the [`CheckTokenMembership`]
    /// function. To determine group membership for app container tokens, use the
    /// [`CheckTokenMembershipEx`] function.
    ///
    /// # Parameters
    ///  * `token_handle` - A handle to an access token from which information is retrieved. If
    ///                     `token_information_class` specifies
    ///                     [`TOKEN_INFORMATION_CLASS::TokenSource`], the handle must have
    ///                     [`TOKEN_QUERY_SOURCE`] access. For all other `token_information_class`
    ///                     values, the handle must have [`TOKEN_QUERY`] access.
    ///  * `token_information_class` - Specifies a value from the [`TOKEN_INFORMATION_CLASS`]
    ///                                enumerated type to identify the type of information the
    ///                                function retrieves. Any callers who check the
    ///                                [`TOKEN_INFORMATION_CLASS::TokenIsAppContainer`] and have it
    ///                                return 0 should also verify that the caller token is not an
    ///                                identify level impersonation token. If the current token is
    ///                                not an app container but is an identity level token, you
    ///                                should return [`AccessDenied`].
    ///  * `token_information` - A pointer to a buffer the function fills with the requested
    ///                          information. The structure put into this buffer depends upon the
    ///                          type of information specified by the `token_information_class`
    ///                          parameter.
    ///  * `token_information_length` - Specifies the size, in bytes, of the buffer pointed to by
    ///                                 the `token_information` parameter. If `token_information`
    ///                                 is [`null_mut`], this parameter must be zero.
    ///  * `return_length` - A pointer to a variable that receives the number of bytes needed for
    ///                      the buffer pointed to by the `token_information` parameter. If this
    ///                      value is larger than the value specified in the
    ///                      `token_information_length` parameter, the function fails and stores no
    ///                      data in the buffer. If the value of the `token_information_class`
    ///                      parameter is [`TOKEN_INFORMATION_CLASS::TokenDefaultDacl`] and the
    ///                      token has no default DACL, the function sets the variable pointed to
    ///                      by `return_length` to `std::mem::size_of::<TOKEN_DEFAULT_DACL>()` and
    ///                      sets the `default_dacl` member of the [`TOKEN_DEFAULT_DACL`] structure
    ///                      to [`null_mut`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    pub fn GetTokenInformation(
        token_handle: HANDLE,
        token_information_class: TOKEN_INFORMATION_CLASS,
        token_information: LPVOID,
        token_information_length: DWORD,
        return_length: PDWORD,
    ) -> BOOL;
}
