use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    winsock2::{WSAGetLastError, SOCKET_ERROR},
    GUID,
};

#[link(name = "Ws2_32")]
unsafe extern "system" {
    /// The [`gethostname`] function retrieves the standard host name for the local computer.
    ///
    /// # Parameters
    ///  * `name` - A pointer to a buffer that receives the local host name.
    ///  * `namelen` - The length, in bytes, of the buffer pointed to by the name parameter.
    ///
    /// # Return Value
    /// If no error occurs, [`gethostname`] returns zero. Otherwise, it returns [`SOCKET_ERROR`]
    /// and a specific error code can be retrieved by calling [`WSAGetLastError`].
    ///
    /// # Remarks
    /// The [`gethostname`] function returns the name of the local host into the buffer specified
    /// by the `name` parameter. The host name is returned as a null-terminated string. The form of
    /// the host name is dependent on the Windows Sockets provider—it can be a simple host name, or
    /// it can be a fully qualified domain name. However, it is guaranteed that the name returned
    /// will be successfully parsed by [`gethostbyname`] and [`WSAAsyncGetHostByName`].
    ///
    /// The maximum length of the name returned in the buffer pointed to by the `name` parameter is
    /// dependent on the namespace provider.
    ///
    /// If the [`gethostname`] function is used on a cluster resource on Windows Server 2008,
    /// Windows Server 2003, or Windows 2000 Server and the `CLUSTER_NETWORK_NAME` environment
    /// variable is defined, then the value in this environment variable overrides the actual
    /// hostname and is returned. On a cluster resource, the `CLUSTER_NETWORK_NAME` environment
    /// variable contains the name of the cluster.
    ///
    /// The [`gethostname`] function queries namespace providers to determine the local host name
    /// using the [`SVCID_HOSTNAME`] [`GUID`]. If no namespace provider responds, then the
    /// [`gethostname`] function returns the NetBIOS name of the local computer.
    ///
    /// The maximum length, in bytes, of the string returned in the buffer pointed to by the `name`
    /// parameter is dependent on the namespace provider, but this string must be 256 bytes or
    /// less. So if a buffer of 256 bytes is passed in the `name` parameter and the `namelen`
    /// parameter is set to 256, the buffer size will always be adequate.
    pub fn gethostname(name: *mut c_char, namelen: c_int) -> c_int;
}
