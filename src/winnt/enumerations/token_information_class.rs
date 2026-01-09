// rustdoc imports
#[allow(unused_imports)]
use crate::{GetTokenInformation, DWORD, TOKEN_ELEVATION};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// The [`TOKEN_INFORMATION_CLASS`] enumeration contains values that specify the type of
/// information being assigned to or retrieved from an access token.
///
/// The [`GetTokenInformation`] function uses these values to indicate the type of token
/// information to retrieve.
///
/// The [`SetTokenInformation`] function uses these values to set the token information.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum TOKEN_INFORMATION_CLASS {
    /// The buffer receives a [`TOKEN_USER`] structure that contains the user account of the token.
    TokenUser = 1,

    /// The buffer receives a [`TOKEN_GROUPS`] structure that contains the group accounts
    /// associated with the token.
    TokenGroups,

    /// The buffer receives a [`TOKEN_PRIVILEGES`] structure that contains the privileges of the
    /// token.
    TokenPrivileges,

    /// The buffer receives a [`TOKEN_OWNER`] structure that contains the default owner security
    /// identifier (SID) for newly created objects.
    TokenOwner,

    /// The buffer receives a [`TOKEN_PRIMARY_GROUP`] structure that contains the default primary
    /// group SID for newly created objects.
    TokenPrimaryGroup,

    /// The buffer receives a [`TOKEN_DEFAULT_DACL`] structure that contains the default DACL for
    /// newly created objects.
    TokenDefaultDacl,

    /// The buffer receives a [`TOKEN_SOURCE`] structure that contains the source of the token.
    /// [`TOKEN_QUERY_SOURCE`] access is needed to retrieve this information.
    TokenSource,

    /// The buffer receives a [`TOKEN_TYPE`] value that indicates whether the token is a primary or
    /// impersonation token.
    TokenType,

    /// The buffer receives a [`SECURITY_IMPERSONATION_LEVEL`] value that indicates the
    /// impersonation level of the token. If the access token is not an impersonation token, the
    /// function fails.
    TokenImpersonationLevel,

    /// The buffer receives a [`TOKEN_STATISTICS`] structure that contains various token
    /// statistics.
    TokenStatistics,

    /// The buffer receives a restricted token.
    TokenRestrictedSids,

    /// The buffer receives a [`DWORD`] value that indicates the Terminal Services session
    /// identifier that is associated with the token.
    ///
    /// If [`TOKEN_INFORMATION_CLASS::TokenSessionId`] is set with [`SetTokenInformation`], the
    /// application must have the Act As Part Of the Operating System privilege, and the
    /// application must be enabled to set the session ID in a token.
    TokenSessionId,

    /// The buffer receives a [`TOKEN_GROUPS_AND_PRIVILEGES`] structure that contains the user SID,
    /// the group accounts, the restricted SIDs, and the authentication ID associated with the
    /// token.
    TokenGroupsAndPrivileges,

    /// Reserved.
    TokenSessionReference,

    /// The buffer receives a [`DWORD`] value that is nonzero if the token includes the
    /// [`SANDBOX_INERT`] flag.
    TokenSandBoxInert,

    /// Reserved.
    TokenAuditPolicy,

    /// The buffer receives a [`TOKEN_ORIGIN`] value.
    ///
    /// If the token resulted from network authentication, such as a call to
    /// [`AcceptSecurityContext`] or a call to [`LogonUser`] with `logon_type` set to
    /// [`LOGON32_LOGON_NETWORK`] or [`LOGON32_LOGON_NETWORK_CLEARTEXT`], then this value will be
    /// zero.
    TokenOrigin,

    /// The buffer receives a [`TOKEN_ELEVATION_TYPE`] value that specifies the elevation level of
    /// the token.
    TokenElevationType,

    /// The buffer receives a [`TOKEN_LINKED_TOKEN`] structure that contains a handle to another
    /// token that is linked to this token.
    TokenLinkedToken,

    /// The buffer receives a [`TOKEN_ELEVATION`] structure that specifies whether the token is
    /// elevated.
    TokenElevation,

    /// The buffer receives a [`DWORD`] value that is nonzero if the token has ever been filtered.
    TokenHasRestrictions,

    /// The buffer receives a [`TOKEN_ACCESS_INFORMATION`] structure that specifies security
    /// information contained in the token.
    TokenAccessInformation,

    /// The buffer receives a [`DWORD`] value that is nonzero if virtualization is allowed for the
    /// token.
    TokenVirtualizationAllowed,

    /// The buffer receives a [`DWORD`] value that is nonzero if virtualization is enabled for the
    /// token.
    TokenVirtualizationEnabled,

    /// The buffer receives a [`TOKEN_MANDATORY_LABEL`] structure that specifies the token's
    /// integrity level.
    TokenIntegrityLevel,

    /// The buffer receives a [`DWORD`] value that is nonzero if the token has the [`UIAccess`]
    /// flag set.
    TokenUIAccess,

    /// The buffer receives a [`TOKEN_MANDATORY_POLICY`] structure that specifies the token's
    /// mandatory integrity policy.
    TokenMandatoryPolicy,

    /// The buffer receives a [`TOKEN_GROUPS`] structure that specifies the token's logon SID.
    TokenLogonSid,

    /// The buffer receives a [`DWORD`] value that is nonzero if the token is an app container
    /// token. Any callers who check the [`TOKEN_INFORMATION_CLASS::TokenIsAppContainer`] and have
    /// it return 0 should also verify that the caller token is not an identify level impersonation
    /// token. If the current token is not an app container but is an identity level token, you
    /// should return [`AccessDenied`].
    TokenIsAppContainer,

    /// The buffer receives a [`TOKEN_GROUPS`] structure that contains the capabilities associated
    /// with the token.
    TokenCapabilities,

    /// The buffer receives a [`TOKEN_APPCONTAINER_INFORMATION`] structure that contains the
    /// [`AppContainerSid`] associated with the token. If the token is not associated with an app
    /// container, the `token_app_container` member of the [`TOKEN_APPCONTAINER_INFORMATION`]
    /// structure points to [`null_mut`].
    TokenAppContainerSid,

    /// The buffer receives a [`DWORD`] value that includes the app container number for the token.
    /// For tokens that are not app container tokens, this value is zero.
    TokenAppContainerNumber,

    /// The buffer receives a [`CLAIM_SECURITY_ATTRIBUTES_INFORMATION`] structure that contains the
    /// user claims associated with the token.
    TokenUserClaimAttributes,

    /// The buffer receives a [`CLAIM_SECURITY_ATTRIBUTES_INFORMATION`] structure that contains the
    /// device claims associated with the token.
    TokenDeviceClaimAttributes,

    /// This value is reserved.
    TokenRestrictedUserClaimAttributes,

    /// This value is reserved.
    TokenRestrictedDeviceClaimAttributes,

    /// The buffer receives a [`TOKEN_GROUPS`] structure that contains the device groups that are
    /// associated with the token.
    TokenDeviceGroups,

    /// The buffer receives a [`TOKEN_GROUPS`] structure that contains the restricted device groups
    /// that are associated with the token.
    TokenRestrictedDeviceGroups,

    /// This value is reserved.
    TokenSecurityAttributes,

    /// This value is reserved.
    TokenIsRestricted,

    #[allow(missing_docs)]
    TokenProcessTrustLevel,

    #[allow(missing_docs)]
    TokenPrivateNameSpace,

    #[allow(missing_docs)]
    TokenSingletonAttributes,

    #[allow(missing_docs)]
    TokenBnoIsolation,

    #[allow(missing_docs)]
    TokenChildProcessFlags,

    /// Refers to a Least Privileged AppContainer (LPAC). An LPAC is effectively an AppContainer
    /// that's disregarded by the [`ALL_APPLICATION_PACKAGES`] SID.
    ///
    /// The maximum value for this enumeration.
    TokenIsLessPrivilegedAppContainer,
}
