use crate::{DWORD, DWORD_PTR, LPVOID, WORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VirtualAlloc, PROCESSOR_ARCHITECTURE_AMD64, PROCESSOR_ARCHITECTURE_ARM,
    PROCESSOR_ARCHITECTURE_ARM64, PROCESSOR_ARCHITECTURE_IA64, PROCESSOR_ARCHITECTURE_INTEL,
    PROCESSOR_ARCHITECTURE_UNKNOWN,
};

/// Contains information about the current computer system. This includes the architecture and type
/// of the processor, the number of processors in the system, the page size, and other such
/// information.
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SYSTEM_INFO {
    #[allow(missing_docs)]
    pub u: SYSTEM_INFO_UNION,

    /// The page size and the granularity of page protection and commitment. This is the page size
    /// used by the [`VirtualAlloc`] function.
    pub page_size: DWORD,

    /// A pointer to the lowest memory address accessible to applications and dynamic-link
    /// libraries (DLLs).
    pub minimum_application_address: LPVOID,

    /// A pointer to the highest memory address accessible to applications and DLLs.
    pub maximum_application_address: LPVOID,

    /// A mask representing the set of processors configured into the system. Bit 0 is processor 0;
    /// bit 31 is processor 31.
    pub active_processor_mask: DWORD_PTR,

    /// The number of logical processors in the current group. To retrieve the current processor
    /// group, use the [`GetLogicalProcessorInformation`] function.
    pub number_of_processors: DWORD,

    /// An obsolete member that is retained for compatibility. Use the `process_architecture`,
    /// `processor_level`, and `processor_revision` members to determine the type of processor.
    pub processor_type: DWORD,

    /// The granularity for the starting address at which virtual memory can be allocated. For more
    /// information, see [`VirtualAlloc`].
    pub allocation_granularity: DWORD,

    /// The architecture-dependent processor level. It should be used only for display purposes. To
    /// determine the feature set of a processor, use the [`IsProcessorFeaturePresent`] function.
    ///
    /// If `processor_architecture` is [`PROCESSOR_ARCHITECTURE_INTEL`], `processor_level` is
    /// defined by the CPU vendor.
    ///
    /// If `processor_architecture` is [`PROCESSOR_ARCHITECTURE_IA64`], `processor_level` is set to
    /// 1.
    pub processor_level: WORD,

    /// The architecture-dependent processor revision.
    pub processor_revision: WORD,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union SYSTEM_INFO_UNION {
    /// An obsolete member that is retained for compatibility. Applications should use the
    /// `p.processor_architecture` branch of the union.
    pub oem_id: DWORD,

    #[allow(missing_docs)]
    pub p: SYSTEM_INFO_PROCESSOR_ARCHITECTURE,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SYSTEM_INFO_PROCESSOR_ARCHITECTURE {
    /// The processor architecture of the installed operating system. This member can be one of the
    /// following values:
    ///  * [`PROCESSOR_ARCHITECTURE_AMD64`] - x64 (AMD or Intel)
    ///  * [`PROCESSOR_ARCHITECTURE_ARM`] - ARM
    ///  * [`PROCESSOR_ARCHITECTURE_ARM64`] - ARM64
    ///  * [`PROCESSOR_ARCHITECTURE_IA64`] - Intel Itanium-based
    ///  * [`PROCESSOR_ARCHITECTURE_INTEL`] - x86
    ///  * [`PROCESSOR_ARCHITECTURE_UNKNOWN`] - Unknown architecture
    pub processor_architecture: WORD,

    /// This member is reserved for future use.
    pub reserved: WORD,
}
