use crate::raw::{AddrInfoW, Int, PAddrInfoW, PCStr};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{sockaddr, WChar, AF_INET, AF_INET6, AF_UNSPEC, AI_NUMERICHOST, SOCK_STREAM};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Ws2_32")]
extern "C" {
    /// # GetAddrInfoW function (ws2tcpip.h)
    ///
    /// The [`GetAddrInfoW`] function provides protocol-independent translation
    /// from a Unicode host name to an address.
    ///
    /// ## Parameters
    /// `node_name`\
    /// A pointer to a NULL-terminated Unicode string that contains a host
    /// (node) name or a numeric host address string. For the Internet
    /// protocol, the numeric host address string is a dotted-decimal IPv4
    /// address or an IPv6 hex address.
    ///
    /// `service_name`\
    /// A pointer to a NULL-terminated Unicode string that contains either a
    /// service name or port number represented as a string.
    ///
    /// A service name is a string alias for a port number. For example, “http”
    /// is an alias for port 80 defined by the Internet Engineering Task Force
    /// (IETF) as the default port used by web servers for the HTTP protocol.
    /// Possible values for the `service_name` parameter when a port number is
    /// not specified are listed in the following file:
    /// `%WINDIR%\system32\drivers\etc\services`
    ///
    /// `hints`\
    /// A pointer to an [`AddrInfoW`] structure that provides hints about the
    /// type of socket the caller supports.
    ///
    /// The `addr_len`, `canon_name`, `addr`, and `next` members of the
    /// [`AddrInfoW`] structure pointed to by the `hints` parameter must be
    /// zero or [`null`]. Otherwise the [`GetAddrInfoEx`] function will fail
    /// with [`WSANO_RECOVERY`].
    ///
    /// See the Remarks for more details.
    ///
    /// `result`\
    /// A pointer to a linked list of one or more [`AddrInfoW`] structures that
    /// contains response information about the host.
    ///
    /// ## Return Value
    /// Success returns zero. Failure returns a nonzero Windows Sockets error
    /// code, as found in the Windows Sockets Error Codes.
    ///
    /// Most nonzero error codes returned by the [`GetAddrInfoW`] function map
    /// to the set of errors outlined by Internet Engineering Task Force (IETF)
    /// recommendations. The following table lists these error codes and their
    /// WSA equivalents. It is recommended that the WSA error codes be used, as
    /// they offer familiar and comprehensive error information for Winsock
    /// programmers.
    /// | Error Value      | WSA Equivalent            | Description                                                                                                               |
    /// |------------------|---------------------------|---------------------------------------------------------------------------------------------------------------------------|
    /// | [`EAI_AGAIN`]    | [`WSATRY_AGAIN`]          | A temporary failure in name resolution occurred.                                                                          |
    /// | [`EAI_BADFLAGS`] | [`WSAEINVAL`]             | An invalid value was provided for the `flags` member of the `hints` parameter.                                            |
    /// | [`EAI_FAIL`]     | [`WSANO_RECOVERY`]        | A nonrecoverable failure in name resolution occurred.                                                                     |
    /// | [`EAI_FAMILY`]   | [`WSAEAFNOSUPPORT`]       | The `family` member of the `hints` parameter is not supported.                                                            |
    /// | [`EAI_MEMORY`]   | [`WSA_NOT_ENOUGH_MEMORY`] | A memory allocation failure occurred.                                                                                     |
    /// | [`EAI_NONAME`]   | [`WSAHOST_NOT_FOUND`]     | The name does not resolve for the supplied parameters or the `node_name` and `service_name` parameters were not provided. |
    /// | [`EAI_SERVICE`]  | [`WSATYPE_NOT_FOUND`]     | The `service_name` parameter is not supported for the specified `socktype` member of the `hints` parameter.               |
    /// | [`EAI_SOCKTYPE`] | [`WSAESOCKETNOSUPPORT`]   | The `sock_type` member of the `hints` parameter is not supported.                                                         |
    ///
    /// ## Remarks
    /// The [`GetAddrInfoW`] function is the Unicode version of a function that
    /// provides protocol-independent translation from host name to address.
    /// The ANSI version of this function is [`getaddrinfo`].
    ///
    /// The [`GetAddrInfoW`] function returns results for the [`NS_DNS`]
    /// namespace. The [`GetAddrInfoW`] function aggregates all responses if
    /// more than one namespace provider returns information. For use with the
    /// IPv6 and IPv4 protocol, name resolution can be by the Domain Name
    /// System (DNS), a local hosts file, or by other naming mechanisms for the
    /// [`NS_DNS`] namespace.
    ///
    /// Macros in the Winsock header file define a mixed-case function name of
    /// [`GetAddrInfoW`] and a [`AddrInfoW`] structure. This [`GetAddrInfoW`]
    /// function should be called with the `node_name` and `service_name`
    /// parameters of a pointer of type [`WChar`] and the `hints` and `result`
    /// parameters of a pointer of type [`AddrInfoW`].
    ///
    /// One or both of the `node_name` or `service_name` parameters must point
    /// to a NULL-terminated Unicode string; generally both are provided.
    ///
    /// Upon success, a linked list of [`AddrInfoW`] structures is returned in
    /// the `result` parameter. The list can be processed by following the
    /// pointer provided in the `next` member of each returned [`AddrInfoW`]
    /// structure until a [`null`] pointer is encountered. In each returned
    /// [`AddrInfoW`] structure, the `family`, `sock_type`, and `protocol`
    /// members correspond to respective arguments in a [`socket`] or
    /// [`WSASocket`] function call. Also, the `addr` member in each returned
    /// [`AddrInfoW`] structure points to a filled-in socket address structure,
    /// the length of which is specified in its `addr_len` member.
    ///
    /// If the `node_name` parameter points to a computer name, all permanent
    /// addresses for the computer that can be used as a source address are
    /// returned. On Windows Vista and later, these addresses would include all
    /// unicast IP addresses returned by the [`GetUnicastIpAddressTable`] or
    /// [`GetUnicastIpAddressEntry`] functions in which the `SkipAsSource`
    /// member is set to false in the [`MIB_UNICASTIPADDRESS_ROW`] structure.
    ///
    /// If the `node_name` parameter points to a string equal to "localhost",
    /// all loopback addresses on the local computer are returned.
    ///
    /// If the `node_name` parameter contains an empty string, all registered
    /// addresses on the local computer are returned.
    ///
    /// On Windows Server 2003 and later if the `node_name` parameter points to
    /// a string equal to "..localmachine", all registered addresses on the
    /// local computer are returned.
    ///
    /// If the `node_name` parameter refers to a cluster virtual server name,
    /// only virtual server addresses are returned. On Windows Vista and later,
    /// these addresses would include all unicast IP addresses returned by the
    /// [`GetUnicastIpAddressTable`] or [`GetUnicastIpAddressEntry`] functions
    /// in which the `SkipAsSource` member is set to true in the
    /// [`MIB_UNICASTIPADDRESS_ROW`] structure. See Windows Clustering for more
    /// information about clustering.
    ///
    /// Windows 7 with Service Pack 1 (SP1) and Windows Server 2008 R2 with
    /// Service Pack 1 (SP1) add support to Netsh.exe for setting the
    /// `SkipAsSource` attribute on an IP address. This also changes the
    /// behavior such that if the `SkipAsSource` member in the
    /// [`MIB_UNICASTIPADDRESS_ROW`] structure is set to false, the IP address
    /// will be registered in DNS. If the `SkipAsSource`` member is set to
    /// true, the IP address is not registered in DNS.
    ///
    /// A hotfix is available for Windows 7 and Windows Server 2008 R2 that
    /// adds support to Netsh.exe for setting the `SkipAsSource` attribute on
    /// an IP address. This hotfix also changes behavior such that if the
    /// `SkipAsSource` member in the [`MIB_UNICASTIPADDRESS_ROW`] structure is
    /// set to false, the IP address will be registered in DNS. If the
    /// `SkipAsSource` member is set to true, the IP address is not registered
    /// in DNS. For more information, see Knowledge Base (KB) 2386184.
    ///
    /// A similar hotfix is also available for Windows Vista with Service Pack
    /// 2 (SP2) and Windows Server 2008 with Service Pack 2 (SP2) that adds
    /// support to Netsh.exe for setting the `SkipAsSource` attribute on an IP
    /// address. This hotfix also changes behavior such that if the
    /// `SkipAsSource` member in the [`MIB_UNICASTIPADDRESS_ROW`] structure is
    /// set to false, the IP address will be registered in DNS. If the
    /// `SkipAsSource` member is set to true, the IP address is not registered
    /// in DNS.
    ///
    /// Callers of the [`GetAddrInfoW`] function can provide hints about the
    /// type of socket supported through an [`AddrInfoW`] structure pointed to
    /// by the `hints` parameter. When the `hints` parameter is used, the
    /// following rules apply to its associated [`AddrInfoW`] structure:
    ///  - A value of [`AF_UNSPEC`] for `family` indicates the caller will
    ///    accept only the [`AF_INET`] and [`AF_INET6`] address families. Note
    ///    that [`AF_UNSPEC`] and [`PF_UNSPEC`] are the same.
    ///  - A value of zero for `sock_type` indicates the caller will accept any
    ///    socket type.
    ///  - A value of zero for `protocol` indicates the caller will accept any
    ///    protocol.
    ///  - The `addr_len` member must be set to zero.
    ///  - The `canon_name` member must be set to [`null`].
    ///  - The `addr` member must be set to [`null`].
    ///  - The `next` member must be set to [`null`].
    ///
    /// Other values in the [`AddrInfoW`] structure provided in the `hints`
    /// parameter indicate specific requirements. For example, if the caller
    /// handles only IPv4 and does not handle IPv6, the `family` member should
    /// be set to [`AF_INET`]. For another example, if the caller handles only
    /// TCP and does not handle UDP, the `sock_type` member should be set to
    /// [`SOCK_STREAM`].
    ///
    /// If the `hints` parameter is a [`null`] pointer, the [`GetAddrInfoW`]
    /// function handles it as if the [`AddrInfoW`] structure in `hints` were
    /// initialized with its `family` member set to [`AF_UNSPEC`] and all other
    /// members set to zero.
    ///
    /// On Windows Vista and later when [`GetAddrInfoW`] is called from a
    /// service, if the operation is the result of a user process calling the
    /// service, then the service should impersonate the user. This is to allow
    /// security to be properly enforced.
    ///
    /// The [`GetAddrInfoW`] function can be used to convert a text string
    /// representation of an IP address to an [`AddrInfoW`] structure that
    /// contains a [`sockaddr`] structure for the IP address and other
    /// information. To be used in this way, the string pointed to by the
    /// `node_name` parameter must contain a text representation of an IP
    /// address and the [`AddrInfoW`] structure pointed to by the `hints`
    /// parameter must have the [`AI_NUMERICHOST`] flag set in the `flags`
    /// member. The string pointed to by the `node_name` parameter may contain
    /// a text representation of either an IPv4 or an IPv6 address. The text IP
    /// address is converted to an [`AddrInfoW`] structure pointed to by the
    /// `result` parameter. The returned [`AddrInfoW`] structure contains a
    /// [`sockaddr`] structure for the IP address along with additional
    /// information about the IP address. For this method to work with an IPv6
    /// address string on Windows Server 2003 and Windows XP, the IPv6 protocol
    /// must be installed on the local computer. Otherwise, the
    /// [`WSAHOST_NOT_FOUND`] error is returned.
    ///
    /// ### Freeing Address Information from Dynamic Allocation
    /// All information returned by the [`GetAddrInfoW`] function pointed to by
    /// the `result` parameter is dynamically allocated, including all
    /// [`AddrInfoW`] structures, socket address structures, and canonical host
    /// name strings pointed to by [`AddrInfoW`] structures. Memory allocated
    /// by a successful call to this function must be released with a
    /// subsequent call to [`FreeAddrInfoW`].
    pub fn GetAddrInfoW(
        node_name: PCStr,
        service_name: PCStr,
        hints: *const AddrInfoW,
        result: *mut *mut PAddrInfoW,
    ) -> Int;
}
