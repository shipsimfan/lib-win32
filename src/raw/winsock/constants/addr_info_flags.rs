use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::GetAddrInfoW;

/// The socket address will be used in a call to the [`bind`] function.
pub const AI_PASSIVE: c_int = 0x01;

/// The canonical name is returned in the first `canon_name` member.
pub const AI_CANONNAME: c_int = 0x02;

/// The `node_name` parameter passed to the [`GetAddrInfoW`] function must be a
/// numeric string.
pub const AI_NUMERICHOST: c_int = 0x04;

/// If this bit is set, a request is made for IPv6 addresses and IPv4 addresses
/// with [`AI_V4MAPPED`].
///
/// This option is supported on Windows Vista and later.
pub const AI_ALL: c_int = 0x0100;

/// The [`GetAddrInfoW`] will resolve only if a global address is configured.
/// The IPv6 and IPv4 loopback address is not considered a valid global
/// address.
///
/// This option is only supported on Windows Vista and later.
pub const AI_ADDRCONFIG: c_int = 0x0400;

/// If the [`GetAddrInfoW`] request for an IPv6 addresses fails, a name service
/// request is made for IPv4 addresses and these addresses are converted to
/// IPv4-mapped IPv6 address format.
///
/// This option is supported on Windows Vista and later.
pub const AI_V4MAPPED: c_int = 0x0800;

/// The address information can be from a non-authoritative namespace provider.
///
/// This option is only supported on Windows Vista and later for the
/// [`NS_EMAIL`] namespace.
pub const AI_NON_AUTHORITATIVE: c_int = 0x04000;

/// The address information is from a secure channel.
///
/// This option is only supported on Windows Vista and later for the
/// [`NS_EMAIL`] namespace.
pub const AI_SECURE: c_int = 0x08000;

/// The address information is for a preferred name for a user.
///
/// This option is only supported on Windows Vista and later for the
/// [`NS_EMAIL`] namespace.
pub const AI_RETURN_PREFERRED_NAMES: c_int = 0x010000;

/// If a flat name (single label) is specified, [`GetAddrInfoW`] will return
/// the fully qualified domain name that the name eventually resolved to. The
/// fully qualified domain name is returned in the `canon_name` member.
///
/// This is different than [`AI_CANONNAME`] bit flag that returns the canonical
/// name registered in DNS which may be different than the fully qualified
/// domain name that the flat name resolved to.
///
/// Only one of the [`AI_FQDN`] and [`AI_CANONNAME`] bits can be set. The
/// [`GetAddrInfoW`] function will fail if both flags are present with
/// [`EAI_BADFLAGS`].
///
/// This option is supported on Windows 7, Windows Server 2008 R2, and later.
pub const AI_FQDN: c_int = 0x00020000;

/// A hint to the namespace provider that the hostname being queried is being
/// used in a file share scenario. The namespace provider may ignore this hint.
///
/// This option is supported on Windows 7, Windows Server 2008 R2, and later.
pub const AI_FILESERVER: c_int = 0x00040000;

/// Disable the automatic International Domain Name encoding using Punycode in
/// the name resolution functions called by the [`GetAddrInfoW`] function.
///
/// This option is supported on Windows 8, Windows Server 2012, and later.
pub const AI_DISABLE_IDN_ENCODING: c_int = 0x00080000;
