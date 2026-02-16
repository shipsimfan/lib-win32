use crate::{CCHDEVICENAME, CCHFORMNAME, DWORD, POINTL, WCHAR, WORD};
use std::ffi::c_short;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    DEVMODE, DMCOLLATE_FALSE, DMCOLLATE_TRUE, DMCOLOR_COLOR, DMCOLOR_MONOCHROME, DMDUP_HORIZONTAL,
    DMDUP_SIMPLEX, DMDUP_VERTICAL, DM_ORIENTATION, DM_SPECVERSION,
};

/// The [`DEVMODEW`] structure is used for specifying characteristics of display and print devices
/// in the Unicode (wide) character set.
///
/// # Remarks
/// The [`DEVMODEW`] structure is the Unicode version of the [`DEVMODE`] structure (described in
/// the Microsoft Windows SDK documentation). While applications can use either the ANSI or Unicode
/// version of the structure, drivers are required to use the Unicode version.
///
/// For printer drivers, the [`DEVMODEW`] structure is used for specifying printer characteristics
/// required by a print document. It is also used for specifying a printer's default
/// characteristics.
///
/// Immediately following a [`DEVMODEW`] structure's defined members (often referred to as its
/// public members), there can be a set of driver-defined members (often referred to as private
/// [`DEVMODEW`] members). The driver supplies the size, in bytes, of this private area in
/// `driver_extra`. Driver-defined private members are for exclusive use by the driver. The
/// starting address for the private members can be referenced using the `size` member as follows:
///
/// ```no_run
/// let driver_data: PVOID = (dm as usize + unsafe { (*dm).size }) as PVOID;
/// ```
///
/// A driver can rely on the spooler to pass a [`DEVMODEW`] buffer that is no smaller than
/// `size + driver_extra` bytes. As a result, the driver can safely read that number of bytes
/// starting from the beginning of the buffer without causing an access violation, and without
/// needing to probe memory.
///
/// Prior to playing EMF, GDI calls the spooler to validate the contents of the public portion of
/// the [`DEVMODEW`] buffer. If the [`DEVMODEW`] buffer does not pass the validation tests
/// performed in the spooler, GDI does not pass the buffer on to the printer driver.
///
/// In Windows 2000, a new union member was added to the [`DEVMODEW`] structure. This union member
/// contains an existing [`DEVMODEW`] structure member, `display_flags`, together with a new
/// member, `nup`.
///
/// In Windows XP, a new struct member was added. This struct member contains an existing
/// [`DEVMODEW`] structure member, `position`, together with two new members, `display_orientation`
/// and `display_fixed_output`.
///
/// Also for Windows XP, several members of the [`DEVMODEW`] structure were moved to different
/// locations in this structure. The `scale`, `copies`, `default_source`, and `print_quality`
/// members were appended to the struct member containing the `orientation`, `paper_size`,
/// `paper_length`, and `paper_width` members.
#[repr(C)]
#[derive(Clone)]
pub struct DEVMODEW {
    /// For a display, specifies the name of the display driver's DLL; for example, "perm3dd" for
    /// the 3Dlabs Permedia3 display driver.
    ///
    /// For a printer, specifies the "friendly name"; for example, "PCL/HP LaserJet" in the case of
    /// PCL/HP LaserJet. If the name is greater than [`CCHDEVICENAME`] characters in length, the
    /// spooler truncates it to fit in the array.
    pub device_name: [WCHAR; CCHDEVICENAME],

    /// Specifies the version number of this [`DEVMODEW`] structure. The current version number is
    /// identified by the [`DM_SPECVERSION`] constant in [`wingdi`](crate::wingdi).
    pub spec_version: WORD,

    /// For a printer, specifies the printer driver version number assigned by the printer driver
    /// developer.
    ///
    /// Display drivers can set this member to [`DM_SPECVERSION`].
    pub driver_version: WORD,

    /// Specifies the size in bytes of the public [`DEVMODEW`] structure, not including any
    /// private, driver-specified members identified by the `driver_extra` member.
    pub size: WORD,

