use crate::{
    d3d11::{D3D11_AUTHENTICATED_CONFIGURE_INPUT, D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE},
    BOOL, HANDLE,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE, TRUE};

/// Contains input data for a [`D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE`] command.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    /// A [`D3D11_AUTHENTICATED_CONFIGURE_INPUT`] structure that contains the command GUID and
    /// other data.
    pub parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,

    /// A [`D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE`] value that specifies the type of process.
    ///
    /// To specify the Desktop Window Manager (DWM) process, set this member to
    /// [`D3D11_PROCESSIDTYPE_DWM`]. Otherwise, set this member to [`D3D11_PROCESSIDTYPE_HANDLE`]
    /// and set the ProcessHandle member to a valid handle.
    pub process_type: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,

    /// A process handle. If the `process_type` member equals [`D3D11_PROCESSIDTYPE_HANDLE`], the
    /// `process_handle` member specifies a handle to a process. Otherwise, the value is ignored.
    pub process_handle: HANDLE,

    /// If [`TRUE`], the specified process has access to restricted shared resources.
    pub allow_access: BOOL,
}
