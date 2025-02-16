use crate::{
    com_interface,
    dxgi::{IDXGIObject, IDXGIObjectTrait},
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, REFIID,
};
use std::ffi::c_void;

com_interface!(
    /// Inherited from objects that are tied to the device so that they can retrieve a pointer to it.
    pub abstract IDXGIDeviceSubObject(IDXGIDeviceSubObjectVTable/IDXGIDeviceSubObjectTrait):
        IDXGIObject/IDXGIObjectTrait(object) +
        IUnknown/IUnknownTrait(object.unknown) {
        const IID = 0x3D3E0379-0xF9DE-0x4D58-0xBB6C-0x18D62992F1A6;

        /// Retrieves the device.
        ///
        /// # Parameters
        ///  * `iid` - The reference id for the device.
        ///  * `device` - The address of a pointer to the device.
        ///
        /// # Return Value
        /// A code that indicates success or failure.
        fn get_device(&mut self, iid: REFIID, device: *mut *mut c_void) -> HRESULT;
    }
);