    /// Specifies the number of bytes of private driver data that follow the public structure
    /// members. If a device driver does not provide private [`DEVMODEW`] members, this member
    /// should be set to zero.
    pub driver_extra: WORD,

    /// Specifies bit flags identifying which of the following [`DEVMODEW`] members are in use. For
    /// example, the [`DM_ORIENTATION`] flag is set when the `orientation` member contains valid
    /// data. The `DM_XXX` flags are defined in [`wingdi`](crate::wingdi).
    pub fields: DWORD,

    #[allow(missing_docs)]
    pub dummy: DEVMODEW_UNION,

    /// For printers, specifies whether a color printer should print color or monochrome. This
    /// member can be one of [`DMCOLOR_COLOR`] or [`DMCOLOR_MONOCHROME`].
    ///
    /// This member is not used for displays.
    pub color: c_short,

    /// For printers, specifies duplex (double-sided) printing for duplex-capable printers. This
    /// member can be one of the following values:
    ///  * [`DMDUP_HORIZONTAL`] - Print double-sided, using short edge binding.
    ///  * [`DMDUP_SIMPLEX`] - Print single-sided.
    ///  * [`DMDUP_VERTICAL`] - Print double-sided, using long edge binding.
    ///
    /// This member is not used for displays.
    pub duplex: c_short,

    /// For printers, specifies the y resolution of the printer, in DPI. If this member is used,
    /// the `print_quality` member specifies the x resolution.
    ///
    /// This member is not used for displays.
    pub y_resolution: c_short,

    /// For printers, specifies how TrueType fonts should be printed. This member must be one of
    /// the DMTT-prefixed constants defined in [`wingdi`](crate::wingdi).
    ///
    /// This member is not used for displays.
    pub tt_option: c_short,

    /// For printers, specifies whether multiple copies should be collated. This member can be one
    /// of the following values:
    ///  * [`DMCOLLATE_TRUE`] - Collate when printing multiple copies.
    ///  * [`DMCOLLATE_FALSE`] - Do not collate when printing multiple copies.
    ///
    /// This member is not used for displays.
    pub collate: c_short,

    /// For printers, specifies the name of the form to use; such as "Letter" or "Legal". This must
    /// be a name that can be obtain by calling the Win32 [`EnumForms`] function.
    ///
    /// This member is not used for displays.
    pub form_name: [WCHAR; CCHFORMNAME],

    /// For displays, specifies the number of logical pixels per inch of a display device and
    /// should be equal to the `log_pixels` member of the [`GDIINFO`] structure.
    ///
    /// This member is not used for printers.
    pub log_pixels: WORD,

    /// For displays, specifies the color resolution, in bits per pixel, of a display device.
    ///
    /// This member is not used for printers.
    pub bits_per_pel: DWORD,

    /// For displays, specifies the width, in pixels, of the visible device surface.
    ///
    /// This member is not used for printers.
    pub pels_width: DWORD,

    /// For displays, specifies the height, in pixels, of the visible device surface.
    ///
    /// This member is not used for printers.
    pub pels_height: DWORD,

    #[allow(missing_docs)]
    pub dummy2: DEVMODEW_UNION2,

    /// For displays, specifies the frequency, in hertz, of a display device in its current mode.
    ///
    /// This member is not used for printers.
    pub display_frequency: DWORD,

    /// Specifies one of the `DMICMMETHOD`-prefixed constants defined in [`wingdi`](crate::wingdi).
    pub icm_method: DWORD,

    /// Specifies one of the `DMICM`-prefixed constants defined in [`wingdi`](crate::wingdi).
    pub icm_intent: DWORD,

    /// Specifies one of the `DMMEDIA`-prefixed constants defined in [`wingdi`](crate::wingdi).
    pub media_type: DWORD,

    /// Specifies one of the `DMDITHER`-prefixed constants defined in [`wingdi`](crate::wingdi).
    pub dither_type: DWORD,

    /// Is reserved for system use and should be ignored by the driver.
    pub reserved1: DWORD,

