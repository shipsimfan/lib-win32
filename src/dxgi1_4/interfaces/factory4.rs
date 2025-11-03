use crate::{
    com_interface,
    dxgi::{IDXGIFactory, IDXGIFactory1, IDXGIObject},
    dxgi1_2::IDXGIFactory2,
    dxgi1_3::IDXGIFactory3,
    unknwn::IUnknown,
    HRESULT, LUID, REFIID,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi::IDXGIAdapter, S_OK};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// Enables creating Microsoft DirectX Graphics Infrastructure (DXGI) objects.
    pub abstract IDXGIFactory4(IDXGIFactory4VTable):
        IDXGIFactory3(factory3) +
        IDXGIFactory2 +
        IDXGIFactory1 +
        IDXGIFactory +
        IDXGIObject +
        IUnknown {
        const IID = 0x1BC6EA02-0xEF36-0x464F-0xBF0C-0x21CA39E5168A;

        /// Outputs the [`IDXGIAdapter`] for the specified LUID.
        ///
        /// # Parameters
        ///  * `adapter_luid` - A unique value that identifies the adapter. See [`LUID`] for a
        ///                     definition of the structure.
        ///  * `iid` - The globally unique identifier (GUID) of the [`IDXGIAdapter`] object
        ///            referenced by the `adapter` parameter.
        ///  * `adapter` - The address of an [`IDXGIAdapter`] interface pointer to the adapter.
        ///                This parameter must not be [`null_mut`].
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// # Remarks
        /// For Direct3D 12, it's no longer possible to backtrack from a device to the
        /// [`IDXGIAdapter`] that was used to create it. [`IDXGIFactory4::enum_adapter_by_luid`]
        /// enables an app to retrieve information about the adapter where a D3D12 device was
        /// created. [`IDXGIFactory4::enum_adapter_by_luid`] is designed to be paired with
        /// [`ID3D12Device::get_adapter_luid`].
        fn enum_adapter_by_luid(
            &mut self,
            adapter_luid: LUID,
            iid: REFIID,
            adapter: *mut *mut c_void
        ) -> HRESULT;

        /// Provides an adapter which can be provided to [`D3D12CreateDevice`] to use the WARP
        /// renderer.
        ///
        /// # Parameters
        ///  * `iid` - The globally unique identifier (GUID) of the [`IDXGIAdapter`] object
        ///            referenced by the `adapter` parameter.
        ///  * `adapter` - The address of an [`IDXGIAdapter`] interface pointer to the adapter.
        ///                This parameter must not be [`null_mut`].
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn enum_warp_adapter(&mut self, iid: REFIID, adapter: *mut *mut c_void) -> HRESULT;
    }
);
