use crate::{
    com_interface,
    dxgi::{
        IDXGIDeviceSubObject, IDXGIDeviceSubObjectTrait, IDXGIObject, IDXGIObjectTrait, DXGI_USAGE,
    },
    unknwn::{IUnknown, IUnknownTrait},
    HANDLE, HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11")]
use crate::d3d11::ID3D11Device;
#[allow(unused_imports)]
use crate::{
    dxgi::{
        DXGI_RESOURCE_PRIORITY_HIGH, DXGI_RESOURCE_PRIORITY_LOW, DXGI_RESOURCE_PRIORITY_MAXIMUM,
        DXGI_RESOURCE_PRIORITY_MINIMUM, DXGI_RESOURCE_PRIORITY_NORMAL,
    },
    CloseHandle,
};

com_interface!(
    /// An [`IDXGIResource`] interface allows resource sharing and identifies the memory that a
    /// resource resides in.
    pub abstract IDXGIResource(IDXGIResourceVTable/IDXGIResourceTrait):
        IDXGIDeviceSubObject/IDXGIDeviceSubObjectTrait(device_sub_object) +
        IDXGIObject/IDXGIObjectTrait(device_sub_object.object) +
        IUnknown/IUnknownTrait(device_sub_object.object.unknown) {
        const IID = 0x035F3AB4-0x482E-0x4E50-0xB41F-0x8A7F8BD8960B;

        /// Gets the handle to a shared resource.
        ///
        /// # Parameters
        ///  * `shared_handle` - A pointer to a handle.
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        ///
        /// # Remarks
        /// [`IDXGIResource::get_shared_handle`] returns a handle for the resource that you created
        /// as shared (that is, you set the [`D3D11_RESOURCE_MISC_SHARED`] with or without the
        /// [`D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX`] flag). You can pass this handle to the
        /// [`ID3D11Device::open_shared_resource`] method to give another device access to the
        /// shared resource. You can also marshal this handle to another process to share a
        /// resource with a device in another process. However, this handle is not an NT handle.
        /// Therefore, don't use the handle with [`CloseHandle`], [`DuplicateHandle`], and so on.
        ///
        /// The creator of a shared resource must not destroy the resource until all intended
        /// entities have opened the resource. The validity of the handle is tied to the lifetime
        /// of the underlying video memory. If no resource objects exist on any devices that refer
        /// to this resource, the handle is no longer valid. To extend the lifetime of the handle
        /// and video memory, you must open the shared resource on a device.
        ///
        /// [`IDXGIResource::get_shared_handle`] can also return handles for resources that were
        /// passed into [`ID3D11Device::open_shared_resource`] to open those resources.
        ///
        /// [`IDXGIResource::get_shared_handle`] fails if the resource to which it wants to get a
        /// handle is not shared.
        fn get_shared_handle(&mut self, shared_handle: *mut HANDLE) -> HRESULT;

        /// Get the expected resource usage.
        ///
        /// # Parameters
        ///  * `usage` - A pointer to a usage flag (see [`DXGI_USAGE`]). For Direct3D 10, a surface
        /// can be used as a shader input or a render-target output.
        ///
        /// # Return Value
        /// Returns one of the following `DXGI_ERROR`.
        fn get_usage(&mut self, usage: *mut DXGI_USAGE) -> HRESULT;

        /// Set the priority for evicting the resource from memory.
        ///
        /// # Parameters
        ///  * `eviction_priority` - The priority is one of the following values:
        ///    * [`DXGI_RESOURCE_PRIORITY_MINIMUM`] - The resource is unused and can be evicted as
        ///                                           soon as another resource requires the memory
        ///                                           that the resource occupies.
        ///    * [`DXGI_RESOURCE_PRIORITY_LOW`] - The eviction priority of the resource is low. The
        ///                                       placement of the resource is not critical, and
        ///                                       minimal work is performed to find a location for
        ///                                       the resource. For example, if a GPU can render
        ///                                       with a vertex buffer from either local or
        ///                                       non-local memory with little difference in
        ///                                       performance, that vertex buffer is low priority.
        ///                                       Other more critical resources (for example, a
        ///                                       render target or texture) can then occupy the
        ///                                       faster memory.
        ///    * [`DXGI_RESOURCE_PRIORITY_NORMAL`] - The eviction priority of the resource is
        ///                                          normal. The placement of the resource is
        ///                                          important, but not critical, for performance.
        ///                                          The resource is placed in its preferred
        ///                                          location instead of a low-priority resource.
        ///    * [`DXGI_RESOURCE_PRIORITY_HIGH`] - The eviction priority of the resource is high.
        ///                                        The resource is placed in its preferred location
        ///                                        instead of a low-priority or normal-priority
        ///                                        resource.
        ///    * [`DXGI_RESOURCE_PRIORITY_MAXIMUM`] - The resource is evicted from memory only if
        ///                                           there is no other way of resolving the memory
        ///                                           requirement.
        ///
        /// # Return Value
        /// Returns one of the following `DXGI_ERROR`.
        ///
        /// # Remarks
        /// The eviction priority is a memory-management variable that is used by DXGI for
        /// determining how to populate overcommitted memory.
        ///
        /// You can set priority levels other than the defined values when appropriate. For
        /// example, you can set a resource with a priority level of `0x78000001` to indicate that
        /// the resource is slightly above normal.
        fn set_eviction_priority(&mut self, eviction_priority: UINT) -> HRESULT;

        /// Get the eviction priority.
        ///
        /// # Parameters
        ///  * `eviction_priority` - A pointer to the eviction priority, which determines when a
        ///                          resource can be evicted from memory.
        ///
        /// # Return Value
        /// Returns one of the following `DXGI_ERROR`.
        ///
        /// # Remarks
        /// The eviction priority is a memory-management variable that is used by DXGI to determine
        /// how to manage overcommitted memory.
        ///
        /// Priority levels other than the defined values are used when appropriate. For example, a
        /// resource with a priority level of `0x78000001` indicates that the resource is slightly
        /// above normal.
        fn get_eviction_priority(&mut self, eviction_priority: *mut UINT) -> HRESULT;
    }
);
