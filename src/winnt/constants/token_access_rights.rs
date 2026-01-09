use crate::{DWORD, STANDARD_RIGHTS_READ, STANDARD_RIGHTS_REQUIRED, STANDARD_RIGHTS_WRITE};

/// Required to attach a primary token to a process. The [`SE_ASSIGNPRIMARYTOKEN_NAME`] privilege
/// is also required to accomplish this task.
pub const TOKEN_ASSIGN_PRIMARY: DWORD = 0x0001;

/// Required to duplicate an access token.
pub const TOKEN_DUPLICATE: DWORD = 0x0002;

/// Required to attach an impersonation access token to a process.
pub const TOKEN_IMPERSONATE: DWORD = 0x0004;

/// Required to query an access token.
pub const TOKEN_QUERY: DWORD = 0x0008;

/// Required to query the source of an access token.
pub const TOKEN_QUERY_SOURCE: DWORD = 0x0010;

/// Required to enable or disable the privileges in an access token.
pub const TOKEN_ADJUST_PRIVILEGES: DWORD = 0x0020;

/// Required to adjust the attributes of the groups in an access token.
pub const TOKEN_ADJUST_GROUPS: DWORD = 0x0040;

/// Required to change the default owner, primary group, or DACL of an access token.
pub const TOKEN_ADJUST_DEFAULT: DWORD = 0x0080;

/// Required to adjust the session ID of an access token. The [`SE_TCB_NAME`] privilege is
/// required.
pub const TOKEN_ADJUST_SESSIONID: DWORD = 0x0100;

/// Combines [`STANDARD_RIGHTS_READ`] and [`TOKEN_QUERY`].
pub const TOKEN_READ: DWORD = STANDARD_RIGHTS_READ | TOKEN_QUERY;

/// Combines [`STANDARD_RIGHTS_WRITE`], [`TOKEN_ADJUST_PRIVILEGES`], [`TOKEN_ADJUST_GROUPS`], and
/// [`TOKEN_ADJUST_DEFAULT`].
pub const TOKEN_WRITE: DWORD =
    STANDARD_RIGHTS_WRITE | TOKEN_ADJUST_PRIVILEGES | TOKEN_ADJUST_GROUPS | TOKEN_ADJUST_DEFAULT;

/// Combines all possible access rights for a token.
pub const TOKEN_ALL_ACCESS_P: DWORD = STANDARD_RIGHTS_REQUIRED
    | TOKEN_ASSIGN_PRIMARY
    | TOKEN_DUPLICATE
    | TOKEN_IMPERSONATE
    | TOKEN_QUERY
    | TOKEN_QUERY_SOURCE
    | TOKEN_ADJUST_PRIVILEGES
    | TOKEN_ADJUST_GROUPS
    | TOKEN_ADJUST_DEFAULT;
