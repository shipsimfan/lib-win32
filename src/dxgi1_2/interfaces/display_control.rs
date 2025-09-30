use crate::{
    com_interface,
    unknwn::{IUnknown, IUnknownTrait},
    BOOL,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGIFactory, IDXGIFactory1},
    dxgi1_2::IDXGIFactory2,
    FALSE, TRUE,
};

com_interface!(
    /// The [`IDXGIDisplayControl`] interface exposes methods to indicate user preference for the
    /// operating system's stereoscopic 3D display behavior and to set stereoscopic 3D display
    /// status to enable or disable.
    ///
    /// We recommend that you not use [`IDXGIDisplayControl`] to query or set system-wide
    /// stereoscopic 3D settings in your stereoscopic 3D apps. Instead, for your windowed apps,
    /// call the [`IDXGIFactory2::is_windowed_stereo_enabled`] method to determine whether to
    /// render in stereo; for your full-screen apps, call the
    /// [`IDXGIOutput1::get_display_mode_list1`] method and then determine whether any of the
    /// returned display modes support rendering in stereo.
    ///
    /// # Remarks
    /// Call [`IUnknown::query_interface`] from a factory object ([`IDXGIFactory`],
    /// [`IDXGIFactory1`] or [`IDXGIFactory2`]) to retrieve the [`IDXGIDisplayControl`] interface.
    ///
    /// The operating system processes changes to stereo-enabled configuration asynchronously.
    /// Therefore, these changes might not be immediately visible in every process that calls
    /// [`IDXGIDisplayControl::is_stereo_enabled`] to query for stereo configuration. Control
    /// applets can use the [`IDXGIFactory2::register_stereo_status_event`] or
    /// [`IDXGIFactory2::register_stereo_status_window`] method to register for notifications of
    /// all stereo configuration changes.
    pub abstract IDXGIDisplayControl(IDXGIDisplayControlVTable/IDXGIDisplayControlTrait):
        IUnknown/IUnknownTrait(unknown) {
        const IID = 0xEA9DBF1A-0xC88E-0x4486-0x854A-0x98AA0138F30C;

        /// Retrieves a Boolean value that indicates whether the operating system's stereoscopic 3D
        /// display behavior is enabled.
        ///
        /// # Return Value
        /// [`IDXGIDisplayControl::is_stereo_enabled`] returns [`TRUE`] when the operating system's
        /// stereoscopic 3D display behavior is enabled and [`FALSE`] when this behavior is
        /// disabled.
        ///
        /// # Remarks
        /// You pass a Boolean value to the [`IDXGIDisplayControl::set_stereo_enabled`] method to
        /// either enable or disable the operating system's stereoscopic 3D display behavior.
        /// [`TRUE`] enables the operating system's stereoscopic 3D display behavior and [`FALSE`]
        /// disables it.
        fn is_stereo_enabled(&mut self) -> BOOL;

        /// Set a Boolean value to either enable or disable the operating system's stereoscopic 3D
        /// display behavior.
        ///
        /// # Parameters
        ///  * `enabled` - A Boolean value that either enables or disables the operating system's
        ///                stereoscopic 3D display behavior. [`TRUE`] enables the operating
        ///                system's stereoscopic 3D display behavior and [`FALSE`] disables it.
        fn set_stereo_enabled(&mut self, enabled: BOOL);
    }
);
