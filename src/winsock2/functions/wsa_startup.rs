use crate::{winsock2::LPWSADATA, WORD};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::{WSAGetLastError, WSADATA};

#[link(name = "Ws2_32")]
unsafe extern "system" {
    /// The [`WSAStartup`] function initiates use of the Winsock DLL by a process.
    ///
    /// # Parameters
    ///  * `version_required` - The highest version of Windows Sockets specification that the
    ///                         caller can use. The high-order byte specifies the minor version
    ///                         number; the low-order byte specifies the major version number.
    ///  * `wsa_data` - A pointer to the [`WSADATA`] data structure that is to receive the details
    ///                 of the Windows Sockets implementation.
    ///
    /// # Return Value
    /// If successful, the [`WSAStartup`] function returns zero. Otherwise, it returns an error
    /// code.
    ///
    /// The [`WSAStartup`] function directly returns the extended error code in the return value
    /// for this function. A call to the [`WSAGetLastError`] function is not needed and should not
    /// be used.
    ///
    /// # Remarks
    /// The [`WSAStartup`] function must be the first Windows Sockets function called by an
    /// application or DLL. It allows an application or DLL to specify the version of Windows
    /// Sockets required and retrieve details of the specific Windows Sockets implementation. The
    /// application or DLL can only issue further Windows Sockets functions after successfully
    /// calling [`WSAStartup`].
    ///
    /// When an application or DLL calls the [`WSAStartup`] function, the Winsock DLL examines the
    /// version of the Windows Sockets specification requested by the application passed in the
    /// `version_requested` parameter. If the version requested by the application is equal to or
    /// higher than the lowest version supported by the Winsock DLL, the call succeeds and the
    /// Winsock DLL returns detailed information in the [`WSADATA`] structure pointed to by the
    /// `wsa_data` parameter. The `high_version` member of the [`WSADATA`] structure indicates the
    /// highest version of the Windows Sockets specification that the Winsock DLL supports. The
    /// `version` member of the [`WSADATA`] structure indicates the version of the Windows Sockets
    /// specification that the Winsock DLL expects the caller to use.
    ///
    /// If the `version` member of the [`WSADATA`] structure is unacceptable to the caller, the
    /// application or DLL should call [`WSACleanup`] to release the Winsock DLL resources and fail
    /// to initialize the Winsock application. In order to support this application or DLL, it will
    /// be necessary to search for an updated version of the Winsock DLL to install on the
    /// platform.
    ///
    /// The current version of the Windows Sockets specification is version 2.2. The current
    /// Winsock DLL, `Ws2_32.dll`, supports applications that request any of the following versions
    /// of Windows Sockets specification:
    ///  - 1.0
    ///  - 1.1
    ///  - 2.0
    ///  - 2.1
    ///  - 2.2
    ///
    /// To get full access to the new syntax of a higher version of the Windows Sockets
    /// specification, the application must negotiate for this higher version. In this case, the
    /// `version_requested` parameter should be set to request version 2.2. The application must
    /// also fully conform to that higher version of the Windows Socket specification, such as
    /// compiling against the appropriate header file, linking with a new library, or other special
    /// cases. The Winsock2.h header file for Winsock 2 support is included with the Microsoft
    /// Windows Software Development Kit (SDK).
    ///
    /// Windows Sockets version 2.2 is supported on Windows Server 2008, Windows Vista, Windows
    /// Server 2003, Windows XP, Windows 2000, Windows NT 4.0 with Service Pack 4 (SP4) and later,
    /// Windows Me, Windows 98, and Windows 95 OSR2. Windows Sockets version 2.2 is also supported
    /// on Windows 95 with the Windows Socket 2 Update. Applications on these platforms should
    /// normally request Winsock 2.2 by setting the `version_requested` parameter accordingly.
    ///
    /// On Windows 95 and versions of Windows NT 3.51 and earlier, Windows Sockets version 1.1 is
    /// the highest version of the Windows Sockets specification supported.
    ///
    /// It is legal and possible for an application or DLL written to use a lower version of the
    /// Windows Sockets specification that is supported by the Winsock DLL to successfully
    /// negotiate this lower version using the [`WSAStartup`] function. For example, an application
    /// can request version 1.1 in the `version_requested` parameter passed to the [`WSAStartup`]
    /// function on a platform with the Winsock 2.2 DLL. In this case, the application should only
    /// rely on features that fit within the version requested. New Ioctl codes, new behavior of
    /// existing functions, and new functions should not be used. The version negotiation provided
    /// by the [`WSAStartup`] was primarily used to allow older Winsock 1.1 applications developed
    /// for Windows 95 and Windows NT 3.51 and earlier to run with the same behavior on later
    /// versions of Windows.
    ///
    /// This negotiation in the [`WSAStartup`] function allows both the application or DLL that
    /// uses Windows Sockets and the Winsock DLL to support a range of Windows Sockets versions. An
    /// application or DLL can use the Winsock DLL if there is any overlap in the version ranges.
    /// Detailed information on the Windows Sockets implementation is provided in the [`WSADATA`]
    /// structure returned by the [`WSAStartup`] function.
    ///
    /// Once an application or DLL has made a successful [`WSAStartup`] call, it can proceed to
    /// make other Windows Sockets calls as needed. When it has finished using the services of the
    /// Winsock DLL, the application must call [`WSACleanup`] to allow the Winsock DLL to free
    /// internal Winsock resources used by the application.
    ///
    /// An application can call [`WSAStartup`] more than once if it needs to obtain the [`WSADATA`]
    /// structure information more than once. On each such call, the application can specify any
    /// version number supported by the Winsock DLL.
    ///
    /// The [`WSAStartup`] function typically leads to protocol-specific helper DLLs being loaded.
    /// As a result, the [`WSAStartup`] function should not be called from the `DllMain` function
    /// in a application DLL. This can potentially cause deadlocks.
    ///
    /// An application must call the [`WSACleanup`] function for every successful time the
    /// [`WSAStartup`] function is called. This means, for example, that if an application calls
    /// [`WSAStartup`] three times, it must call [`WSACleanup`] three times. The first two calls to
    /// [`WSACleanup`] do nothing except decrement an internal counter; the final WSACleanup call
    /// for the task does all necessary resource deallocation for the task.
    pub fn WSAStartup(version_required: WORD, wsa_data: LPWSADATA) -> c_int;
}
