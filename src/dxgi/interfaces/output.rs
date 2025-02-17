use crate::{
    com_interface,
    dxgi::{
        IDXGIObject, IDXGIObjectTrait, IDXGISurface, DXGI_FORMAT, DXGI_FRAME_STATISTICS,
        DXGI_GAMMA_CONTROL, DXGI_GAMMA_CONTROL_CAPABILITIES, DXGI_MODE_DESC, DXGI_OUTPUT_DESC,
    },
    unknwn::{IUnknown, IUnknownTrait},
    BOOL, HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGISwapChain, DXGI_ENUM_MODES_SCALING, DXGI_USAGE_BACK_BUFFER},
    DXGI_ERROR_INVALID_CALL, DXGI_ERROR_MORE_DATA, DXGI_ERROR_NOT_CURRENTLY_AVAILABLE, FALSE, S_OK,
    TRUE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// An [`IDXGIOutput`] interface represents an adapter output (such as a monitor).
    pub abstract IDXGIOutput(IDXGIOutputVTable/IDXGIOutputTrait):
        IDXGIObject/IDXGIObjectTrait(object) +
        IUnknown/IUnknownTrait(object.unknown) {
        const IID = 0xAE02EEDB-0xC735-0x4690-0x8D52-0x5A8DC20213AA;

        /// Get a description of the output.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to the output description (see [`DXGI_OUTPUT_DESC`]).
        ///
        /// # Return Value
        /// Returns a code that indicates success or failure. [`S_OK`] if successful,
        /// [`DXGI_ERROR_INVALID_CALL`] if `desc` is passed in as [`null_mut`].
        ///
        /// # Remarks
        /// On a high DPI desktop, [`IDXGIOutput::get_desc`] returns the visualized screen size
        /// unless the app is marked high DPI aware.
        fn get_desc(&mut self, desc: *mut DXGI_OUTPUT_DESC) -> HRESULT;

        /// Gets the display modes that match the requested format and other input options.
        ///
        /// # Parameters
        ///  * `format` - The color format.
        ///  * `flags` - Options for modes to include. [`DXGI_ENUM_MODES_SCALING`] needs to be
        ///              specified to expose the display modes that require scaling. Centered
        ///              modes, requiring no scaling and corresponding directly to the display
        ///              output, are enumerated by default.
        ///  * `num_modes` - Set `desc` to [`null_mut`] so that `num_modes` returns the number of
        ///                  display modes that match the format and the options. Otherwise,
        ///                  `num_modes` returns the number of display modes returned in `desc`.
        ///  * `desc` - A pointer to a list of display modes (see [`DXGI_MODE_DESC`]); set to
        ///             [`null_mut`] to get the number of display modes.
        ///
        /// # Return Value
        /// Returns one of the following `DXGI_ERROR`. It is rare, but possible, that the display
        /// modes available can change immediately after calling this method, in which case
        /// [`DXGI_ERROR_MORE_DATA`] is returned (if there is not enough room for all the display
        /// modes).
        ///
        /// If [`IDXGIOutput::get_display_mode_list`] is called from a Remote Desktop Services
        /// session (formerly Terminal Services session), [`DXGI_ERROR_NOT_CURRENTLY_AVAILABLE`] is
        /// returned.
        ///
        /// # Remarks
        /// In general, when switching from windowed to full-screen mode, a swap chain
        /// automatically chooses a display mode that meets (or exceeds) the resolution, color
        /// depth and refresh rate of the swap chain. To exercise more control over the display
        /// mode, use this API to poll the set of display modes that are validated against monitor
        /// capabilities, or all modes that match the desktop (if the desktop settings are not
        /// validated against the monitor).
        fn get_display_mode_list(&mut self, format: DXGI_FORMAT, flags: UINT, num_modes: *mut UINT, desc: *mut DXGI_MODE_DESC) -> HRESULT;

        /// Finds the display mode that most closely matches the requested display mode.
        ///
        /// # Parameters
        ///  * `mode_to_match` - The desired display mode (see [`DXGI_MODE_DESC`]). Members of
        ///                      [`DXGI_MODE_DESC`] can be unspecified indicating no preference for
        ///                      that member. A value of 0 for `width` or `height` indicates the
        ///                      value is unspecified. If either `width` or `height` are 0, both
        ///                      must be 0. A numerator and denominator of 0 in `refresh_rate`
        ///                      indicate it is unspecified. Other members of [`DXGI_MODE_DESC`]
        ///                      have enumeration values indicating the member is unspecified. If
        ///                      `concerned_device` is [`null_mut`], Format cannot be
        ///                      [`DXGI_FORMAT::Unknown`].
        ///  * `closest_match` - The mode that most closely matches `mode_to_match`.
        ///  * `concerned_device` - A pointer to the Direct3D device interface. If this parameter is
        ///                         [`null_mut`], only modes whose format matches that of
        ///                         `mode_to_match` will be returned; otherwise, only those formats
        ///                         that are supported for scan-out by the device are returned.
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR`.
        ///
        /// # Remarks
        /// [`IDXGIOutput::find_closest_matching_mode`] behaves similarly to the
        /// [`IDXGIOutput1::find_closest_matching_mode1`] except
        /// [`IDXGIOutput::find_closest_matching_mode`] considers only the mono display modes.
        /// [`IDXGIOutput1::find_closest_matching_mode1`] considers only stereo modes if you set the
        /// `stereo` member in the [`DXGI_MODE_DESC1`] structure that `mode_to_match` points to,
        /// and considers only mono modes if `stereo` is not set.
        ///
        /// [`IDXGIOutput1::find_closest_matching_mode1`] returns a matched display-mode set with
        /// only stereo modes or only mono modes. [`IDXGIOutput::find_closest_matching_mode`]
        /// behaves as though you specified the input mode as mono.
        fn find_closest_matching_mode(&mut self, mode_to_match: *const DXGI_MODE_DESC, closest_match: *mut DXGI_MODE_DESC, concerned_device: *mut IUnknown) -> HRESULT;

        /// Halt a thread until the next vertical blank occurs.
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR`.
        ///
        /// # Remarks
        /// A vertical blank occurs when the raster moves from the lower right corner to the upper
        /// left corner to begin drawing the next frame.
        fn wait_for_vblank(&mut self) -> HRESULT;

        /// Takes ownership of an output.
        ///
        /// # Parameters
        ///  * `device` - A pointer to the [`IUnknown`] interface of a device (such as an
        ///               [`ID3D10Device`]).
        ///  * `exclusive` - Set to [`TRUE`] to enable other threads or applications to take
        ///                  ownership of the device; otherwise, set to [`FALSE`].
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        ///
        /// # Remarks
        /// When you are finished with the output, call [`IDXGIOutput::release_ownership`].
        ///
        /// [`IDXGIOutput::take_ownership`] should not be called directly by applications, since
        /// results will be unpredictable. It is called implicitly by the DXGI swap chain object
        /// during full-screen transitions, and should not be used as a substitute for swap-chain
        /// methods.
        fn take_ownership(&mut self, device: *mut IUnknown, exclusive: BOOL) -> HRESULT;

        /// Releases ownership of the output.
        ///
        /// # Remarks
        /// If you are not using a swap chain, get access to an output by calling
        /// [`IDXGIOutput::take_ownership`] and release it when you are finished by calling
        /// [`IDXGIOutput::release_ownership`]. An application that uses a swap chain will
        /// typically not call either of these methods.
        fn release_ownership(&mut self);

        /// Gets a description of the gamma-control capabilities.
        ///
        /// # Parameters
        ///  * `gamma_caps` - A pointer to a description of the gamma-control capabilities (see
        ///                   [`DXGI_GAMMA_CONTROL_CAPABILITIES`]).
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        fn get_gamma_control_capabilities(&mut self, gamma_caps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES) -> HRESULT;

        /// Sets the gamma controls.
        ///
        /// # Parameters
        ///  * `array` - A pointer to a [`DXGI_GAMMA_CONTROL`] structure that describes the gamma
        ///              curve to set.
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        fn set_gamma_control(&mut self, array: *mut DXGI_GAMMA_CONTROL) -> HRESULT;

        /// Gets the gamma control settings.
        ///
        /// # Parameters
        ///  * `array` - An array of gamma control settings (see [`DXGI_GAMMA_CONTROL`]).
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        fn get_gamma_control(&mut self, array: *mut DXGI_GAMMA_CONTROL) -> HRESULT;

        /// Changes the display mode.
        ///
        /// # Parameters
        ///  * `scanout_surface` - A pointer to a surface (see [`IDXGISurface`]) used for rendering
        ///                        an image to the screen. The surface must have been created as a
        ///                        back buffer ([`DXGI_USAGE_BACK_BUFFER`]).
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        ///
        /// # Remarks
        /// [`IDXGIOutput::set_display_surface`] should not be called directly by applications,
        /// since results will be unpredictable. It is called implicitly by the DXGI swap chain
        /// object during full-screen transitions, and should not be used as a substitute for
        /// swap-chain methods.
        ///
        /// This method should only be called between [`IDXGIOutput::take_ownership`] and
        /// [`IDXGIOutput::release_ownership`] calls.
        fn set_display_surface(&mut self, scanout_surface: *mut IDXGISurface) -> HRESULT;

        /// Gets a copy of the current display surface.
        ///
        /// # Parameters
        ///  * `destination` - A pointer to a destination surface (see [`IDXGISurface`]).
        ///
        /// # Return Value
        /// Returns one of the `DXGI_ERROR` values.
        ///
        /// # Remarks
        /// [`IDXGIOutput::get_display_surface_data`] can only be called when an output is in
        /// full-screen mode. If the method succeeds, DXGI fills the destination surface.
        ///
        /// Use [`IDXGIOutput::get_desc`] to determine the size (width and height) of the output
        /// when you want to allocate space for the destination surface. This is true regardless of
        /// target monitor rotation. A destination surface created by a graphics component (such as
        /// Direct3D 10) must be created with CPU-write permission
        /// (see [`D3D10_CPU_ACCESS_WRITE`]). Other surfaces should be created with CPU read-write
        /// permission (see [`D3D10_CPU_ACCESS_READ_WRITE`]). This method will modify the surface
        /// data to fit the destination surface (stretch, shrink, convert format, rotate). The
        /// stretch and shrink is performed with point-sampling.
        fn get_display_surface_data(&mut self, destination: *mut IDXGISurface) -> HRESULT;

        /// Gets statistics about recently rendered frames.
        ///
        /// # Parameters
        ///  * `stats` - A pointer to frame statistics (see [`DXGI_FRAME_STATISTICS`]).
        ///
        /// # Return Value
        /// If this function succeeds, it returns [`S_OK`]. Otherwise, it might return
        /// [`DXGI_ERROR_INVALID_CALL`].
        ///
        /// # Remarks
        /// This API is similar to [`IDXGISwapChain::get_frame_statistics`].
        fn get_frame_statistics(&mut self, stats: *mut DXGI_FRAME_STATISTICS) -> HRESULT;
    }
);
