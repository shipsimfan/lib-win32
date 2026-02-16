use crate::HDC;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    ASPECTX, ASPECTXY, ASPECTY, BITSPIXEL, BLTALIGNMENT, CC_CHORD, CC_CIRCLES, CC_ELLIPSES,
    CC_INTERIORS, CC_NONE, CC_PIE, CC_ROUNDRECT, CC_STYLED, CC_WIDE, CC_WIDESTYLED, CLIPCAPS,
    CM_CMYK_COLOR, CM_DEVICE_ICM, CM_GAMMA_RAMP, CM_NONE, COLORMGMTCAPS, COLORRES, CURVECAPS,
    DRIVERVERSION, DT_CHARSTREAM, DT_DISPFILE, DT_METAFILE, DT_PLOTTER, DT_RASCAMERA,
    DT_RASDISPLAY, DT_RASPRINTER, HORZRES, HORZSIZE, LC_INTERIORS, LC_MARKER, LC_NONE, LC_POLYLINE,
    LC_POLYMARKER, LC_STYLED, LC_WIDE, LC_WIDESTYLED, LINECAPS, LOGPIXELSX, LOGPIXELSY, NUMBRUSHES,
    NUMCOLORS, NUMFONTS, NUMPENS, NUMRESERVED, PC_INTERIORS, PC_NONE, PC_POLYGON, PC_RECTANGLE,
    PC_SCANLINE, PC_STYLED, PC_WIDE, PC_WIDESTYLED, PC_WINDPOLYGON, PDEVICESIZE, PHYSICALHEIGHT,
    PHYSICALOFFSETX, PHYSICALOFFSETY, PHYSICALWIDTH, PLANES, POLYGONALCAPS, RASTERCAPS, RC_BANDING,
    RC_BITBLT, RC_BITMAP64, RC_DIBTODEV, RC_DI_BITMAP, RC_FLOODFILL, RC_PALETTE, RC_SCALING,
    RC_STRETCHBLT, RC_STRETCHDIB, SB_CONST_ALPHA, SB_GRAD_RECT, SB_GRAD_TRI, SB_NONE,
    SB_PIXEL_ALPHA, SB_PREMULT_ALPHA, SCALINGFACTORX, SCALINGFACTORY, SHADEBLENDCAPS, SIZEPALETTE,
    TC_CP_STROKE, TC_CR_90, TC_CR_ANY, TC_EA_DOUBLE, TC_IA_ABLE, TC_OP_CHARACTER, TC_OP_STROKE,
    TC_RA_ABLE, TC_RESERVED, TC_SA_CONTIN, TC_SA_DOUBLE, TC_SA_INTEGER, TC_SCROLLBLT,
    TC_SF_X_YINDEP, TC_SO_ABLE, TC_UA_ABLE, TC_VA_ABLE, TECHNOLOGY, TEXTCAPS, VERTRES, VERTSIZE,
    VREFRESH,
};

