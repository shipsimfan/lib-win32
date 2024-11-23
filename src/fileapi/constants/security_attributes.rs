use crate::DWORD;

/// Impersonates a client at the Anonymous impersonation level.
pub const SECURITY_ANONYMOUS: DWORD = 0;

/// The security tracking mode is dynamic. If this flag is not specified, the security tracking mode is static.
pub const SECURITY_CONTEXT_TRACKING: DWORD = 0x00040000;

/// Impersonates a client at the Delegation impersonation level.
pub const SECURITY_DELEGATION: DWORD = 3 << 16;

/// Only the enabled aspects of the client's security context are available to the server. If you do not specify this flag, all aspects of the client's security context are available.
///
/// This allows the client to limit the groups and privileges that a server can use while impersonating the client.
pub const SECURITY_EFFECTIVE_ONLY: DWORD = 0x00080000;

/// Impersonates a client at the Identification impersonation level.
pub const SECURITY_IDENTIFICATION: DWORD = 1 << 16;

/// Impersonate a client at the impersonation level. This is the default behavior if no other flags are specified along with the SECURITY_SQOS_PRESENT flag.
pub const SECURITY_IMPERSONATION: DWORD = 2 << 16;

#[allow(missing_docs)]
pub const SECURITY_SQOS_PRESENT: DWORD = 0x00100000;

#[allow(missing_docs)]
pub const SECURITY_VALID_SQOS_FLAGS: DWORD = 0x001F0000;
