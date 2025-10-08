use crate::{
    com_interface,
    dxgi::{IDXGIDeviceSubObject, IDXGIDeviceSubObjectTrait, IDXGIObject, IDXGIObjectTrait},
    unknwn::{IUnknown, IUnknownTrait},
    DWORD, HRESULT, UINT64,
};

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d10")]
use crate::d3d10::D3D10_RESOURCE_MISC_FLAG;
#[allow(unused_imports)]
#[cfg(feature = "d3d11")]
use crate::d3d11::D3D11_RESOURCE_MISC_FLAG;
#[allow(unused_imports)]
use crate::{dxgi::IDXGIFactory1, E_FAIL, INFINITE, SUCCEEDED, S_OK, WAIT_ABANDONED, WAIT_TIMEOUT};

com_interface!(
    /// Represents a keyed mutex, which allows exclusive access to a shared resource that is used
    /// by multiple devices.
    ///
    /// # Remarks
    /// The [`IDXGIFactory1`] is required to create a resource capable of supporting the
    /// [`IDXGIKeyedMutex`] interface.
    ///
    /// An [`IDXGIKeyedMutex`] should be retrieved for each device sharing a resource. In Direct3D
    /// 10.1, such a resource that is shared between two or more devices is created with the
    /// [`D3D10_RESOURCE_MISC_FLAG::SharedKeyedMutex`] flag. In Direct3D 11, such a resource that
    /// is shared between two or more devices is created with the
    /// [`D3D11_RESOURCE_MISC_FLAG::SharedKeyedMutex`] flag.
    ///
    /// For information about creating a keyed mutex, see the [`IDXGIKeyedMutex::acquire_sync`]
    /// method.
    pub abstract IDXGIKeyedMutex(IDXGIKeyedMutexVTable/IDXGIKeyedMutexTrait):
        IDXGIDeviceSubObject/IDXGIDeviceSubObjectTrait(device_sub_object) +
        IDXGIObject/IDXGIObjectTrait(device_sub_object.object) +
        IUnknown/IUnknownTrait(device_sub_object.object.unknown) {
        const IID = 0x9D8E1289-0xD7B3-0x465F-0x8126-0x250E349AF85D;

        /// Using a key, acquires exclusive rendering access to a shared resource.
        ///
        /// # Parameters
        ///  * `key` - A value that indicates which device to give access to. This method will
        ///            succeed when the device that currently owns the surface calls the
        ///            [`IDXGIKeyedMutex::release_sync`] method using the same value. This value
        ///            can be any [`UINT64`] value.
        ///  * `milliseconds` - The time-out interval, in milliseconds. This method will return if
        ///                     the interval elapses, and the keyed mutex has not been released
        ///                     using the specified Key. If this value is set to zero, the
        ///                     [`IDXGIKeyedMutex::acquire_sync`] method will test to see if the
        ///                     keyed mutex has been released and returns immediately. If this
        ///                     value is set to [`INFINITE`], the time-out interval will never
        ///                     elapse.
        ///
        /// # Return Value
        /// Return [`S_OK`] if successful.
        ///
        /// If the owning device attempted to create another keyed mutex on the same shared
        /// resource, [`IDXGIKeyedMutex::acquire_sync`] returns [`E_FAIL`].
        ///
        /// [`IDXGIKeyedMutex::acquire_sync`] can also return the following [`DWORD`] constants.
        /// Therefore, you should explicitly check for these constants. If you only use the
        /// [`SUCCEEDED`] macro on the return value to determine if
        /// [`IDXGIKeyedMutex::acquire_sync`] succeeded, you will not catch these constants.
        ///  * [`WAIT_ABANDONED`] - The shared surface and keyed mutex are no longer in a
        ///                         consistent state. If [`IDXGIKeyedMutex::acquire_sync`] returns
        ///                         this value, you should release and recreate both the keyed
        ///                         mutex and the shared surface.
        ///  * [`WAIT_TIMEOUT`] - The time-out interval elapsed before the specified key was
        ///                       released.
        ///
        /// # Remarks
        /// The [`IDXGIKeyedMutex::acquire_sync`] method creates a lock to a surface that is shared
        /// between multiple devices, allowing only one device to render to a surface at a time.
        /// This method uses a key to determine which device currently has exclusive access to the
        /// surface.
        ///
        /// When a surface is created using the [`D3D10_RESOURCE_MISC_FLAG::SharedKeyedMutex`]
        /// value of the [`D3D10_RESOURCE_MISC_FLAG`] enumeration, you must call the
        /// [`IDXGIKeyedMutex::acquire_sync`] method before rendering to the surface. You must call
        /// the [`IDXGIKeyedMutex::release_sync`] method when you are done rendering to a surface.
        ///
        /// To acquire a reference to the keyed mutex object of a shared resource, call the
        /// [`IUnknown::query_interface`] method of the resource and pass in the UUID of the
        /// [`IDXGIKeyedMutex`] interface.
        ///
        /// The [`IDXGIKeyedMutex::acquire_sync`] method uses the key as follows, depending on the
        /// state of the surface:
        ///  - On initial creation, the surface is unowned and any device can call the
        ///    [`IDXGIKeyedMutex::acquire_sync`] method to gain access. For an unowned device, only
        ///    a key of 0 will succeed. Calling the [`IDXGIKeyedMutex::acquire_sync`] method for
        ///    any other key will stall the calling CPU thread.
        ///  - If the surface is owned by a device when you call the
        ///    [`IDXGIKeyedMutex::acquire_sync`] method, the CPU thread that called the
        ///    [`IDXGIKeyedMutex::acquire_sync`] method will stall until the owning device calls
        ///    the [`IDXGIKeyedMutex::release_sync`] method using the same Key.
        ///  - If the surface is unowned when you call the [`IDXGIKeyedMutex::acquire_sync`] method
        ///    (for example, the last owning device has already called the
        ///    [`IDXGIKeyedMutex::release_sync`] method), the [`IDXGIKeyedMutex::acquire_sync`]
        ///    method will succeed if you specify the same key that was specified when the
        ///    [`IDXGIKeyedMutex::release_sync`] method was last called. Calling the
        ///    [`IDXGIKeyedMutex::acquire_sync`] method using any other key will cause a stall.
        ///  - When the owning device calls the [`IDXGIKeyedMutex::release_sync`] method with a
        ///    particular key, and more than one device is waiting after calling the
        ///    [`IDXGIKeyedMutex::acquire_sync`] method using the same key, any one of the waiting
        ///    devices could be woken up first. The order in which devices are woken up is
        ///    undefined.
        ///  - A keyed mutex does not support recursive calls to the
        ///    [`IDXGIKeyedMutex::acquire_sync`] method.
        fn acquire_sync(&mut self, key: UINT64, milliseconds: DWORD) -> HRESULT;

        /// Using a key, releases exclusive rendering access to a shared resource.
        ///
        /// # Parameters
        ///  * `key` - A value that indicates which device to give access to. This method succeeds
        ///            when the device that currently owns the surface calls the
        ///            [`IDXGIKeyedMutex::release_sync`] method using the same value. This value
        ///            can be any [`UINT64`] value.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful.
        ///
        /// If the device attempted to release a keyed mutex that is not valid or owned by the
        /// device, [`IDXGIKeyedMutex::release_sync`] returns [`E_FAIL`].
        ///
        /// # Remarks
        /// The [`IDXGIKeyedMutex::release_sync`] method releases a lock to a surface that is
        /// shared between multiple devices. This method uses a key to determine which device
        /// currently has exclusive access to the surface.
        ///
        /// When a surface is created using the [`D3D10_RESOURCE_MISC_FLAG::SharedKeyedMutex`]
        /// value of the [`D3D10_RESOURCE_MISC_FLAG`] enumeration, you must call the
        /// [`IDXGIKeyedMutex::acquire_sync`] method before rendering to the surface. You must call
        /// the [`IDXGIKeyedMutex::release_sync`] method when you are done rendering to a surface.
        ///
        /// After you call the [`IDXGIKeyedMutex::release_sync`] method, the shared resource is
        /// unset from the rendering pipeline.
        ///
        /// To acquire a reference to the keyed mutex object of a shared resource, call the
        /// [`IUnknown::query_interface`] method of the resource and pass in the UUID of the
        /// [`IDXGIKeyedMutex`] interface.
        fn release_sync(&mut self, key: UINT64) -> HRESULT;
    }
);
