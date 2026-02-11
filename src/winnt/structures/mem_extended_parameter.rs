use crate::{DWORD, DWORD64, HANDLE, PVOID, SIZE_T};
use std::ops::{Deref, DerefMut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    MEM_ADDRESS_REQUIREMENTS, MEM_EXTENDED_PARAMETER_EC_CODE, MEM_EXTENDED_PARAMETER_NONPAGED_HUGE,
    MEM_EXTENDED_PARAMETER_NONPAGED_LARGE, MEM_EXTENDED_PARAMETER_TYPE,
};

/// Represents an extended parameter for a function that manages virtual memory.
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct MEM_EXTENDED_PARAMETER {
    /// A [`MEM_EXTENDED_PARAMETER_TYPE`] value that indicates the type of the parameter. If
    /// `r#type` is set to
    /// [`MEM_EXTENDED_PARAMETER_TYPE::MemExtendedParameterAddressRequirements`], then `pointer`
    /// must be a pointer to a caller-allocated [`MEM_ADDRESS_REQUIREMENTS`] structure that
    /// specifies the lowest and highest base address and alignment. If `r#type` is set to
    /// [`MEM_EXTENDED_PARAMETER_TYPE::MemExtendedParameterNumaNode`], then `ulong64` must be set
    /// to the desired node number. If `r#type` is set to MemExtendedParameterAttributeFlags, then
    /// `ulong64` must be set to a value that contains the desired flags:
    ///  * [`MEM_EXTENDED_PARAMETER_NONPAGED_LARGE`] - The allocation is mapped using large pages.
    ///  * [`MEM_EXTENDED_PARAMETER_NONPAGED_HUGE`] - The allocation is mapped using huge pages.
    ///  * [`MEM_EXTENDED_PARAMETER_EC_CODE`] - The allocation will contain emulation-compatible
    ///                                         (EC) code.
    pub r#type: DWORD64,

    #[allow(missing_docs)]
    pub dummy: MEM_EXTENDED_PARAMETER_UNION,
}

impl Deref for MEM_EXTENDED_PARAMETER {
    type Target = MEM_EXTENDED_PARAMETER_UNION;

    fn deref(&self) -> &Self::Target {
        &self.dummy
    }
}

impl DerefMut for MEM_EXTENDED_PARAMETER {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dummy
    }
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union MEM_EXTENDED_PARAMETER_UNION {
    /// If `r#Type` is set to [`MEM_EXTENDED_PARAMETER_TYPE::MemExtendedParameterNumaNode`], then
    /// `ulong64` must be set to the desired node number.
    pub ulong64: DWORD64,

    /// If `r#type` is set to
    /// [`MEM_EXTENDED_PARAMETER_TYPE::MemExtendedParameterAddressRequirements`], then `pointer`
    /// must be a pointer to a caller-allocated [`MEM_ADDRESS_REQUIREMENTS`] structure that
    /// specifies the lowest and highest base address and alignment.
    pub pointer: PVOID,

    #[allow(missing_docs)]
    pub size: SIZE_T,

    #[allow(missing_docs)]
    pub handle: HANDLE,

    #[allow(missing_docs)]
    pub ulong: DWORD,
}