#[link(name = "Gdi32")]
extern "system" {
    /// The [`GetDeviceCaps`] function retrieves device-specific information for the specified
    /// device.
    ///
    /// # Parameters
    ///  * `dc` - A handle to the DC.
    ///  * `index` - The item to be returned. This parameter can be one of the following values:
    ///    * [`DRIVERVERSION`] - The device driver version.
    ///    * [`TECHNOLOGY`] - Device technology. If the `dc` parameter is a handle to the DC of an
    ///                       enhanced metafile, the device technology is that of the referenced
    ///                       device as specified to the [`CreateEnhMetaFile`] function. To
    ///                       determine whether it is an enhanced metafile DC, use the
    ///                       [`GetObjectType`] function. It can be any one of the following
    ///                       values:
    ///      * [`DT_PLOTTER`] - Vector plotter
    ///      * [`DT_RASDISPLAY`] - Raster display
    ///      * [`DT_RASPRINTER`] - Raster printer
    ///      * [`DT_RASCAMERA`] - Raster camera
    ///      * [`DT_CHARSTREAM`] - Character stream
    ///      * [`DT_METAFILE`] - Metafile
    ///      * [`DT_DISPFILE`] - Display file
    ///    * [`HORZSIZE`] - Width, in millimeters, of the physical screen.
    ///    * [`VERTSIZE`] - Height, in millimeters, of the physical screen.
    ///    * [`HORZRES`] - Width, in pixels, of the screen; or for printers, the width, in pixels,
    ///                    of the printable area of the page.
    ///    * [`VERTRES`] - Height, in raster lines, of the screen; or for printers, the height, in
    ///                    pixels, of the printable area of the page.
    ///    * [`LOGPIXELSX`] - Number of pixels per logical inch along the screen width. In a system
    ///                       with multiple display monitors, this value is the same for all
    ///                       monitors.
    ///    * [`LOGPIXELSY`] - Number of pixels per logical inch along the screen height. In a
    ///                       system with multiple display monitors, this value is the same for all
    ///                       monitors.
    ///    * [`BITSPIXEL`] - Number of adjacent color bits for each pixel.
    ///    * [`PLANES`] - Number of color planes.
    ///    * [`NUMBRUSHES`] - Number of device-specific brushes.
    ///    * [`NUMPENS`] - Number of device-specific pens.
    ///    * [`NUMFONTS`] - Number of device-specific fonts.
    ///    * [`NUMCOLORS`] - Number of entries in the device's color table, if the device has a
    ///                      color depth of no more than 8 bits per pixel. For devices with greater
    ///                      color depths, -1 is returned.
    ///    * [`ASPECTX`] - Relative width of a device pixel used for line drawing.
    ///    * [`ASPECTY`] - Relative height of a device pixel used for line drawing.
    ///    * [`ASPECTXY`] - Diagonal width of the device pixel used for line drawing.
    ///    * [`PDEVICESIZE`] - Reserved.
    ///    * [`CLIPCAPS`] - Flag that indicates the clipping capabilities of the device. If the
    ///                     device can clip to a rectangle, it is 1. Otherwise, it is 0.
    ///    * [`SIZEPALETTE`] - Number of entries in the system palette. This index is valid only if
    ///                        the device driver sets the [`RC_PALETTE`] bit in the [`RASTERCAPS`]
    ///                        index and is available only if the driver is compatible with 16-bit
    ///                        Windows.
    ///    * [`NUMRESERVED`] - Number of reserved entries in the system palette. This index is
    ///                        valid only if the device driver sets the [`RC_PALETTE`] bit in the
    ///                        [`RASTERCAPS`] index and is available only if the driver is
    ///                        compatible with 16-bit Windows.
    ///    * [`COLORRES`] - Actual color resolution of the device, in bits per pixel. This index is
    ///                     valid only if the device driver sets the [`RC_PALETTE`] bit in the
    ///                     [`RASTERCAPS`] index and is available only if the driver is compatible
    ///                     with 16-bit Windows.
    ///    * [`PHYSICALWIDTH`] - For printing devices: the width of the physical page, in device
    ///                          units. For example, a printer set to print at 600 dpi on
    ///                          8.5-x11-inch paper has a physical width value of 5100 device
    ///                          units. Note that the physical page is almost always greater than
    ///                          the printable area of the page, and never smaller.
    ///    * [`PHYSICALHEIGHT`] - For printing devices: the height of the physical page, in device
    ///                           units. For example, a printer set to print at 600 dpi on
    ///                           8.5-by-11-inch paper has a physical height value of 6600 device
    ///                           units. Note that the physical page is almost always greater than
    ///                           the printable area of the page, and never smaller.
    ///    * [`PHYSICALOFFSETX`] - For printing devices: the distance from the left edge of the
    ///                            physical page to the left edge of the printable area, in device
    ///                            units. For example, a printer set to print at 600 dpi on
    ///                            8.5-by-11-inch paper, that cannot print on the leftmost
    ///                            0.25-inch of paper, has a horizontal physical offset of 150
    ///                            device units.
    ///    * [`PHYSICALOFFSETY`] - For printing devices: the distance from the top edge of the
    ///                            physical page to the top edge of the printable area, in device
    ///                            units. For example, a printer set to print at 600 dpi on
    ///                            8.5-by-11-inch paper, that cannot print on the topmost 0.5-inch
    ///                            of paper, has a vertical physical offset of 300 device units.
    ///    * [`VREFRESH`] - For display devices: the current vertical refresh rate of the device,
    ///                     in cycles per second (Hz). A vertical refresh rate value of 0 or 1
    ///                     represents the display hardware's default refresh rate. This default
    ///                     rate is typically set by switches on a display card or computer
    ///                     motherboard, or by a configuration program that does not use display
    ///                     functions such as [`ChangeDisplaySettings`].
    ///    * [`SCALINGFACTORX`] - Scaling factor for the x-axis of the printer.
    ///    * [`SCALINGFACTORY`] - Scaling factor for the y-axis of the printer.
    ///    * [`BLTALIGNMENT`] - Preferred horizontal drawing alignment, expressed as a multiple of
    ///                         pixels. For best drawing performance, windows should be
    ///                         horizontally aligned to a multiple of this value. A value of zero
    ///                         indicates that the device is accelerated, and any alignment may be
    ///                         used.
    ///    * [`SHADEBLENDCAPS`] - Value that indicates the shading and blending capabilities of the
    ///                           device.
    ///      * [`SB_CONST_ALPHA`] - Handles the `source_constant_alpha` member of the
    ///                             [`BLENDFUNCTION`] structure, which is referenced by the
    ///                             `blend_function` parameter of the [`AlphaBlend`] function.
    ///      * [`SB_GRAD_RECT`] - Capable of doing [`GradientFill`] rectangles.
    ///      * [`SB_GRAD_TRI`] - Capable of doing [`GradientFill`] triangles.
    ///      * [`SB_NONE`] - Device does not support any of these capabilities.
    ///      * [`SB_PIXEL_ALPHA`] - Capable of handling per-pixel alpha in [`AlphaBlend`].
    ///      * [`SB_PREMULT_ALPHA`] - Capable of handling premultiplied alpha in [`AlphaBlend`].
    ///    * [`RASTERCAPS`] - Value that indicates the raster capabilities of the device, as shown
    ///                       in the following table:
    ///      * [`RC_BANDING`] - Requires banding support.
    ///      * [`RC_BITBLT`] - Capable of transferring bitmaps.
    ///      * [`RC_BITMAP64`] - Capable of supporting bitmaps larger than 64 KB.
    ///      * [`RC_DI_BITMAP`] - Capable of supporting the [`SetDIBits`] and [`GetDIBits`]
    ///                           functions.
    ///      * [`RC_DIBTODEV`] - Capable of supporting the [`SetDIBitsToDevice`] function.
    ///      * [`RC_FLOODFILL`] - Capable of performing flood fills.
    ///      * [`RC_PALETTE`] - Specifies a palette-based device.
    ///      * [`RC_SCALING`] - Capable of scaling.
    ///      * [`RC_STRETCHBLT`] - Capable of performing the [`StretchBlt`] function.
    ///      * [`RC_STRETCHDIB`] - Capable of performing the [`StretchDIBits`] function.
    ///    * [`CURVECAPS`] - Value that indicates the curve capabilities of the device, as shown in
    ///                      the following table:
    ///      * [`CC_NONE`] - Device does not support curves.
    ///      * [`CC_CHORD`] - Device can draw chord arcs.
    ///      * [`CC_CIRCLES`] - Device can draw circles.
    ///      * [`CC_ELLIPSES`] - Device can draw ellipses.
    ///      * [`CC_INTERIORS`] - Device can draw interiors.
    ///      * [`CC_PIE`] - Device can draw pie wedges.
    ///      * [`CC_ROUNDRECT`] - Device can draw rounded rectangles.
    ///      * [`CC_STYLED`] - Device can draw styled borders.
    ///      * [`CC_WIDE`] - Device can draw wide borders.
    ///      * [`CC_WIDESTYLED`] - Device can draw borders that are wide and styled.
    ///    * [`LINECAPS`] - Value that indicates the line capabilities of the device, as shown in
    ///                     the following table:
    ///      * [`LC_NONE`] - Device does not support lines.
    ///      * [`LC_INTERIORS`] - Device can draw interiors.
    ///      * [`LC_MARKER`] - Device can draw a marker.
    ///      * [`LC_POLYLINE`] - Device can draw a polyline.
    ///      * [`LC_POLYMARKER`] - Device can draw multiple markers.
    ///      * [`LC_STYLED`] - Device can draw styled lines.
    ///      * [`LC_WIDE`] - Device can draw wide lines.
    ///      * [`LC_WIDESTYLED`] - Device can draw lines that are wide and styled.
    ///    * [`POLYGONALCAPS`] - Value that indicates the polygon capabilities of the device, as
    ///                          shown in the following table:
    ///      * [`PC_NONE`] - Device does not support polygons.
    ///      * [`PC_INTERIORS`] - Device can draw interiors.
    ///      * [`PC_POLYGON`] - Device can draw alternate-fill polygons.
    ///      * [`PC_RECTANGLE`] - Device can draw rectangles.
    ///      * [`PC_SCANLINE`] - Device can draw a single scanline.
    ///      * [`PC_STYLED`] - Device can draw styled borders.
    ///      * [`PC_WIDE`] - Device can draw wide borders.
    ///      * [`PC_WIDESTYLED`] - Device can draw borders that are wide and styled.
    ///      * [`PC_WINDPOLYGON`] - Device can draw winding-fill polygons.
    ///    * [`TEXTCAPS`] - Value that indicates the text capabilities of the device, as shown in
    ///                     the following table:
    ///      * [`TC_OP_CHARACTER`] - Device is capable of character output precision.
    ///      * [`TC_OP_STROKE`] - Device is capable of stroke output precision.
    ///      * [`TC_CP_STROKE`] - Device is capable of stroke clip precision.
    ///      * [`TC_CR_90`] - Device is capable of 90-degree character rotation.
    ///      * [`TC_CR_ANY`] - Device is capable of any character rotation.
    ///      * [`TC_SF_X_YINDEP`] - Device can scale independently in the x- and y-directions.
    ///      * [`TC_SA_DOUBLE`] - Device is capable of doubled character for scaling.
    ///      * [`TC_SA_INTEGER`] - Device uses integer multiples only for character scaling.
    ///      * [`TC_SA_CONTIN`] - Device uses any multiples for exact character scaling.
    ///      * [`TC_EA_DOUBLE`] - Device can draw double-weight characters.
    ///      * [`TC_IA_ABLE`] - Device can italicize.
    ///      * [`TC_UA_ABLE`] - Device can underline.
    ///      * [`TC_SO_ABLE`] - Device can draw strikeouts.
    ///      * [`TC_RA_ABLE`] - Device can draw raster fonts.
    ///      * [`TC_VA_ABLE`] - Device can draw vector fonts.
    ///      * [`TC_RESERVED`] - Reserved; must be zero.
    ///      * [`TC_SCROLLBLT`] - Device cannot scroll using a bit-block transfer. Note that this
    ///                           meaning may be the opposite of what you expect.
    ///    * [`COLORMGMTCAPS`] - Value that indicates the color management capabilities of the
    ///                          device.
    ///      * [`CM_CMYK_COLOR`] - Device can accept CMYK color space ICC color profile.
    ///      * [`CM_DEVICE_ICM`] - Device can perform ICM on either the device driver or the device
    ///                            itself.
    ///      * [`CM_GAMMA_RAMP`] - Device supports [`GetDeviceGammaRamp`] and
    ///                            [`SetDeviceGammaRamp`]
    ///      * [`CM_NONE`] - Device does not support ICM.
    ///
    /// # Return Value
    /// The return value specifies the value of the desired item.
    ///
    /// When `index` is [`BITSPIXEL`] and the device has 15bpp or 16bpp, the return value is 16.
    ///
    /// # Remarks
    /// When `index` is [`SHADEBLENDCAPS`]:
    ///  - For a printer, [`GetDeviceCaps`] returns whatever the printer reports.
    ///  - For a display device, all blending operations are available; besides [`SB_NONE`], the
    ///    only return values are [`SB_CONST_ALPHA`] and [`SB_PIXEL_ALPHA`], which indicate whether
    ///    these operations are accelerated.
    ///
    /// On a multiple monitor system, if `dc` is the desktop, [`GetDeviceCaps`] returns the
    /// capabilities of the primary monitor. If you want info for other monitors, you must use the
    /// multi-monitor APIs or [`CreateDC`] to get a [`HDC`] for the device context (DC) of a
    /// specific monitor.
    ///
    /// [`GetDeviceCaps`] provides the following six indexes in place of printer escapes:
    ///  * [`PHYSICALWIDTH`] - [`GETPHYSPAGESIZE`]
    ///  * [`PHYSICALHEIGHT`] - [`GETPHYSPAGESIZE`]
    ///  * [`PHYSICALOFFSETX`] - [`GETPRINTINGOFFSET`]
    ///  * [`PHYSICALOFFSETY`] - [`GETPHYSICALOFFSET`]
    ///  * [`SCALINGFACTORX`] - [`GETSCALINGFACTOR`]
    ///  * [`SCALINGFACTORY`] - [`GETSCALINGFACTOR`]
    pub fn GetDeviceCaps(dc: HDC, index: c_int) -> c_int;
}
