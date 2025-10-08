use crate::{
    d3d11::{D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE, D3D11_AUTHENTICATED_QUERY_OUTPUT},
    HANDLE, UINT,
};

#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS;

/// Contains the response to a [`D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS`]
/// query.
///
/// # Remarks
/// The Desktop Window Manager (DWM) process is identified by setting `process_identifier` equal to
/// `D3D11_PROCESSIDTYPE_DWM`. Other processes are identified by setting the process handle in
/// `process_handle` and setting `process_identifier` equal to `D3D11_PROCESSIDTYPE_HANDLE`.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_OUTPUT`] structure that contains a Message Authentication Code
    /// (MAC) and other data.
    pub output: D3D11_AUTHENTICATED_QUERY_OUTPUT,

    /// The index of the process in the list of processes.
    pub process_index: UINT,

    /// A [`D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE`] value that specifies the type of process.
    pub process_identifier: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,

    /// A process handle. If the `process_identifier` member equals `D3D11_PROCESSIDTYPE_HANDLE`,
    /// the `process_handle` member contains a valid handle to a process. Otherwise, this member is
    /// ignored.
    pub process_handle: HANDLE,
}
