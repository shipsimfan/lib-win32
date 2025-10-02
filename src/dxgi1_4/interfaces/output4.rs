use crate::{
    com_interface,
    dxgi::{
        IDXGIObject, IDXGIObjectTrait, IDXGIOutput, IDXGIOutputTrait, DXGI_COLOR_SPACE_TYPE,
        DXGI_FORMAT,
    },
    dxgi1_2::{IDXGIOutput1, IDXGIOutput1Trait},
    dxgi1_3::{IDXGIOutput2, IDXGIOutput2Trait, IDXGIOutput3, IDXGIOutput3Trait},
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_4::DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG;

com_interface!(
    /// Represents an adapter output (such as a monitor). The [`IDXGIOutput4`] interface exposes a
    /// method to check for overlay color space support.
    pub abstract IDXGIOutput4(IDXGIOutput4VTable/IDXGIOutput4Trait):
        IDXGIOutput3/IDXGIOutput3Trait(output3) +
        IDXGIOutput2/IDXGIOutput2Trait(output3.output2) +
        IDXGIOutput1/IDXGIOutput1Trait(output3.output2.output1) +
        IDXGIOutput/IDXGIOutputTrait(output3.output2.output1.output) +
        IDXGIObject/IDXGIObjectTrait(output3.output2.output1.output.object) +
        IUnknown/IUnknownTrait(output3.output2.output1.output.object.unknown) {
        const IID = 0xDC7DCA35-0x2196-0x414D-0x9F53-0x617884032A60;

        /// Checks for overlay color space support.
        ///
        /// # Parameters
        ///  * `format` - A [`DXGI_FORMAT`]-typed value for the color format.
        ///  * `color_space` - A [`DXGI_COLOR_SPACE_TYPE`]-typed value that specifies color space
        ///                    type to check overlay support for.
        ///  * `concerned_device` - A pointer to the Direct3D device interface.
        ///                         [`IDXGIOutput4::check_overlay_color_space_support`] returns
        ///                         only support info about this scan-out device.
        ///  * `flags` - A pointer to a variable that receives a combination of
        ///              [`DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG`]-typed values that are combined
        ///              by using a bitwise OR operation. The resulting value specifies options for
        ///              overlay color space support.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or it returns one of the error codes that are
        /// described in the `DXGI_ERROR` topic.
        fn check_overlay_color_space_support(
            &mut self,
            format: DXGI_FORMAT,
            color_space: DXGI_COLOR_SPACE_TYPE,
            concerned_device: *mut IUnknown,
            flags: *mut UINT
        ) -> HRESULT;
    }
);
