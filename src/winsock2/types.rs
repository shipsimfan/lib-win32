use crate::{
    winsock2::{WSABUF, WSADATA, WSAOVERLAPPED},
    DWORD, HANDLE, UINT_PTR,
};
use std::ffi::{c_uint, c_ulong, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{winsock2::WSARecv, SleepEx, WaitForMultipleObjectsEx, WaitForSingleObjectEx, TRUE};

/// A socket group
pub type GROUP = c_uint;

/// An IPv4 transport address
#[allow(non_camel_case_types)]
pub type in_addr = c_ulong;

/// An IPv6 transport address
#[allow(non_camel_case_types)]
pub type in6_addr = [u8; 16];

/// A pointer to a [`WSABUF`] structure
pub type LPWSABUF = *mut WSABUF;

/// A pointer to a [`WSADATA`] structure
pub type LPWSADATA = *mut WSADATA;

/// A pointer to a [`WSAOVERLAPPED`] structure
pub type LPWSAOVERLAPPED = *mut WSAOVERLAPPED;

/// [`LPWSAOVERLAPPED_COMPLETION_ROUTINE`] is a function pointer type. You implement a matching
/// callback function in your app, and pass that to functions such as [`WSAIoctl`], [`WSARecv`],
/// and [`WSASend`], among others.
///
/// The system calls your callback function when the asynchronous input and output (I/O) operation
/// is completed or canceled, and the calling thread is in an alertable state (by using the
/// [`SleepEx`], [`MsgWaitForMultipleObjectsEx`], [`WaitForSingleObjectEx`], or
/// [`WaitForMultipleObjectsEx`] function with the `alertable` parameter set to [`TRUE`]).
///
/// # Parameters
///  * `error` - The I/O completion status. This parameter can be one of the system error codes.
///  * `transferred` - The number of bytes transferred. If an error occurs, this parameter is zero.
///  * `overlapped` - A pointer to the [`WSAOVERLAPPED`] structure specified by the asynchronous
///                   I/O function. The system doesn't use the [`WSAOVERLAPPED`] structure after
///                   the completion routine is called, so the completion routine can deallocate
///                   the memory used by the overlapped structure.
///  * `flags` - Flags associated with the call.
#[allow(non_camel_case_types)]
pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE =
    extern "system" fn(error: DWORD, transferred: DWORD, overlapped: LPWSAOVERLAPPED, flags: DWORD);

/// A pointer to a [`WSAPROTOCOL_INFOW`] structure
#[allow(non_camel_case_types)]
pub type LPWSAPROTOCOL_INFOW = *mut c_void;

/// A network socket
pub type SOCKET = UINT_PTR;

/// A Windows Socket event
pub type WSAEVENT = HANDLE;
