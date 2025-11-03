use crate::{
    com_interface,
    dxgi::{IDXGIFactory, IDXGIFactory1, IDXGIObject},
    dxgi1_2::IDXGIFactory2,
    dxgi1_3::IDXGIFactory3,
    dxgi1_4::IDXGIFactory4,
    dxgi1_5::DXGI_FEATURE,
    unknwn::IUnknown,
    HRESULT, UINT,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::DXGI_SWAP_CHAIN_FLAG;

com_interface!(
    /// This interface enables a single method to support variable refresh rate displays.
    pub abstract IDXGIFactory5(IDXGIFactory5VTable):
        IDXGIFactory4(factory4) +
        IDXGIFactory3 +
        IDXGIFactory2 +
        IDXGIFactory1 +
        IDXGIFactory +
        IDXGIObject +
        IUnknown {
        const IID = 0x7632E1F5-0xEE65-0x4DCA-0x87FD-0x84CD75F8838D;

        /// Used to check for hardware feature support.
        ///
        /// # Parameters
        ///  * `feature` - Specifies one member of [`DXGI_FEATURE`] to query support for.
        ///  * `feature_support_data` - Specifies a pointer to a buffer that will be filled with
        ///                             data that describes the feature support.
        ///  * `feature_support_data_size` - The size, in bytes, of `feature_support_data`.
        ///
        /// # Return Value
        /// This method returns an [`HRESULT`] success or error code.
        ///
        /// # Remarks
        /// Refer to the description of [`DXGI_SWAP_CHAIN_FLAG::AllowTearing`].
        fn check_feature_support(
            &mut self,
            feature: DXGI_FEATURE,
            feature_support_data: *mut c_void,
            feature_support_data_size: UINT
        ) -> HRESULT;
    }
);
