/// Specifies the type of process that is identified in the
/// [`D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE {
    /// Unknown process type.
    Unknown = 1,

    /// Desktop Window Manager (DWM) process.
    DWM = 2,

    /// Handle to a process.
    Handle1 = 3,
}
