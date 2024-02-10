// rustdoc imports
#[allow(unused_imports)]
use crate::VirtualAlloc2;

/// Defines values for extended parameters used for file mapping into an address space.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum MEM_EXTENDED_PARAMETER_TYPE {
    /// 0
    MemExtendedParameterInvalidType = 0,

    /// This extended parameter type is used to specify alignment and virtual address range
    /// restrictions for new memory allocations created by [`VirtualAlloc2`] and
    /// [`MapViewOfFile3`].
    MemExtendedParameterAddressRequirements,

    /// This extended parameter type is used to specify the preferred NUMA node for new memory
    /// allocations created by [`VirtualAlloc2`] and [`MapViewOfFile3`].
    MemExtendedParameterNumaNode,

    #[allow(missing_docs)]
    MemExtendedParameterPartitionHandle,

    #[allow(missing_docs)]
    MemExtendedParameterUserPhysicalHandle,

    /// This extended parameter type is used to specify additional flags for new memory allocations
    /// created by [`VirtualAlloc2`] and [`MapViewOfFile3`].
    MemExtendedParameterAttributeFlags,

    #[allow(missing_docs)]
    MemExtendedParameterImageMachine,
}