    /// Is reserved for system use and should be ignored by the driver.
    pub reserved2: DWORD,

    /// Is reserved for system use and should be ignored by the driver.
    pub panning_width: DWORD,

    /// Is reserved for system use and should be ignored by the driver.
    pub panning_height: DWORD,
}

impl Default for DEVMODEW {
    fn default() -> Self {
        DEVMODEW {
            device_name: [0; CCHDEVICENAME],
            spec_version: 0,
            driver_version: 0,
            size: 0,
            driver_extra: 0,
            fields: 0,
            dummy: DEVMODEW_UNION::default(),
            color: 0,
            duplex: 0,
            y_resolution: 0,
            tt_option: 0,
            collate: 0,
            form_name: [0; CCHFORMNAME],
            log_pixels: 0,
            bits_per_pel: 0,
            pels_width: 0,
            pels_height: 0,
            dummy2: DEVMODEW_UNION2::default(),
            display_frequency: 0,
            icm_method: 0,
            icm_intent: 0,
            media_type: 0,
            dither_type: 0,
            reserved1: 0,
            reserved2: 0,
            panning_width: 0,
            panning_height: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub union DEVMODEW_UNION {
    #[allow(missing_docs)]
    pub dummy: DEVMODEW_STRUCT,

    #[allow(missing_docs)]
    pub position: POINTL,

    #[allow(missing_docs)]
    pub dummy2: DEVMODEW_STRUCT2,
}

impl Default for DEVMODEW_UNION {
    fn default() -> Self {
        DEVMODEW_UNION {
            dummy: DEVMODEW_STRUCT::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub struct DEVMODEW_STRUCT {
    /// For printers, specifies the paper orientation. This member can be either DMORIENT_PORTRAIT or DMORIENT_LANDSCAPE.
    ///
    /// This member is not used for displays.
    pub orientation: c_short,

    /// For printers, specifies the size of the paper to be printed on. This member must be zero if the length and width of the paper are specified by the dmPaperLength and dmPaperWidth members. Otherwise, the dmPaperSize member must be one of the DMPAPER-prefixed constants defined in [`wingdi`](crate::wingdi).
    ///
    /// This member is not used for displays.
    pub paper_size: c_short,

    /// For printers, specifies the length of the paper, in units of 1/10 of a millimeter. This value overrides the length of the paper specified by the dmPaperSize member, and is used if the paper is of a custom size, or if the device is a dot matrix printer, which can print a page of arbitrary length.
    ///
    /// This member is not used for displays.
    pub paper_length: c_short,

    /// For printers, specifies the width of the paper, in units of 1/10 of a millimeter. This value overrides the width of the paper specified by the dmPaperSize member. This member must be used if dmPaperLength is used.
    ///
    /// This member is not used for displays.
    pub paper_width: c_short,

    /// For printers, specifies the percentage by which the image is to be scaled for printing. The image's page size is scaled to the physical page by a factor of dmScale/100. For example, a 17-inch by 22-inch image with a scale value of 100 requires 17x22-inch paper, while the same image with a scale value of 50 should print as half-sized and fit on letter-sized paper.
    ///
    /// This member is not used for displays.
    pub scale: c_short,

    /// For printers, specifies the number of copies to be printed, if the device supports multiple copies.
    ///
    /// This member is not used for displays.
    pub copies: c_short,

    /// For printers, specifies the printer's default input bin. This must be one of the DMBIN-prefixed constants defined in [`wingdi`](crate::wingdi). If the specified constant is DMBIN_FORMSOURCE, the input bin should be selected automatically.
    ///
    /// This member is not used for displays.
    pub default_source: c_short,

    /// For printers, specifies the printer resolution. The following negative constant values are defined in [`wingdi`](crate::wingdi):
    ///  - [`DMRES_HIGH`]
    ///  - [`DMRES_MEDIUM`]
    ///  - [`DMRES_LOW`]
    ///  - [`DMRES_DRAFT`]
    ///
    /// If a positive value is specified, it represents the number of dots per inch (DPI) for the x resolution, and the y resolution is specified by dmYResolution.
    ///
    /// This member is not used for displays.
    pub print_quality: c_short,
}

impl Default for DEVMODEW_STRUCT {
    fn default() -> Self {
        DEVMODEW_STRUCT {
            orientation: 0,
            paper_size: 0,
            paper_length: 0,
            paper_width: 0,
            scale: 0,
            copies: 0,
            default_source: 0,
            print_quality: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub struct DEVMODEW_STRUCT2 {
    /// For displays, specifies a POINTL structure containing the x- and y-coordinates of upper-left corner of the display, in desktop coordinates. This member is used to determine the relative position of monitors in a multiple monitor environment.
    ///
    /// This member is not used for printers.
    pub position: POINTL,

    /// This member is defined only for Windows XP and later.
    ///
    /// For displays, specifies the orientation at which images should be presented. When the DM_DISPLAYORIENTATION bit is not set in the dmFields member, this member must be set to zero. When the DM_DISPLAYORIENTATION bit is set in the dmFields member, this member must be set to one of the following values:
    ///  * [`DMDO_DEFAULT`] - The current mode's display device orientation is the natural orientation of the device, and should be used as the default.
    ///  * [`DMDO_90`] - The display device orientation is 90 degrees (measured counter-clockwise) from that of DMDO_DEFAULT.
    ///  * [`DMDO_180`] - The display device orientation is 180 degrees (measured counter-clockwise) from that of DMDO_DEFAULT.
    ///  * [`DMDO_270`] - The display device orientation is 270 degrees (measured counter-clockwise) from that of DMDO_DEFAULT.
    ///
    /// This member is not used for printers.
    pub display_orientation: DWORD,

    /// This member is defined only for Windows XP and later.
    ///
    /// For fixed-resolution displays, specifies how the device can present a lower-resolution mode on a higher-resolution display. For example, if a display device's resolution is fixed at 1024 X 768, and its mode is set to 640 x 480, the device can either display a 640 X 480 image within the 1024 X 768 screen space, or stretch the 640 X 480 image to fill the larger screen space.
    ///
    /// When the DM_DISPLAYFIXEDOUTPUT bit is not set in the dmFields member, this member must be set to zero. When the DM_DISPLAYFIXEDOUTPUT bit is set in the dmFields member, this member must be set to one of the following values:
    ///  * [`DMDFO_CENTER`] - The display device presents a lower resolution mode image by centering it in the larger screen space.
    ///  * [`DMDFO_STRETCH`] - The display device presents a lower-resolution mode image by stretching it to fill the larger screen space.
    ///
    /// This member is not used for printers.
    pub display_fixed_output: DWORD,
}

impl Default for DEVMODEW_STRUCT2 {
    fn default() -> Self {
        DEVMODEW_STRUCT2 {
            position: POINTL::default(),
            display_orientation: 0,
            display_fixed_output: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub union DEVMODEW_UNION2 {
    /// Specifies the device's display mode. This member can be a combination of the following values:
    ///  * [`DM_GRAYSCALE`] - Specifies that the display is a noncolor device. If this flag is not set, color is assumed. This flag is no longer valid.
    ///  * [`DM_INTERLACED`] - Specifies that the display mode is interlaced. If the flag is not set, noninterlaced is assumed.
    ///
    /// Display drivers use this member; for example, in the ChangeDisplaySettings function. Printer drivers don't use this member.
    pub display_flags: DWORD,

    /// For printers, specifies whether the print system handles "N-up" printing (playing multiple EMF logical pages onto a single physical page). The value of this member can be one of the following:
    ///  * [`DMNUP_SYSTEM`] -  The print system handles "N-up" printing.
    ///  * [`DMNUP_ONEUP`] -  The print system does not handle "N-up" printing. An application can set dmNup to DMNUP_ONEUP if it intends to carry out "N-up" printing on its own.
    ///
    /// This member is not used for displays.
    pub nup: DWORD,
}

impl Default for DEVMODEW_UNION2 {
    fn default() -> Self {
        DEVMODEW_UNION2 { display_flags: 0 }
    }
}
