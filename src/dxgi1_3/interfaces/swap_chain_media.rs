use crate::{
    com_interface,
    dxgi1_3::DXGI_FRAME_STATISTICS_MEDIA,
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::S_OK;

com_interface!(
    /// This swap chain interface allows desktop media applications to request a seamless change to
    /// a specific refresh rate.
    ///
    /// For example, a media application presenting video at a typical framerate of 23.997 frames
    /// per second can request a custom refresh rate of 24 or 48 Hz to eliminate jitter. If the
    /// request is approved, the app starts presenting frames at the custom refresh rate
    /// immediately - without the typical 'mode switch' a user would experience when changing the
    /// refresh rate themselves by using the control panel.
    ///
    /// # Remarks
    /// Seamless changes to custom framerates can only be done on integrated panels. Custom frame
    /// rates cannot be applied to external displays. If the DXGI output adapter is attached to an
    /// external display then [`IDXGISwapChainMedia::check_present_duration_support`] will return
    /// (0, 0) for upper and lower bounds, indicating that the device does not support seamless
    /// refresh rate changes.
    ///
    /// Custom refresh rates can be used when displaying video with a dynamic framerate. However,
    /// the refresh rate change should be kept imperceptible to the user. A best practice for
    /// keeping the refresh rate transition imperceptible is to only set the custom framerate if
    /// the app determines it can present at that rate for least 5 seconds.
    pub abstract IDXGISwapChainMedia(IDXGISwapChainMediaVTable/IDXGISwapChainMediaTrait):
        IUnknown/IUnknownTrait(unknown) {
        const IID = 0xDD95B90B-0xF05F-0x4F6A-0xBD65-0x25BFB264BD84;

        /// Queries the system for a [`DXGI_FRAME_STATISTICS_MEDIA`] structure that indicates
        /// whether a custom refresh rate is currently approved by the system.
        ///
        /// # Parameters
        ///  * `stats` - A [`DXGI_FRAME_STATISTICS_MEDIA`] structure indicating whether the system
        ///              currently approves the custom refresh rate request.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or a DXGI error code on failure.
        fn get_frame_statistics_media(&mut self, stats: DXGI_FRAME_STATISTICS_MEDIA) -> HRESULT;

        /// Requests a custom presentation duration (custom refresh rate).
        ///
        /// # Parameters
        ///  * `duration` - The custom presentation duration, specified in hundreds of nanoseconds.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or a DXGI error code on failure.
        fn set_present_duration(&mut self, duration: UINT) -> HRESULT;

        /// Queries the graphics driver for a supported frame present duration corresponding to a
        /// custom refresh rate.
        ///
        /// # Parameters
        ///  * `desired_present_duration` - Indicates the frame duration to check. This value is
        ///                                 the duration of one frame at the desired refresh rate,
        ///                                 specified in hundreds of nanoseconds. For example, set
        ///                                 this field to 167777 to check for 60 Hz refresh rate
        ///                                 support.
        ///  * `closest_smaller_present_duration` - A variable that will be set to the closest
        ///                                         supported frame present duration that's smaller
        ///                                         than the requested value, or zero if the device
        ///                                         does not support any lower duration.
        ///  * `closest_larger_present_duration` - A variable that will be set to the closest
        ///                                        supported frame present duration that's larger
        ///                                        than the requested value, or zero if the device
        ///                                        does not support any higher duration.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or a DXGI error code on failure.
        ///
        /// # Remarks
        /// If the DXGI output adapter does not support custom refresh rates (for example, an
        /// external display) then the display driver will set upper and lower bounds to (0, 0).
        fn check_present_duration_support(
            &mut self,
            desired_present_duration: UINT,
            closest_smaller_present_duration: *mut UINT,
            closest_larger_present_duration: *mut UINT
        ) -> HRESULT;
    }
);
