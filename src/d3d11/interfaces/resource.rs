use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, D3D11_RESOURCE_DIMENSION},
    unknwn::{IUnknown},
    UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::{
    DXGI_RESOURCE_PRIORITY_HIGH, DXGI_RESOURCE_PRIORITY_LOW, DXGI_RESOURCE_PRIORITY_MAXIMUM,
    DXGI_RESOURCE_PRIORITY_MINIMUM, DXGI_RESOURCE_PRIORITY_NORMAL,
};

com_interface!(
    /// A resource interface provides common actions on all resources.
    ///
    /// # Remarks
    /// You don't directly create a resource interface; instead, you create buffers and textures
    /// that inherit from a resource interface.
    pub abstract ID3D11Resource(ID3D11ResourceVTable):
        ID3D11DeviceChild(device_child) +
        IUnknown {
        const IID = 0xDC8E63F3-0xD12B-0x4952-0xB47B-0x5E45026A862D;

        /// Get the type of the resource.
        ///
        /// # Parameters
        ///  * `resource_dimension` - Pointer to the resource type (see
        ///                           [`D3D11_RESOURCE_DIMENSION`]).
        fn get_type(&mut self, resource_dimension: *mut D3D11_RESOURCE_DIMENSION);

        /// Set the eviction priority of a resource.
        ///
        /// # Parameters
        ///  * `eviction_priority` - Eviction priority for the resource, which is one of the
        ///                          following values:
        ///    - [`DXGI_RESOURCE_PRIORITY_MINIMUM`]
        ///    - [`DXGI_RESOURCE_PRIORITY_LOW`]
        ///    - [`DXGI_RESOURCE_PRIORITY_NORMAL`]
        ///    - [`DXGI_RESOURCE_PRIORITY_HIGH`]
        ///    - [`DXGI_RESOURCE_PRIORITY_MAXIMUM`]
        ///
        /// # Remarks
        /// Resource priorities determine which resource to evict from video memory when the system
        /// has run out of video memory. The resource will not be lost; it will be removed from
        /// video memory and placed into system memory, or possibly placed onto the hard drive. The
        /// resource will be loaded back into video memory when it is required.
        ///
        /// A resource that is set to the maximum priority, [`DXGI_RESOURCE_PRIORITY_MAXIMUM`], is
        /// only evicted if there is no other way of resolving the incoming memory request. The
        /// Windows Display Driver Model (WDDM) tries to split an incoming memory request to its
        /// minimum size and evict lower-priority resources before evicting a resource with maximum
        /// priority.
        ///
        /// Changing the priorities of resources should be done carefully. The wrong eviction
        /// priorities could be a detriment to performance rather than an improvement.
        fn set_eviction_priority(&mut self, eviction_priority: UINT);

        /// Get the eviction priority of a resource.
        ///
        /// # Return Value
        /// One of the following values, which specifies the eviction priority for the resource:
        ///  - [`DXGI_RESOURCE_PRIORITY_MINIMUM`]
        ///  - [`DXGI_RESOURCE_PRIORITY_LOW`]
        ///  - [`DXGI_RESOURCE_PRIORITY_NORMAL`]
        ///  - [`DXGI_RESOURCE_PRIORITY_HIGH`]
        ///  - [`DXGI_RESOURCE_PRIORITY_MAXIMUM`]
        fn get_eviction_priority(&mut self) -> UINT;
    }
);
