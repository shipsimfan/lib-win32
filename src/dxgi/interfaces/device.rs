use crate::{
    com_interface,
    dxgi::{
        IDXGIAdapter, IDXGIObject, IDXGIObjectTrait, IDXGISurface, DXGI_RESIDENCY,
        DXGI_SHARED_RESOURCE, DXGI_SURFACE_DESC, DXGI_USAGE,
    },
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, INT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi::IDXGIResource, DXGI_ERROR_DEVICE_REMOVED, E_INVALIDARG, E_POINTER, S_OK};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// An IDXGIDevice interface implements a derived class for DXGI objects that produce image
    /// data.
    pub abstract IDXGIDevice(IDXGIDeviceVTable/IDXGIDeviceTrait):
        IDXGIObject/IDXGIObjectTrait(object) +
        IUnknown/IUnknownTrait(object.unknown) {
        const IID = 0x54EC77FA-0x1377-0x44E6-0x8C32-0x88FD5F44C84C;

        /// Returns the adapter for the specified device.
        ///
        /// # Parameters
        ///  * `adapter` - The address of an [`IDXGIAdapter`] interface pointer to the adapter.
        ///                This parameter must not be [`null_mut`].
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the `DXGI_ERROR` that
        /// indicates failure. If the `adapter` parameter is [`null_mut`] this method returns
        /// [`E_INVALIDARG`].
        ///
        /// # Remarks
        /// If the [`IDXGIDevice::get_adapter`] method succeeds, the reference count on the adapter
        /// interface will be incremented. To avoid a memory leak, be sure to release the interface
        /// when you are finished using it.
        fn get_adapter(&mut self, adapter: *mut *mut IDXGIAdapter) -> HRESULT;

        /// Returns a surface. This method is used internally and you should not call it directly
        /// in your application.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`DXGI_SURFACE_DESC`] structure that describes the surface.
        ///  * `num_surfaces` - The number of surfaces to create.
        ///  * `usage` - A [`DXGI_USAGE`] flag that specifies how the surface is expected to be
        ///              used.
        ///  * `shared_resource` - An optional pointer to a [`DXGI_SHARED_RESOURCE`] structure that
        ///                        contains shared resource information for opening views of such
        ///                        resources.
        ///  * `surface` - The address of an [`IDXGISurface`] interface pointer to the first
        ///                created surface.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// # Remarks
        /// The [`IDXGIDevice::create_surface`] method creates a buffer to exchange data between
        /// one or more devices. It is used internally, and you should not directly call it.
        ///
        /// The runtime automatically creates an [`IDXGISurface`] interface when it creates a
        /// Direct3D resource object that represents a surface. For example, the runtime creates an
        /// [`IDXGISurface`] interface when it calls [`ID3D11Device::create_texture_2d`] or
        /// [`ID3D10Device::create_texture_2d`] to create a 2D texture. To retrieve the
        /// [`IDXGISurface`] interface that represents the 2D texture surface, call
        /// [`ID3D11Texture2D::query_interface`] or [`ID3D10Texture2D::query_interface`]. In this
        /// call, you must pass the identifier of [`IDXGISurface`]. If the 2D texture has only a
        /// single MIP-map level and does not consist of an array of textures, `query_interface`
        /// succeeds and returns a pointer to the [`IDXGISurface`] interface pointer. Otherwise,
        /// `query_interface` fails and does not return the pointer to [`IDXGISurface`].
        fn create_surface(
            &mut self,
            desc: *const DXGI_SURFACE_DESC,
            num_surfaces: UINT,
            usage: DXGI_USAGE,
            shared_resource: *const DXGI_SHARED_RESOURCE,
            surface: *mut *mut IDXGISurface
        ) -> HRESULT;

        /// Gets the residency status of an array of resources.
        ///
        /// # Parameters
        ///  * `resources` - An array of [`IDXGIResource`] interfaces.
        ///  * `residency_status` - An array of [`DXGI_RESIDENCY`] flags. Each element describes
        ///                         the residency status for corresponding element in the
        ///                         `resources` argument array.
        ///  * `num_resources` - The number of resources in the `resources` argument array and
        ///                      `residency_status` argument array.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns [`DXGI_ERROR_DEVICE_REMOVED`],
        /// [`E_INVALIDARG`], or [`E_POINTER`].
        ///
        /// # Remarks
        /// The information returned by the `residency_status` argument array describes the
        /// residency status at the time that the [`IDXGIDevice::query_resource_residency`] method
        /// was called.
        ///
        /// If you call the [`IDXGIDevice::query_resource_residency`] method during a device
        /// removed state, the `residency_status` argument will return the
        /// [`DXGI_RESIDENCY::ResidentInSharedMemory`] flag.
        fn query_resource_residency(
            &mut self,
            resources: *mut *const IUnknown,
            residency_status: *mut DXGI_RESIDENCY,
            num_resources: UINT
        ) -> HRESULT;

        /// Sets the GPU thread priority.
        ///
        /// # Parameters
        ///  * `priority` - A value that specifies the required GPU thread priority. This value
        ///                 must be between -7 and 7, inclusive, where 0 represents normal
        ///                 priority.
        ///
        /// # Return Value
        /// Return [`S_OK`] if successful; otherwise, returns [`E_INVALIDARG`] if the `priority`
        /// parameter is invalid.
        ///
        /// # Remarks
        /// The values for the `priority` parameter function as follows:
        ///  - Positive values increase the likelihood that the GPU scheduler will grant GPU
        ///    execution cycles to the device when rendering.
        ///  - Negative values lessen the likelihood that the device will receive GPU execution
        ///    cycles when devices compete for them.
        ///  - The device is guaranteed to receive some GPU execution cycles at all settings.
        ///
        /// To use the [`IDXGIDevice::set_gpu_thread_priority`] method, you should have a
        /// comprehensive understanding of GPU scheduling. You should profile your application to
        /// ensure that it behaves as intended. If used inappropriately, the
        /// [`IDXGIDevice::set_gpu_thread_priority`] method can impede rendering speed and result
        /// in a poor user experience.
        fn set_gpu_thread_priority(&mut self, priority: INT) -> HRESULT;

        /// Gets the GPU thread priority.
        ///
        /// # Parameters
        ///  * `priority` - A pointer to a variable that receives a value that indicates the
        ///                 current GPU thread priority. The value will be between -7 and 7,
        ///                 inclusive, where 0 represents normal priority.
        ///
        /// # Return Value
        /// Return [`S_OK`] if successful; otherwise, returns [`E_POINTER`] if the `priority`
        /// parameter is [`null_mut`].
        fn get_gpu_thread_priority(&mut self, priority: *mut INT) -> HRESULT;
    }
);
