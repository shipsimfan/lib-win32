use crate::DWORD;

/// Use the standard logon provider for the system. The default security provider is NTLM.
pub const LOGON32_PROVIDER_DEFAULT: DWORD = 0;

/// Use the negotiate logon provider.
pub const LOGON32_PROVIDER_WINNT40: DWORD = 2;

/// Use the NTLM logon provider.
pub const LOGON32_PROVIDER_WINNT50: DWORD = 3;
