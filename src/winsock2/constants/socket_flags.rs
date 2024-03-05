use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::{bind, WSASocket};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Create a socket that supports overlapped I/O operations.
///
/// Most sockets should be created with this flag set. Overlapped sockets can utilize [`WSASend`],
/// [`WSASendTo`], [`WSARecv`], [`WSARecvFrom`], and [`WSAIoctl`] for overlapped I/O operations,
/// which allow multiple operations to be initiated and in progress simultaneously.
///
/// All functions that allow overlapped operation ([`WSASend`], [`WSARecv`], [`WSASendTo`],
/// [`WSARecvFrom`], [`WSAIoctl`]) also support nonoverlapped usage on an overlapped socket if the
/// values for parameters related to overlapped operations are [`null_mut`].
pub const WSA_FLAG_OVERLAPPED: DWORD = 0x01;

/// Create a socket that will be a c_root in a multipoint session.
///
/// This attribute is only allowed if the [`WSAPROTOCOL_INFO`] structure for the transport provider
/// that creates the socket supports a multipoint or multicast mechanism and the control plane for
/// a multipoint session is rooted. This would be indicated by the `service_flags1` member of the
/// [`WSAPROTOCOL_INFO`] structure with the [`XP1_SUPPORT_MULTIPOINT`] and
/// [`XP1_MULTIPOINT_CONTROL_PLANE`] flags set.
///
/// When the `protocol_info` parameter is not [`null_mut`], the [`WSAPROTOCOL_INFO`] structure for
/// the transport provider is pointed to by the `protocol_info` parameter. When the `protocol_info`
/// parameter is [`null_mut`], the [`WSAPROTOCOL_INFO`] structure is based on the transport
/// provider selected by the values specified for the `af`, `type`, and `protocol` parameters.
pub const WSA_FLAG_MULTIPOINT_C_ROOT: DWORD = 0x02;

/// Create a socket that will be a c_leaf in a multipoint session.
///
/// This attribute is only allowed if the [`WSAPROTOCOL_INFO`] structure for the transport provider
/// that creates the socket supports a multipoint or multicast mechanism and the control plane for
/// a multipoint session is non-rooted. This would be indicated by the `service_flags1` member of
/// the [`WSAPROTOCOL_INFO`] structure with the [`XP1_SUPPORT_MULTIPOINT`] flag set and the
/// [`XP1_MULTIPOINT_CONTROL_PLANE`] flag not set.
///
/// When the `protocol_info` parameter is not [`null_mut`], the [`WSAPROTOCOL_INFO`] structure for
/// the transport provider is pointed to by the `protocol_info` parameter. When the `protocl_info`
/// parameter is [`null_mut`], the [`WSAPROTOCOL_INFO`] structure is based on the transport
/// provider selected by the values specified for the `af`, `type`, and `protocol` parameters.
///
/// When the `protocol_info` parameter is not [`null_mut`], the [`WSAPROTOCOL_INFO`] structure for
/// the transport provider is pointed to by the `protocol_info` parameter. When the `protocol_info`
/// parameter is [`null_mut`], the [`WSAPROTOCOL_INFO`] structure is based on the transport
/// provider selected by the values specified for the `af`, `type`, and `protocol` parameters.
pub const WSA_FLAG_MULTIPOINT_C_LEAF: DWORD = 0x04;

/// Create a socket that will be a d_leaf in a multipoint session.
///
/// This attribute is only allowed if the [`WSAPROTOCOL_INFO`] structure for the transport provider
/// that creates the socket supports a multipoint or multicast mechanism and the data plane for a
/// multipoint session is non-rooted. This would be indicated by the `service_flags1` member of the
/// [`WSAPROTOCOL_INFO`] structure with the [`XP1_SUPPORT_MULTIPOINT`] flag set and the
/// [`XP1_MULTIPOINT_DATA_PLANE`] flag not set.
///
/// When the `protocol_info` parameter is not [`null_mut`], the [`WSAPROTOCOL_INFO`] structure for
/// the transport provider is pointed to by the `protocol_info` parameter. When the `protocol_info`
/// parameter is [`null_mut`], the [`WSAPROTOCOL_INFO`] structure is based on the transport
/// provider selected by the values specified for the `af`, `type`, and `protocol` parameters.
pub const WSA_FLAG_MULTIPOINT_D_LEAF: DWORD = 0x10;

/// Create a socket that allows the ability to set a security descriptor on the socket that
/// contains a security access control list (SACL) as opposed to just a discretionary access
/// control list (DACL).
///
/// SACLs are used for generating audits and alarms when an access check occurs on the object. For
/// a socket, an access check occurs to determine whether the socket should be allowed to bind to a
/// specific address specified to the [`bind`] function.
///
/// The ACCESS_SYSTEM_SECURITY access right controls the ability to get or set the SACL in an
/// object's security descriptor. The system grants this access right only if the
/// `SE_SECURITY_NAME` privilege is enabled in the access token of the requesting thread.
pub const WSA_FLAG_ACCESS_SYSTEM_SECURITY: DWORD = 0x40;

/// Create a socket that is non-inheritable.
///
/// A socket handle created by the [`WSASocket`] or the socket function is inheritable by default.
/// When this flag is set, the socket handle is non-inheritable.
///
/// The GetHandleInformation function can be used to determine if a socket handle was created with
/// the [`WSA_FLAG_NO_HANDLE_INHERIT`] flag set. The [`GetHandleInformation`] function will return
/// that the [`HANDLE_FLAG_INHERIT`] value is set.
///
/// This flag is supported on Windows 7 with SP1, Windows Server 2008 R2 with SP1, and later
pub const WSA_FLAG_NO_HANDLE_INHERIT: DWORD = 0x80;
