/// Identifies the dots per inch (dpi) setting for a monitor.
///
/// # Remarks
/// All of these settings are affected by the [`PROCESS_DPI_AWARENESS`] of your application
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum MONITOR_DPI_TYPE {
    /// The effective DPI. This value should be used when determining the correct scale factor for
    /// scaling UI elements. This incorporates the scale factor set by the user for this specific
    /// display.
    EffectiveDpi = 0,

    /// The angular DPI. This DPI ensures rendering at a compliant angular resolution on the
    /// screen. This does not include the scale factor set by the user for this specific display.
    AngularDpi = 1,

    /// The raw DPI. This value is the linear DPI of the screen as measured on the screen itself.
    /// Use this value when you want to read the pixel density and not the recommended scaling
    /// setting. This does not include the scale factor set by the user for this specific display
    /// and is not guaranteed to be a supported DPI value.
    RawDpi = 2,

    /// The default DPI setting for a monitor is [`MONITOR_DPI_TYPE::Default`].
    Default,
}
