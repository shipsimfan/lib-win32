/// The [`DISPLAYCONFIG_SCALING`] enumeration specifies the scaling transformation applied to
/// content displayed on a video present network (VidPN) present path.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DISPLAYCONFIG_SCALING {
    /// Indicates the identity transformation; the source content is presented with no change. This
    /// transformation is available only if the path's source mode has the same spatial resolution
    /// as the path's target mode.
    Identity = 1,

    /// Indicates the centering transformation; the source content is presented unscaled, centered
    /// with respect to the spatial resolution of the target mode.
    Centered = 2,

    /// Indicates the content is scaled to fit the path's target.
    Stretched = 3,

    /// Indicates the aspect-ratio centering transformation.
    AspectRatioCenteredMax = 4,

    /// Indicates that the caller requests a custom scaling that the caller cannot describe with
    /// any of the other `DISPLAYCONFIG_SCALING::XXX` values. Only a hardware vendor's value-add
    /// application should use [`DISPLAYCONFIG_SCALING::Custom`], because the value-add application
    /// might require a private interface to the driver. The application can then use
    /// [`DISPLAYCONFIG_SCALING::Custom`] to indicate additional context for the driver for the
    /// custom value on the specified path.
    Custom = 5,

    /// Indicates that the caller does not have any preference for the scaling. The
    /// [`SetDisplayConfig`] function will use the scaling value that was last saved in the
    /// database for the path. If such a scaling value does not exist, [`SetDisplayConfig`] will
    /// use the default scaling for the computer. For example, stretched
    /// ([`DISPLAYCONFIG_SCALING::Stretched`]) for tablet computers and aspect-ratio centered
    /// ([`DISPLAYCONFIG_SCALING::AspectRatioCenteredMax`]) for non-tablet computers.
    Preferred = 128,

    /// Forces this enumeration to compile to 32 bits in size. Without this value, some compilers
    /// would allow this enumeration to compile to a size other than 32 bits. You should not use
    /// this value.
    ForceUint32 = 0xFFFFFFFF,
}
