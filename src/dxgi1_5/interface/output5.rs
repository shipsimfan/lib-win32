use crate::{
    com_interface,
    dxgi::{IDXGIObject, IDXGIObjectTrait, IDXGIOutput, IDXGIOutputTrait, DXGI_FORMAT},
    dxgi1_2::{IDXGIOutput1, IDXGIOutput1Trait, IDXGIOutputDuplication},
    dxgi1_3::{IDXGIOutput2, IDXGIOutput2Trait, IDXGIOutput3, IDXGIOutput3Trait},
    dxgi1_4::{IDXGIOutput4, IDXGIOutput4Trait},
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::IDXGIFactory1, dxgi1_5::DXGI_OUTDUPL_FLAG, DXGI_ERROR_NOT_CURRENTLY_AVAILABLE,
    DXGI_ERROR_SESSION_DISCONNECTED, DXGI_ERROR_UNSUPPORTED, E_ACCESSDENIED, E_INVALIDARG, S_OK,
};

com_interface!(
    /// Represents an adapter output (such as a monitor). The [`IDXGIOutput5`] interface exposes a
    /// single method to specify a list of supported formats for fullscreen surfaces.
    pub abstract IDXGIOutput5(IDXGIOutput5VTable/IDXGIOutput5Trait):
        IDXGIOutput4/IDXGIOutput4Trait(output4) +
        IDXGIOutput3/IDXGIOutput3Trait(output4.output3) +
        IDXGIOutput2/IDXGIOutput2Trait(output4.output3.output2) +
        IDXGIOutput1/IDXGIOutput1Trait(output4.output3.output2.output1) +
        IDXGIOutput/IDXGIOutputTrait(output4.output3.output2.output1.output) +
        IDXGIObject/IDXGIObjectTrait(output4.output3.output2.output1.output.object) +
        IUnknown/IUnknownTrait(output4.output3.output2.output1.output.object.unknown) {
        const IID = 0x80A07424-0xAB52-0x42EB-0x833C-0x0C42FD282D98;

        /// Allows specifying a list of supported formats for fullscreen surfaces that can be
        /// returned by the [`IDXGIOutputDuplication`] object.
        ///
        /// # Parameters
        ///  * `device` - A pointer to the Direct3D device interface that you can use to process
        ///               the desktop image. This device must be created from the adapter to which
        ///               the output is connected.
        ///  * `flags` - A bitfield of [`DXGI_OUTDUPL_FLAG`] enumeration values describing the kind
        ///              of capture surface to create.
        ///  * `supported_formats_count` - Specifies the number of supported formats.
        ///  * `supported_formats` - Specifies an array, of length `supported_formats_count` of
        ///                          [`DXGI_FORMAT`] entries.
        ///  * `output_duplication` - A pointer to a variable that receives the new
        ///                           [`IDXGIOutputDuplication`] interface.
        ///
        /// # Return Value
        ///  - [`S_OK`] if [`IDXGIOutput5::duplicate_output1`] successfully created the desktop
        ///             duplication interface.
        ///  - [`E_INVALIDARG`] for one of the following reasons:
        ///    - The specified device (pDevice) is invalid, was not created on the correct adapter,
        ///      or was not created from [`IDXGIFactory1`] (or a later version of a DXGI factory
        ///      interface that inherits from [`IDXGIFactory1`]).
        ///    - The calling application is already duplicating this desktop output.
        ///  - [`E_ACCESSDENIED`] if the application does not have access privilege to the current
        ///    desktop image. For example, only an application that runs at `LOCAL_SYSTEM` can
        ///    access the secure desktop.
        ///  - [`DXGI_ERROR_UNSUPPORTED`] if the created [`IDXGIOutputDuplication`] interface does
        ///    not support the current desktop mode or scenario. For example, 8bpp and non-DWM
        ///    desktop modes are not supported. If [`IDXGIOutput5::duplicate_output1`] fails with
        ///    [`DXGI_ERROR_UNSUPPORTED`], the application can wait for system notification of
        ///    desktop switches and mode changes and then call [`IDXGIOutput5::duplicate_output1`]
        ///    again after such a notification occurs.
        ///  - [`DXGI_ERROR_NOT_CURRENTLY_AVAILABLE`] if DXGI reached the limit on the maximum
        ///    number of concurrent duplication applications (default of four). Therefore, the
        ///    calling application cannot create any desktop duplication interfaces until the other
        ///    applications close.
        ///  - [`DXGI_ERROR_SESSION_DISCONNECTED`] if [`IDXGIOutput5::duplicate_output1`] failed
        ///    because the session is currently disconnected.
        ///  - Other error codes are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// This method allows directly receiving the original back buffer format used by a running
        /// fullscreen application. For comparison, using the original
        /// [`IDXGIOutput1::duplicate_output`] function always converts the fullscreen surface to a
        /// 32-bit BGRA format. In cases where the current fullscreen application is using a
        /// different buffer format, a conversion to 32-bit BGRA incurs a performance penalty.
        /// Besides the performance benefit of being able to skip format conversion, using
        /// [`IDXGIOutput5::duplicate_output1`] also allows receiving the full gamut of colors in
        /// cases where a high-color format (such as R10G10B10A2) is being presented.
        ///
        /// The `supported_formats` array should only contain display scan-out formats. If the
        /// current fullscreen buffer format is not contained in the `supported_formats` array,
        /// DXGI will pick one of the supplied formats and convert the fullscreen buffer to that
        /// format before returning from IDXGIOutputDuplication::AcquireNextFrame. The list of
        /// supported formats should always contain [`DXGI_FORMAT::B8G8R8A8UNorm`], as this is the
        /// most common format for the desktop.
        fn duplicate_output1(
            &mut self,
            device: *mut IUnknown,
            flags: UINT,
            supported_formats_count: UINT,
            supported_formats: *const DXGI_FORMAT,
            output_duplication: *mut *mut IDXGIOutputDuplication
        ) -> HRESULT;
    }
);
