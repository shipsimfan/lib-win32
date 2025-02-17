use crate::UINT;

/// Allow CPU read access.
pub const DXGI_MAP_READ: UINT = 1;

/// Allow CPU write access.
pub const DXGI_MAP_WRITE: UINT = 2;

/// Discard the previous contents of a resource when it is mapped.
pub const DXGI_MAP_DISCARD: UINT = 4;
