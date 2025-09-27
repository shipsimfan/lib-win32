use crate::{
    com_interface,
    d3d11::ID3D11Device,
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, REFGUID, UINT,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

com_interface!(
    /// A device-child interface accesses data used by a device.
    ///
    /// # Remarks
    /// There are several types of device child interfaces, all of which inherit this interface.
    /// They include shaders, state objects, and input layouts.
    pub abstract ID3D11DeviceChild(ID3D11DeviceChildVTable/ID3D11DeviceChildTrait):
        IUnknown/IUnknownTrait(unknown) {
        const IID = 0x1841E5C8-0x16B0-0x489B-0xBCC8-0x44CFB0D5DEAE;

        /// Get a pointer to the device that created this interface.
        ///
        /// # Parameters
        ///  * `device` - Address of a pointer to a device (see [`ID3D11Device`]).
        ///
        /// # Remarks
        /// Any returned interfaces will have their reference count incremented by one, so be sure
        /// to call [`IUnknown::release`] on the returned pointer(s) before they are freed or
        /// else you will have a memory leak.
        fn get_device(&mut self, device: *mut *mut ID3D11Device);

        /// Get application-defined data from a device child.
        ///
        /// # Parameters
        ///  * `guid` - Guid associated with the data.
        ///  * `data_size` - A pointer to a variable that on input contains the size, in bytes, of
        ///                  the buffer that pData points to, and on output contains the size, in
        ///                  bytes, of the amount of data that
        ///                  [`ID3D11DeviceChild::get_private_data`] retrieved.
        ///  * `data` - A pointer to a buffer that [`ID3D11DeviceChild::get_private_data`] fills
        ///             with data from the device child if `data_size` points to a value that
        ///             specifies a buffer large enough to hold the data.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// The data stored in the device child is set by calling
        /// [`ID3D11DeviceChild::set_private_data`].
        ///
        /// If the data returned is a pointer to an [`IUnknown`], or one of its derivative classes,
        /// which was previously set by [`ID3D11DeviceChild::set_private_data_interface`], that
        /// interface will have its reference count incremented before the private data is
        /// returned.
        fn get_private_data(
            &mut self,
            guid: REFGUID,
            data_size: *mut UINT,
            data: *mut c_void
        ) -> HRESULT;

        /// Set application-defined data to a device child and associate that data with an
        /// application-defined guid.
        ///
        /// # Parameters
        ///  * `guid` - Guid associated with the data.
        ///  * `data_size` - Size of the data.
        ///  * `data` - Pointer to the data to be stored with this device child. If `data` is
        ///             [`null`], `data_size` must also be 0, and any data previously associated
        ///             with the specified guid will be destroyed.
        ///
        /// # Return Value
        /// This method returns one of the following Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// The data stored in the device child with this method can be retrieved with
        /// [`ID3D11DeviceChild::get_private_data`].
        ///
        /// The debug layer reports memory leaks by outputting a list of object interface pointers
        /// along with their friendly names. The default friendly name is "\<unnamed>". You can set
        /// the friendly name so that you can determine if the corresponding object interface
        /// pointer caused the leak. To set the friendly name, use the
        /// [`ID3D11DeviceChild::set_private_data`] method and the [`WKPDID_D3DDebugObjectName`]
        /// GUID that is in [`d3d_common`].
        fn set_private_data(
            &mut self,
            guid: REFGUID,
            data_size: UINT,
            data: *const c_void
        ) -> HRESULT;

        /// Associate an [`IUnknown`]-derived interface with this device child and associate that
        /// interface with an application-defined guid.
        ///
        /// # Parameters
        ///  * `guid` - Guid associated with the interface.
        ///  * `data` - A pointer to the [`IUnknown`]-derived interface to be associated with the
        ///             device child. Its reference count is incremented when set, and decremented
        ///             when either the [`ID3D11DeviceChild`] is destroyed, or when the data is
        ///             overwritten by calling [`ID3D11DeviceChild::set_private_data`] or
        ///             [`ID3D11DeviceChild::set_private_data_interface`] with the same GUID.
        ///
        /// # Return Value
        /// This method returns one of the following Direct3D 11 Return Codes.
        fn set_private_data_interface(&mut self, guid: REFGUID, data: *const IUnknown) -> HRESULT;
    }
);
