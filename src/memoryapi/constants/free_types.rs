use crate::DWORD;

/// Coalesces placeholders
pub const MEM_COALESCE_PLACEHOLDERS: DWORD = 0x00000001;

/// Frees an allocation back to a placeholder
pub const MEM_PRESERVE_PLACEHOLDER: DWORD = 0x00000002;

/// Decommits the specified region of committed pages
pub const MEM_DECOMMIT: DWORD = 0x00004000;

/// Releases the specified region of pages
pub const MEM_RELEASE: DWORD = 0x00008000;
