use crate::{
    com_interface,
    dxgi::{IDXGIObject, IDXGIObjectTrait, IDXGIOutput, DXGI_ADAPTER_DESC},
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, LARGE_INTEGER, REFGUID, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::IDXGIFactory, DXGI_ERROR_NOT_FOUND, DXGI_ERROR_UNSUPPORTED, E_INVALIDARG, GUID, S_OK,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// The [`IDXGIAdapter`] interface represents a display subsystem (including one or more GPUs,
    /// DACs and video memory).
    ///
    /// # Remarks
    /// A display subsystem is often referred to as a video card, however, on some machines the
    /// display subsystem is part of the motherboard.
    ///
    /// To enumerate the display subsystems, use [`IDXGIFactory::enum_adapters`].
    ///
    /// To get an interface to the adapter for a particular device, use
    /// [`IDXGIDevice::get_adapter`].
    ///
    /// To create a software adapter, use [`IDXGIFactory::create_software_adapter`].
    ///
    /// Windows Phone 8: This API is supported.
    pub abstract IDXGIAdapter(IDXGIAdapterVTable/IDXGIAdapterTrait):
        IDXGIObject/IDXGIObjectTrait(object) +
        IUnknown/IUnknownTrait(object.unknown) {
        const IID = 0x2411E7E1-0x12AC-0x4CCF-0xBD14-0x9798E8534DC0;

        /// Enumerate adapter (video card) outputs.
        ///
        /// # Parameters
        ///  * `output` - The index of the output.
        ///  * `p_output` - The address of a pointer to an [`IDXGIOutput`] interface at the
        ///                 position specified by the `output` parameter.
        ///
        /// # Return Value
        /// A code that indicates success or failure. [`DXGI_ERROR_NOT_FOUND`] is returned if the
        /// index is greater than the number of outputs.
        ///
        /// If the adapter came from a device created using [`D3D_DRIVER_TYPE_WARP`], then the
        /// adapter has no outputs, so [`DXGI_ERROR_NOT_FOUND`] is returned.
        ///
        /// # Remarks
        /// When the [`IDXGIAdapter::enum_outputs`] method succeeds and fills the `p_output`
        /// parameter with the address of the pointer to the output interface,
        /// [`IDXGIAdapter::enum_outputs`] increments the output interface's reference count. To
        /// avoid a memory leak, when you finish using the output interface, call the
        /// [`IDXGIAdapter::release`] method to decrement the reference count.
        ///
        /// [`IDXGIAdapter::enum_outputs`] first returns the output on which the desktop primary is
        /// displayed. This output corresponds with an index of zero.
        /// [`IDXGIAdapter::enum_outputs`] then returns other outputs.
        fn enum_outputs(&mut self, output: UINT, p_output: *mut *mut IDXGIOutput) -> HRESULT;

        /// Gets a DXGI 1.0 description of an adapter (or video card).
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`DXGI_ADAPTER_DESC`] structure that describes the adapter.
        ///             This parameter must not be [`null_mut`]. On feature level 9 graphics
        ///             hardware, [`IDXGIAdapter::get_desc`] returns zeros for `vendor_id`,
        ///             `device_id`, `sub_sys_id`, and `revision` members of [`DXGI_ADAPTER_DESC`]
        ///             and “Software Adapter” for the description string in the `description`
        ///             member.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise returns [`E_INVALIDARG`] if the `desc`
        /// parameter is [`null_mut`].
        fn get_desc(&mut self, desc: *mut DXGI_ADAPTER_DESC) -> HRESULT;

        /// Checks whether the system supports a device interface for a graphics component.
        ///
        /// # Parameters
        ///  * `interface_name` - The [`GUID`] of the interface of the device version for which
        ///                       support is being checked. This should usually be
        ///                       [`IDXGIDevice::IID`], which returns the version number of the
        ///                       Direct3D 9 UMD (user mode driver) binary. Since WDDM 2.3, all
        ///                       driver components within a driver package (D3D9, D3D11, and
        ///                       D3D12) have been required to share a single version number, so
        ///                       this is a good way to query the driver version regardless of
        ///                       which API is being used.
        ///  * `umd_version` - The user mode driver version of `interface_name`. This is returned
        ///                    only if the interface is supported, otherwise this parameter will be
        ///                    [`null_mut`].
        ///
        /// # Return Value
        /// [`S_OK`] indicates that the interface is supported, otherwise
        /// [`DXGI_ERROR_UNSUPPORTED`] is returned.
        fn check_interface_support(&mut self, interface_name: REFGUID, umd_version: *mut LARGE_INTEGER) -> HRESULT;
    }
);
