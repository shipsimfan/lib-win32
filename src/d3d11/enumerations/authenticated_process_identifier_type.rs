// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT;

/// Specifies the type of process that is identified in the
/// [`D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE {
    /// Unknown process type.
    Unknown = 1,

    /// Desktop Window Manager (DWM) process.
    DWM = 2,

    /// Handle to a process.
    Handle1 = 3,
}
