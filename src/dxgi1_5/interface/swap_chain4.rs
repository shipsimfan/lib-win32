use crate::{
    com_interface,
    dxgi::{
        IDXGIDeviceSubObject, IDXGIObject,
        IDXGISwapChain,
    },
    dxgi1_2::{IDXGISwapChain1},
    dxgi1_3::{IDXGISwapChain2},
    dxgi1_4::{IDXGISwapChain3},
    dxgi1_5::DXGI_HDR_METADATA_TYPE,
    unknwn::{IUnknown},
    HRESULT, UINT,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_5::DXGI_HDR_METADATA_HDR10;

com_interface!(
    /// This interface exposes a single method for setting video metadata.
    pub abstract IDXGISwapChain4(IDXGISwapChain4VTable):
        IDXGISwapChain3(swap_chain3) +
        IDXGISwapChain2 +
        IDXGISwapChain1 +
        IDXGISwapChain +
        IDXGIDeviceSubObject +
        IDXGIObject +
        IUnknown {
        const IID = 0x3D585D5A-0xBD4A-0x489E-0xB1F4-0x3DBCB6452FFB;

        /// This method sets High Dynamic Range (HDR) and Wide Color Gamut (WCG) header metadata.
        ///
        /// # Parameters
        ///  * `r#type` - Specifies one member of the [`DXGI_HDR_METADATA_TYPE`] enum.
        ///  * `size` - Specifies the size of `metadata`, in bytes.
        ///  * `metadata` - Specifies a void pointer that references the metadata, if it exists.
        ///                 Refer to the [`DXGI_HDR_METADATA_HDR10`] structure.
        ///
        /// # Return Value
        /// This method returns an [`HRESULT`] success or error code.
        ///
        /// # Remarks
        /// This method sets metadata to enable a monitor's output to be adjusted depending on its
        /// capabilities. However it does not change how pixel values are interpreted by Windows or
        /// monitors. To adjust the color space of the swap chain, use
        /// [`IDXGISwapChain3::set_color_space1`] instead.
        ///
        /// Applications should not rely on the metadata being sent to the monitor as the metadata
        /// may be ignored. Monitors do not consistently process HDR metadata, resulting in varied
        /// appearance of your content across different monitors. In order to ensure more
        /// consistent output across a range of monitors, devices, and use cases, it is recommended
        /// to not use [`IDXGISwapChain4::set_hdr_metadata`] and to instead tone-map content into
        /// the gamut and luminance range supported by the monitor. Monitors adhering to the VESA
        /// DisplayHDR standard will automatically perform a form of clipping for content outside
        /// of the monitor's supported gamut and luminance range.
        fn set_hdr_metadata(
            &mut self,
            r#type: DXGI_HDR_METADATA_TYPE,
            size: UINT,
            metadata: *mut c_void
        ) -> HRESULT;
    }
);
