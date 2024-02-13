use crate::DWORD;

#[allow(missing_docs)]
pub const HEAP_NO_SERIALIZE: DWORD = 0x00000001;

#[allow(missing_docs)]
pub const HEAP_GROWABLE: DWORD = 0x00000002;

#[allow(missing_docs)]
pub const HEAP_GENERATE_EXCEPTIONS: DWORD = 0x00000004;

#[allow(missing_docs)]
pub const HEAP_ZERO_MEMORY: DWORD = 0x00000008;

#[allow(missing_docs)]
pub const HEAP_REALLOC_IN_PLACE_ONLY: DWORD = 0x00000010;

#[allow(missing_docs)]
pub const HEAP_TAIL_CHECKING_ENABLED: DWORD = 0x00000020;

#[allow(missing_docs)]
pub const HEAP_FREE_CHECKING_ENABLED: DWORD = 0x00000040;

#[allow(missing_docs)]
pub const HEAP_DISABLE_COALESCE_ON_FREE: DWORD = 0x00000080;

#[allow(missing_docs)]
pub const HEAP_CREATE_ALIGN_16: DWORD = 0x00010000;

#[allow(missing_docs)]
pub const HEAP_CREATE_ENABLE_TRACING: DWORD = 0x00020000;

#[allow(missing_docs)]
pub const HEAP_CREATE_ENABLE_EXECUTE: DWORD = 0x00040000;

#[allow(missing_docs)]
pub const HEAP_MAXIMUM_TAG: DWORD = 0x0FFF;

#[allow(missing_docs)]
pub const HEAP_PSEUDO_TAG_FLAG: DWORD = 0x8000;

#[allow(missing_docs)]
pub const HEAP_TAG_SHIFT: DWORD = 18;

#[allow(missing_docs)]
pub const HEAP_CREATE_SEGMENT_HEAP: DWORD = 0x00000100;

#[allow(missing_docs)]
pub const HEAP_CREATE_HARDENED: DWORD = 0x00000200;
