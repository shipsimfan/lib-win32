use crate::UINT;

/// The resource is unused and can be evicted as soon as another resource requires the memory that
/// the resource occupies.
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: UINT = 0x28000000;

/// The eviction priority of the resource is low. The placement of the resource is not critical,
/// and minimal work is performed to find a location for the resource. For example, if a GPU can
/// render with a vertex buffer from either local or non-local memory with little difference in
/// performance, that vertex buffer is low priority. Other more critical resources (for example, a
/// render target or texture) can then occupy the faster memory.
pub const DXGI_RESOURCE_PRIORITY_LOW: UINT = 0x50000000;

/// The eviction priority of the resource is normal. The placement of the resource is important,
/// but not critical, for performance. The resource is placed in its preferred location instead of
///  a low-priority resource.
pub const DXGI_RESOURCE_PRIORITY_NORMAL: UINT = 0x78000000;

/// The eviction priority of the resource is high. The resource is placed in its preferred location
/// instead of a low-priority or normal-priority resource.
pub const DXGI_RESOURCE_PRIORITY_HIGH: UINT = 0xA0000000;

/// The resource is evicted from memory only if there is no other way of resolving the memory
/// requirement.
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: UINT = 0xC8000000;
