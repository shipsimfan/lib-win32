use crate::{
    com_interface,
    dxgi::{IDXGIObject, IDXGIOutput, DXGI_FORMAT},
    dxgi1_2::{IDXGIOutput1},
    dxgi1_3::{IDXGIOutput2},
    unknwn::{IUnknown},
    HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_3::DXGI_OVERLAY_SUPPORT_FLAG;

com_interface!(
    /// Represents an adapter output (such as a monitor). The [`IDXGIOutput3`] interface exposes a
    /// method to check for overlay support.
    pub abstract IDXGIOutput3(IDXGIOutput3VTable):
        IDXGIOutput2(output2) +
        IDXGIOutput1 +
        IDXGIOutput +
        IDXGIObject +
        IUnknown {
        const IID = 0x8A6BB301-0x7E7E-0x41F4-0xA8E0-0x5B32F7F99B18;

        /// Checks for overlay support.
        ///
        /// # Parameters
        ///  * `enum_format` - A [`DXGI_FORMAT`]-typed value for the color format.
        ///  * `concerned_device` - A pointer to the Direct3D device interface.
        ///                         [`IDXGIOutput3::check_overlay_support`] returns only support
        ///                         info about this scan-out device.
        ///  * `flags` - A pointer to a variable that receives a combination of
        ///              [`DXGI_OVERLAY_SUPPORT_FLAG`]-typed values that are combined by using a
        ///              bitwise OR operation. The resulting value specifies options for overlay
        ///              support.
        ///
        /// # Return Value
        /// Returns one of the error codes described in the `DXGI_ERROR` topic.
        fn check_overlay_support(
            &mut self,
            enum_format: DXGI_FORMAT,
            concerned_device: *mut IUnknown,
            flags: *mut UINT
        ) -> HRESULT;
    }
);
