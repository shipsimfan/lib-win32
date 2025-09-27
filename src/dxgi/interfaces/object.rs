use crate::{
    com_interface,
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, REFGUID, REFIID, UINT,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::GUID;

com_interface!(
    /// An [`IDXGIObject`] interface is a base interface for all DXGI objects; [`IDXGIObject`]
    /// supports associating caller-defined (private data) with an object and retrieval of an
    /// interface to the parent object.
    pub abstract IDXGIObject(IDXGIObjectVTable/IDXGIObjectTrait): IUnknown/IUnknownTrait(unknown) {
        const IID = 0xAEC22FB8-0x76F3-0x4639-0x9BE0-0x28EB43A67A2E;

        /// Sets application-defined data to the object and associates that data with a [`GUID`].
        ///
        /// # Parameters
        ///  * `name` - A [`GUID`] that identifies the data. Use this [`GUID`] in a call to
        ///             `get_private_data` to get the data.
        ///  * `data_size` - The size of the object's data.
        ///  * `data` - A pointer to the object's data.
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        ///
        /// # Remarks
        /// `set_private_data` makes a copy of the specified data and stores it with the object.
        ///
        /// Private data that `set_private_data` stores in the object occupies the same storage
        /// space as private data that is stored by associated Direct3D objects (for example, by a
        /// Microsoft Direct3D 11 device through [`ID3D11Device::set_private_data`] or by a
        /// Direct3D 11 child device through [`ID3D11DeviceChild::set_private_data`]).
        ///
        /// The debug layer reports memory leaks by outputting a list of object interface pointers
        /// along with their friendly names. The default friendly name is "\<unnamed>". You can set
        /// the friendly name so that you can determine if the corresponding object interface
        /// pointer caused the leak. To set the friendly name, use the `set_private_data` method
        /// and the well-known private data [`GUID`] ([`WKPDID_D3DDebugObjectName`]) that is in
        /// [`d3dcommon`].
        ///
        /// You can use [`WKPDID_D3DDebugObjectName`] to track down memory leaks and understand
        /// performance characteristics of your applications. This information is reflected in the
        /// output of the debug layer that is related to memory leaks
        /// ([`ID3D11Debug::report_live_device_objects`]) and with the event tracing for Windows
        /// events that we've added to Windows 8.
        fn set_private_data(
            &mut self,
            name: REFGUID,
            data_size: UINT,
            data: *const c_void
        ) -> HRESULT;

        /// Set an interface in the object's private data.
        ///
        /// # Parameters
        ///  * `name` - A [`GUID`] identifying the interface.
        ///  * `unknown` - The interface to set.
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        ///
        /// # Remarks
        /// This API associates an interface pointer with the object.
        ///
        /// When the interface is set its reference count is incremented. When the data are
        /// overwritten (by calling SPD or SPDI with the same GUID) or the object is destroyed,
        /// `::release()` is called and the interface's reference count is decremented.
        fn set_private_data_interface(
            &mut self,
            name: REFGUID,
            unknown: *const IUnknown
        ) -> HRESULT;

        /// Get a pointer to the object's data.
        ///
        /// # Parameters
        ///  * `name` - A [`GUID`] identifying the data.
        ///  * `data_size` - The size of the data.
        ///  * `data` - Pointer to the data.
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        ///
        /// # Remarks
        /// If the data returned is a pointer to an [`IUnknown`], or one of its derivative classes,
        /// previously set by [`IDXGIObject::set_private_data_interface`], you must call
        /// `::release()` on the pointer before the pointer is freed to decrement the reference
        /// count.
        ///
        /// You can pass [`GUID_DeviceType`] in the Name parameter of `get_private_data` to
        /// retrieve the device type from the display adapter object ([`IDXGIAdapter`],
        /// [`IDXGIAdapter1`], [`IDXGIAdapter2`]).
        ///
        /// To get the type of device on which the display adapter was created
        ///  1. Call [`IUnknown::query_interface`] on the [`ID3D11Device`] or [`ID3D10Device`]
        ///     object to retrieve the [`IDXGIDevice`] object.
        ///  2. Call `get_parent` on the [`IDXGIDevice`] object to retrieve the [`IDXGIAdapter`]
        ///     object.
        ///  3. Call `get_private_data` on the [`IDXGIAdapter`] object with [`GUID_DeviceType`] to
        ///     retrieve the type of device on which the display adapter was created. `data` will
        ///     point to a value from the driver-type enumeration (for example, a value from
        ///     [`D3D_DRIVER_TYPE`]).
        ///
        /// On Windows 7 or earlier, this type is either a value from [`D3D10_DRIVER_TYPE`] or
        /// [`D3D_DRIVER_TYPE`] depending on which kind of device was created. On Windows 8, this
        /// type is always a value from [`D3D_DRIVER_TYPE`]. Don't use
        /// `IDXGIObject::set_private_data` with [`GUID_DeviceType`] because the behavior when
        /// doing so is undefined.
        fn get_private_data(
            &mut self,
            name: REFGUID,
            data_size: *mut UINT,
            data: *mut c_void
        ) -> HRESULT;

        /// Gets the parent of the object.
        ///
        /// # Parameters
        ///  * `riid` - The ID of the requested interface.
        ///  * `parent` - The address of a pointer to the parent object.
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        fn get_parent(&mut self, riid: REFIID, parent: *mut *mut c_void) -> HRESULT;
    }
);
