/// The [`DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY`] enumeration specifies the target's connector
/// type.
///
/// # Remarks
/// Values with "embedded" in their names indicate that the graphics adapter's video output device
/// connects internally to the display device. In those cases, the
/// [`DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::Internal`] value is redundant. The caller should
/// ignore [`DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::Internal`] and just process the embedded
/// values, [`DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::DisplayPortEmbedded`] and
/// [`DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::UdiEmbedded`].
///
/// An embedded display port or UDI is also known as an integrated display port or UDI.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY {
    /// Indicates a connector that is not one of the types that is indicated by the following
    /// enumerators in this enumeration.
    Other = -1,

    /// Indicates an HD15 (VGA) connector.
    Hd15 = 0,

    /// Indicates an S-video connector.
    SVideo = 1,

    /// Indicates a composite video connector group.
    CompositeVideo = 2,

    /// Indicates a component video connector group.
    ComponentVideo = 3,

    /// Indicates a Digital Video Interface (DVI) connector.
    Dvi = 4,

    /// Indicates a High-Definition Multimedia Interface (HDMI) connector.
    Hdmi = 5,

    /// Indicates a Low Voltage Differential Swing (LVDS) connector.
    Lvds = 6,

    /// Indicates a Japanese D connector.
    DJpn = 8,

    /// Indicates an SDI connector.
    Sdi = 9,

    /// Indicates an external display port, which is a display port that connects externally to a
    /// display device.
    DisplayPortExternal = 10,

    /// Indicates an embedded display port that connects internally to a display device.
    DisplayPortEmbedded = 11,

    /// Indicates an external Unified Display Interface (UDI), which is a UDI that connects
    /// externally to a display device.
    UdiExternal = 12,

    /// Indicates an embedded UDI that connects internally to a display device.
    UdiEmbedded = 13,

    /// Indicates a dongle cable that supports standard definition television (SDTV).
    SdtvDongle = 14,

    /// Indicates that the VidPN target is a Miracast wireless display device.
    ///
    /// Supported starting in Windows 8.1.
    Miracast = 15,

    #[allow(missing_docs)]
    IndirectWired = 16,

    #[allow(missing_docs)]
    IndirectVirtual = 17,

    /// Indicates that the video output device connects internally to a display device (for
    /// example, the internal connection in a laptop computer).
    Internal = 0x80000000u32 as std::ffi::c_int as isize,
}
