use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{sockaddr, WSACleanup, WSAGetLastError, WSAStartup};

/// Most Windows Sockets 2 functions do not return the specific cause of an
/// error when the function returns. Some Winsock functions return a value of
/// zero if successful. Otherwise, the value [`SOCKET_ERROR`] (-1) is returned
/// and a specific error number can be retrieved by calling the
/// [`WSAGetLastError`] function.
pub const SOCKET_ERROR: c_int = -1;

/// Overlapped operation aborted.
///
/// An overlapped operation was canceled due to the closure of the socket, or
/// the execution of the `SIO_FLUSH`` command in [`WSAIoctl`].
pub const WSA_OPERATION_ABORTED: c_int = 995;

/// Bad address.
///
/// The system detected an invalid pointer address in attempting to use a
/// pointer argument of a call. This error occurs if an application passes an
/// invalid pointer value, or if the length of the buffer is too small. For
/// instance, if the length of an argument, which is a [`sockaddr`] structure,
/// is smaller than the [`std::mem::size_of<sockaddr>()`].
pub const WSAEFAULT: c_int = 10014;

/// Operation now in progress.
///
/// A blocking operation is currently executing. Windows Sockets only allows a
/// single blocking operation—per-task or thread—to be outstanding, and if any
/// other function call is made (whether or not it references that or any other
/// socket) the function fails with the [`WSAEINPROGRESS`] error.
pub const WSAEINPROGRESS: c_int = 10036;

/// Network is down.
///
/// A socket operation encountered a dead network. This could indicate a
/// serious failure of the network system (that is, the protocol stack that the
/// Windows Sockets DLL runs over), the network interface, or the local network
/// itself.
pub const WSAENETDOWN: c_int = 10050;

/// Too many processes.
///
/// A Windows Sockets implementation may have a limit on the number of
/// applications that can use it simultaneously. [`WSAStartup`] may fail with
/// this error if the limit has been reached.
pub const WSAEPROCLIM: c_int = 10067;

/// Network subsystem is unavailable.
///
/// This error is returned by [`WSAStartup`] if the Windows Sockets
/// implementation cannot function at this time because the underlying system
/// it uses to provide network services is currently unavailable. Users should
/// check:
///  - That the appropriate Windows Sockets DLL file is in the current path.
///  - That they are not trying to use more than one Windows Sockets
///    implementation simultaneously. If there is more than one Winsock DLL on
///    your system, be sure the first one in the path is appropriate for the
///    network subsystem currently loaded.
///  - The Windows Sockets implementation documentation to be sure all
///    necessary components are currently installed and configured correctly.
pub const WSASYSNOTREADY: c_int = 10091;

/// Winsock.dll version out of range.
///
/// The current Windows Sockets implementation does not support the Windows
/// Sockets specification version requested by the application. Check that no
/// old Windows Sockets DLL files are being accessed.
pub const WSAVERNOTSUPPORTED: c_int = 10092;

/// Successful [`WSAStartup`] not yet performed.
///
/// Either the application has not called [`WSAStartup`] or [`WSAStartup`]
/// failed. The application may be accessing a socket that the current active
/// task does not own (that is, trying to share a socket between tasks), or
/// [`WSACleanup`] has been called too many times.
pub const WSANOTINITIALISED: c_int = 10093;

/// Most Windows Sockets 2 functions do not return the specific cause of an
/// error when the function returns. For Winsock functions that return a
/// handle, a return value of [`INVALID_SOCKET`] (0xFFFF) indicates an error
/// and a specific error number can be retrieved by calling
/// [`WSAGetLastError`].
pub const INVALID_SOCKET: c_int = 0xFFFF;
