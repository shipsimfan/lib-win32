use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    SetForegroundWindow, SystemParametersInfo, BOOL, CS_DROPSHADOW, DWORD,
    ERROR_OPERATION_IN_PROGRESS, FALSE, HIGHCONTRAST, MAX_PATH, PVOID, RECT, TRUE, ULONG,
    WM_MOUSEHOVER,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Retrieves information about the time-out period associated with the accessibility features. The
/// `pv_param` parameter must point to an [`ACCESSTIMEOUT`] structure that receives the
/// information. Set the `size` member of this structure and the `ui_param` parameter to
/// `std::mem::size_of::<ACCESSTIMEOUT>()`.
pub const SPI_GETACCESSTIMEOUT: UINT = 0x003C;

/// Determines whether audio descriptions are enabled or disabled. The `pv_param` parameter is a
/// pointer to an [`AUDIODESCRIPTION`] structure. Set the `size` member of this structure and the
/// `ui_param` parameter to `std::mem::size_of::<AUDIODESCRIPTION>()`.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_GETAUDIODESCRIPTION: UINT = 0x0074;

/// Determines whether animations are enabled or disabled. The `pv_param` parameter must point to a
/// [`BOOL`] variable that receives [`TRUE`] if animations are enabled, or [`FALSE`] otherwise.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_GETCLIENTAREAANIMATION: UINT = 0x1042;

/// Determines whether overlapped content is enabled or disabled. The `pv_param` parameter must
/// point to a [`BOOL`] variable that receives [`TRUE`] if enabled, or [`FALSE`] otherwise.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_GETDISABLEOVERLAPPEDCONTENT: UINT = 0x1040;

/// Retrieves information about the "FilterKeys" accessibility feature. The `pv_param` parameter
/// must point to a [`FILTERKEYS`] structure that receives the information. Set the `size` member
/// of this structure and the `ui_param` parameter to `std::mem::size_of::<FILTERKEYS>()`.
pub const SPI_GETFILTERKEYS: UINT = 0x0032;

/// Retrieves the height, in pixels, of the top and bottom edges of the focus rectangle drawn with
/// [`DrawFocusRect`]. The `pv_param` parameter must point to a [`UINT`] value.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_GETFOCUSBORDERHEIGHT: UINT = 0x2010;

/// Retrieves the width, in pixels, of the left and right edges of the focus rectangle drawn with
/// [`DrawFocusRect`]. The `pv_param` parameter must point to a [`UINT`].
///
/// Windows 2000: This parameter is not supported.
pub const SPI_GETFOCUSBORDERWIDTH: UINT = 0x200E;

/// Retrieves information about the "HighContrast" accessibility feature. The `pv_param` parameter
/// must point to a [`HIGHCONTRAST`] structure that receives the information. Set the `size` member
/// of this structure and the `ui_param` parameter to `std::mem::size_of::<HIGHCONTRAST>()`.
pub const SPI_GETHIGHCONTRAST: UINT = 0x0042;

/// Retrieves a value that determines whether Windows 8 is displaying apps using the default
/// scaling plateau for the hardware or going to the next higher plateau. This value is based on
/// the current "Make everything on your screen bigger" setting, found in the Ease of Access
/// section of PC settings: 1 is on, 0 is off.
///
/// Note: You should not use this value. It might be altered or unavailable in subsequent versions
/// of Windows. Instead, use the [`GetScaleFactorForDevice`] function to retrieve the preferred
/// scaling factor. Desktop applications should use desktop logical DPI rather than scale factor.
/// Desktop logical DPI can be retrieved through the [`GetDeviceCaps`] function.
pub const SPI_GETLOGICALDPIOVERRIDE: UINT = 0x009E;

/// Retrieves the time that notification pop-ups should be displayed, in seconds. The `pv_param`
/// parameter must point to a [`ULONG`] that receives the message duration.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_GETMESSAGEDURATION: UINT = 0x2016;

/// Retrieves the state of the Mouse ClickLock feature. The `pv_param` parameter must point to a
/// [`BOOL`] variable that receives [`TRUE`] if enabled, or [`FALSE`] otherwise.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_GETMOUSECLICKLOCK: UINT = 0x101E;

/// Retrieves the time delay before the primary mouse button is locked. The `pv_param` parameter
/// must point to [`DWORD`] that receives the time delay, in milliseconds. This is only enabled if
/// [`SPI_SETMOUSECLICKLOCK`] is set to [`TRUE`].
///
/// Windows 2000: This parameter is not supported.
pub const SPI_GETMOUSECLICKLOCKTIME: UINT = 0x2008;

/// Retrieves information about the "MouseKeys" accessibility feature. The `pv_param` parameter
/// must point to a [`MOUSEKEYS`] structure that receives the information. Set the `size` member of
/// this structure and the `ui_param` parameter to `std::mem::size_of::<MOUSEKEYS>()`.
pub const SPI_GETMOUSEKEYS: UINT = 0x0036;

/// Retrieves the state of the Mouse Sonar feature. The `pv_param` parameter must point to a
/// [`BOOL`] variable that receives [`TRUE`] if enabled or [`FALSE`] otherwise.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_GETMOUSESONAR: UINT = 0x101C;

/// Retrieves the state of the Mouse Vanish feature. The `pv_param` parameter must point to a
/// [`BOOL`] variable that receives [`TRUE`] if enabled or [`FALSE`] otherwise.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_GETMOUSEVANISH: UINT = 0x1020;

/// Determines whether a screen reviewer utility is running. A screen reviewer utility directs
/// textual information to an output device, such as a speech synthesizer or Braille display. When
/// this flag is set, an application should provide textual information in situations where it
/// would otherwise present the information graphically.
///
/// Note: Narrator, the screen reader that is included with Windows, does not set the
/// [`SPI_SETSCREENREADER`] or [`SPI_GETSCREENREADER`] flags.
pub const SPI_GETSCREENREADER: UINT = 0x0046;

/// This parameter is not supported.
///
/// Windows Server 2003 and Windows XP/2000: The user should control this setting through the
/// Control Panel.
pub const SPI_GETSERIALKEYS: UINT = 0x003E;

/// Determines whether the Show Sounds accessibility flag is on or off. If it is on, the user
/// requires an application to present information visually in situations where it would otherwise
/// present the information only in audible form. The `pv_param` parameter must point to a [`BOOL`]
/// variable that receives [`TRUE`] if the feature is on, or [`FALSE`] if it is off.
///
/// Using this value is equivalent to calling [`GetSystemMetrics`] with [`SM_SHOWSOUNDS`]. That is
/// the recommended call.
pub const SPI_GETSHOWSOUNDS: UINT = 0x0038;

/// Retrieves information about the "SoundSentry" accessibility feature. The `pv_param` parameter
/// must point to a [`SOUNDSENTRY`] structure that receives the information. Set the `size` member
/// of this structure and the `ui_param` parameter to `std::mem::size_of::<SOUNDSENTRY>()`.
pub const SPI_GETSOUNDSENTRY: UINT = 0x0040;

/// Retrieves information about the "StickyKeys" accessibility feature. The `pv_param` parameter
/// must point to a [`STICKYKEYS`] structure that receives the information. Set the `size` member
/// of this structure and the `ui_param` parameter to `std::mem::size_of::<STICKYKEYS>()`.
pub const SPI_GETSTICKYKEYS: UINT = 0x003A;

/// Retrieves information about the "ToggleKeys" accessibility feature. The `pv_param` parameter
/// must point to a [`TOGGLEKEYS`] structure that receives the information. Set the `size` member
/// of this structure and the `ui_param` parameter to `std::mem::size_of::<TOGGLEKEYS>()`.
pub const SPI_GETTOGGLEKEYS: UINT = 0x0034;

/// Sets the time-out period associated with the accessibility features. The `pv_param` parameter
/// must point to an [`ACCESSTIMEOUT`] structure that contains the new parameters. Set the `size`
/// member of this structure and the `ui_param` parameter to
/// `std::mem::size_of::<ACCESSTIMEOUT>()`.
pub const SPI_SETACCESSTIMEOUT: UINT = 0x003D;

/// Turns the audio descriptions feature on or off. The `pv_param` parameter is a pointer to an
/// [`AUDIODESCRIPTION`] structure.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_SETAUDIODESCRIPTION: UINT = 0x0075;

/// Turns client area animations on or off. The `pv_param` parameter is a [`BOOL`] variable. Set
/// `pv_param` to [`TRUE`] to enable animations and other transient effects in the client area, or
/// [`FALSE`] to disable them.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_SETCLIENTAREAANIMATION: UINT = 0x1043;

/// Turns overlapped content (such as background images and watermarks) on or off. The `pv_param`
/// parameter is a [`BOOL`] variable. Set `pv_param` to [`TRUE`] to disable overlapped content, or
/// [`FALSE`] to enable overlapped content.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_SETDISABLEOVERLAPPEDCONTENT: UINT = 0x1041;

/// Sets the parameters of the "FilterKeys" accessibility feature. The `pv_param` parameter must
/// point to a [`FILTERKEYS`] structure that contains the new parameters. Set the `size` member of
/// this structure and the `ui_param` parameter to `std::mem::size_of::<FILTERKEYS>()`.
pub const SPI_SETFILTERKEYS: UINT = 0x0033;

/// Sets the height of the top and bottom edges of the focus rectangle drawn with [`DrawFocusRect`]
/// to the value of the `pv_param` parameter.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_SETFOCUSBORDERHEIGHT: UINT = 0x2011;

/// Sets the height of the left and right edges of the focus rectangle drawn with [`DrawFocusRect`]
/// to the value of the `pv_param` parameter.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_SETFOCUSBORDERWIDTH: UINT = 0x200F;

/// Sets the parameters of the "HighContrast" accessibility feature. The `pv_param` parameter must
/// point to a [`HIGHCONTRAST`] structure that contains the new parameters. Set the `size` member
/// of this structure and the `ui_param` parameter to `std::mem::size_of::<HIGHCONTRAST>()`.
pub const SPI_SETHIGHCONTRAST: UINT = 0x0043;

/// Do not use.
pub const SPI_SETLOGICALDPIOVERRIDE: UINT = 0x009F;

/// Sets the time that notification pop-ups should be displayed, in seconds. The `pv_param`
/// parameter specifies the message duration.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_SETMESSAGEDURATION: UINT = 0x2017;

/// Turns the Mouse ClickLock accessibility feature on or off. This feature temporarily locks down
/// the primary mouse button when that button is clicked and held down for the time specified by
/// [`SPI_SETMOUSECLICKLOCKTIME`]. The `pv_param` parameter specifies [`TRUE`] for on, or [`FALSE`]
/// for off. The default is off.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_SETMOUSECLICKLOCK: UINT = 0x101F;

/// Adjusts the time delay before the primary mouse button is locked. The `ui_param` parameter
/// should be set to 0. The `pv_param` parameter points to a [`DWORD`] that specifies the time
/// delay in milliseconds. For example, specify 1000 for a 1 second delay. The default is 1200.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_SETMOUSECLICKLOCKTIME: UINT = 0x2009;

/// Sets the parameters of the "MouseKeys" accessibility feature. The `pv_param` parameter must
/// point to a [`MOUSEKEYS`] structure that contains the new parameters. Set the `size` member of
/// this structure and the `ui_param` parameter to `std::mem::size_of::<MOUSEKEYS>()`.
pub const SPI_SETMOUSEKEYS: UINT = 0x0037;

/// Turns the Sonar accessibility feature on or off. This feature briefly shows several concentric
/// circles around the mouse pointer when the user presses and releases the CTRL key. The
/// `pv_param` parameter specifies [`TRUE`] for on and [`FALSE`] for off. The default is off.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_SETMOUSESONAR: UINT = 0x101D;

/// Turns the Vanish feature on or off. This feature hides the mouse pointer when the user types;
/// the pointer reappears when the user moves the mouse. The `pv_param` parameter specifies
/// [`TRUE`] for on and [`FALSE`] for off. The default is off.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_SETMOUSEVANISH: UINT = 0x1021;

/// Determines whether a screen review utility is running. The `ui_param` parameter specifies
/// [`TRUE`] for on, or [`FALSE`] for off.
///
/// Note: Narrator, the screen reader that is included with Windows, does not set the
/// [`SPI_SETSCREENREADER`] or [`SPI_GETSCREENREADER`] flags.
pub const SPI_SETSCREENREADER: UINT = 0x0047;

/// This parameter is not supported.
///
/// Windows Server 2003 and Windows XP/2000: The user should control this setting through the
/// Control Panel.
pub const SPI_SETSERIALKEYS: UINT = 0x003F;

/// Turns the ShowSounds accessibility feature on or off. The `ui_param` parameter specifies
/// [`TRUE`] for on, or [`FALSE`] for off.
pub const SPI_SETSHOWSOUNDS: UINT = 0x0039;

/// Sets the parameters of the "SoundSentry" accessibility feature. The `pv_param` parameter must
/// point to a [`SOUNDSENTRY`] structure that contains the new parameters. Set the `size` member of
/// this structure and the `ui_param` parameter to `std::mem::size_of::<SOUNDSENTRY>()`.
pub const SPI_SETSOUNDSENTRY: UINT = 0x0041;

/// Sets the parameters of the "StickyKeys" accessibility feature. The `pv_param` parameter must
/// point to a [`STICKYKEYS`] structure that contains the new parameters. Set the `size` member of
/// this structure and the `ui_param` parameter to `std::mem::size_of::<STICKYKEYS>()`.
pub const SPI_SETSTICKYKEYS: UINT = 0x003B;

/// Sets the parameters of the "ToggleKeys" accessibility feature. The `pv_param` parameter must
/// point to a [`TOGGLEKEYS`] structure that contains the new parameters. Set the `size` member of
/// this structure and the `ui_param` parameter to `std::mem::size_of::<TOGGLEKEYS>()`.
pub const SPI_SETTOGGLEKEYS: UINT = 0x0035;

/// Determines whether "ClearType" is enabled. The `pv_param` parameter must point to a [`BOOL`]
/// variable that receives [`TRUE`] if "ClearType" is enabled, or [`FALSE`] otherwise.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_GETCLEARTYPE: UINT = 0x1048;

/// Retrieves the full path of the bitmap file for the desktop wallpaper. The `pv_param` parameter
/// must point to a buffer to receive the null-terminated path string. Set the `ui_param` parameter
/// to the size, in characters, of the `pv_param` buffer. The returned string will not exceed
/// [`MAX_PATH`] characters. If there is no desktop wallpaper, the returned string is empty.
pub const SPI_GETDESKWALLPAPER: UINT = 0x0073;

/// Determines whether the drop shadow effect is enabled. The `pv_param` parameter must point to a
/// [`BOOL`] variable that returns [`TRUE`] if enabled or [`FALSE`] if disabled.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_GETDROPSHADOW: UINT = 0x1024;

/// Determines whether native User menus have flat menu appearance. The `pv_param` parameter must
/// point to a [`BOOL`] variable that returns [`TRUE`] if the flat menu appearance is set, or
/// [`FALSE`] otherwise.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_GETFLATMENU: UINT = 0x1022;

/// Determines whether the font smoothing feature is enabled. This feature uses font antialiasing
/// to make font curves appear smoother by painting pixels at different gray levels.
///
/// The `pv_param` parameter must point to a [`BOOL`] variable that receives [`TRUE`] if the
/// feature is enabled, or [`FALSE`] if it is not.
pub const SPI_GETFONTSMOOTHING: UINT = 0x004A;

/// Retrieves a contrast value that is used in "ClearType" smoothing. The `pv_param` parameter must
/// point to a [`UINT`] that receives the information. Valid contrast values are from 1000 to 2200.
/// The default value is 1400.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_GETFONTSMOOTHINGCONTRAST: UINT = 0x200C;

/// Retrieves the font smoothing orientation. The `pv_param` parameter must point to a [`UINT`]
/// that receives the information. The possible values are [`FE_FONTSMOOTHINGORIENTATIONBGR`]
/// (blue-green-red) and [`FE_FONTSMOOTHINGORIENTATIONRGB`] (red-green-blue).
///
/// Windows XP/2000: This parameter is not supported until Windows XP with SP2.
pub const SPI_GETFONTSMOOTHINGORIENTATION: UINT = 0x2012;

/// Retrieves the type of font smoothing. The `pv_param` parameter must point to a [`UINT`] that
/// receives the information. The possible values are [`FE_FONTSMOOTHINGSTANDARD`] and
/// [`FE_FONTSMOOTHINGCLEARTYPE`].
///
/// Windows 2000: This parameter is not supported.
pub const SPI_GETFONTSMOOTHINGTYPE: UINT = 0x200A;

/// Retrieves the size of the work area on the primary display monitor. The work area is the
/// portion of the screen not obscured by the system taskbar or by application desktop toolbars.
/// The `pv_param` parameter must point to a [`RECT`] structure that receives the coordinates of
/// the work area, expressed in physical pixel size. Any DPI virtualization mode of the caller has
/// no effect on this output.
///
/// To get the work area of a monitor other than the primary display monitor, call the
/// [`GetMonitorInfo`] function.
pub const SPI_GETWORKAREA: UINT = 0x0030;

/// Turns "ClearType" on or off. The `pv_param` parameter is a [`BOOL`] variable. Set `pv_param` to
/// [`TRUE`] to enable "ClearType", or [`FALSE`] to disable it.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_SETCLEARTYPE: UINT = 0x1049;

/// Reloads the system cursors. Set the `ui_param` parameter to zero and the `pv_param` parameter
/// to [`null_mut`].
pub const SPI_SETCURSORS: UINT = 0x0057;

/// Sets the current desktop pattern by causing Windows to read the "Pattern=" setting from the
/// "WIN.INI" file.
pub const SPI_SETDESKPATTERN: UINT = 0x0015;

/// Note: When the [`SPI_SETDESKWALLPAPER`] flag is used, [`SystemParametersInfo`] returns [`TRUE`]
/// unless there is an error (like when the specified file doesn't exist).
pub const SPI_SETDESKWALLPAPER: UINT = 0x0014;

/// Enables or disables the drop shadow effect. Set `pv_param` to [`TRUE`] to enable the drop
/// shadow effect or [`FALSE`] to disable it. You must also have [`CS_DROPSHADOW`] in the window
/// class style.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_SETDROPSHADOW: UINT = 0x1025;

/// Enables or disables flat menu appearance for native User menus. Set `pv_param` to [`TRUE`] to
/// enable flat menu appearance or [`FALSE`] to disable it.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_SETFLATMENU: UINT = 0x1023;

/// Enables or disables the font smoothing feature, which uses font antialiasing to make font
/// curves appear smoother by painting pixels at different gray levels.
///
/// To enable the feature, set the `ui_param` parameter to [`TRUE`]. To disable the feature, set
/// `ui_param` to [`FALSE`].
pub const SPI_SETFONTSMOOTHING: UINT = 0x004B;

/// Sets the contrast value used in "ClearType" smoothing. The `pv_param` parameter is the contrast
/// value. Valid contrast values are from 1000 to 2200. The default value is 1400.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_SETFONTSMOOTHINGCONTRAST: UINT = 0x200D;

/// Sets the font smoothing orientation. The `pv_param` parameter is either
/// [`FE_FONTSMOOTHINGORIENTATIONBGR`] (blue-green-red) or [`FE_FONTSMOOTHINGORIENTATIONRGB`]
/// (red-green-blue).
///
/// Windows XP/2000: This parameter is not supported until Windows XP with SP2.
pub const SPI_SETFONTSMOOTHINGORIENTATION: UINT = 0x2013;

/// Sets the font smoothing type. The `pv_param` parameter is either [`FE_FONTSMOOTHINGSTANDARD`],
/// if standard anti-aliasing is used, or [`FE_FONTSMOOTHINGCLEARTYPE`], if "ClearType" is used.
/// The default is [`FE_FONTSMOOTHINGSTANDARD`].
///
/// Windows 2000: This parameter is not supported.
pub const SPI_SETFONTSMOOTHINGTYPE: UINT = 0x200B;

/// Sets the size of the work area. The work area is the portion of the screen not obscured by the
/// system taskbar or by application desktop toolbars. The `pv_param` parameter is a pointer to a
/// [`RECT`] structure that specifies the new work area rectangle, expressed in virtual screen
/// coordinates. In a system with multiple display monitors, the function sets the work area of the
/// monitor that contains the specified rectangle.
pub const SPI_SETWORKAREA: UINT = 0x002F;

/// Retrieves the metrics associated with icons. The `pv_param` parameter must point to an
/// [`ICONMETRICS`] structure that receives the information. Set the `size` member of this
/// structure and the `ui_param` parameter to `std::mem::size_of::<ICONMETRICS>()`.
pub const SPI_GETICONMETRICS: UINT = 0x002D;

/// Retrieves the logical font information for the current icon-title font. The `ui_param`
/// parameter specifies the size of a [`LOGFONT`] structure, and the `pv_param` parameter must
/// point to the [`LOGFONT`] structure to fill in.
pub const SPI_GETICONTITLELOGFONT: UINT = 0x001F;

/// Determines whether icon-title wrapping is enabled. The `pv_param` parameter must point to a
/// [`BOOL`] variable that receives [`TRUE`] if enabled, or [`FALSE`] otherwise.
pub const SPI_GETICONTITLEWRAP: UINT = 0x0019;

/// Sets or retrieves the width, in pixels, of an icon cell. The system uses this rectangle to
/// arrange icons in large icon view.
///
/// To retrieve this value, `pv_param` must point to an integer that receives the current value.
pub const SPI_ICONHORIZONTALSPACING: UINT = 0x000D;

/// Sets or retrieves the height, in pixels, of an icon cell.
///
/// To retrieve this value, `pv_param` must point to an integer that receives the current value.
pub const SPI_ICONVERTICALSPACING: UINT = 0x0018;

/// Sets the metrics associated with icons. The `pv_param` parameter must point to an
/// [`ICONMETRICS`] structure that contains the new parameters. Set the `size` member of this
/// structure and the `ui_param` parameter to `std::mem::size_of::<ICONMETRICS>()`.
pub const SPI_SETICONMETRICS: UINT = 0x002E;

/// Reloads the system icons. Set the `ui_param` parameter to zero and the `pv_param` parameter to
/// [`null_mut`].
pub const SPI_SETICONS: UINT = 0x0058;

/// Sets the font that is used for icon titles. The `ui_param` parameter specifies the size of a
/// [`LOGFONT`] structure, and the `pv_param` parameter must point to a [`LOGFONT`] structure.
pub const SPI_SETICONTITLELOGFONT: UINT = 0x0022;

/// Turns icon-title wrapping on or off. The `ui_param` parameter specifies [`TRUE`] for on, or
/// [`FALSE`] for off.
pub const SPI_SETICONTITLEWRAP: UINT = 0x001A;

/// Determines whether the warning beeper is on.
///
/// The `pv_param` parameter must point to a [`BOOL`] variable that receives [`TRUE`] if the beeper
/// is on, or [`FALSE`] if it is off.
pub const SPI_GETBEEP: UINT = 0x0001;

/// Retrieves a [`BOOL`] indicating whether an application can reset the screensaver's timer by
/// calling the [`SendInput`] function to simulate keyboard or mouse input. The `pv_param`
/// parameter must point to a [`BOOL`] variable that receives [`TRUE`] if the simulated input will
/// be blocked, or [`FALSE`] otherwise.
pub const SPI_GETBLOCKSENDINPUTRESETS: UINT = 0x1026;

/// Retrieves the current contact visualization setting. The `pv_param` parameter must point to a
/// [`ULONG`] variable that receives the setting.
pub const SPI_GETCONTACTVISUALIZATION: UINT = 0x2018;

/// Retrieves the input locale identifier for the system default input language. The `pv_param`
/// parameter must point to an HKL variable that receives this value.
pub const SPI_GETDEFAULTINPUTLANG: UINT = 0x0059;

/// Retrieves the current gesture visualization setting. The `pv_param` parameter must point to a
/// [`ULONG`] variable that receives the setting.
pub const SPI_GETGESTUREVISUALIZATION: UINT = 0x201A;

/// Determines whether menu access keys are always underlined. The `pv_param` parameter must point
/// to a [`BOOL`] variable that receives [`TRUE`] if menu access keys are always underlined, and
/// [`FALSE`] if they are underlined only when the menu is activated by the keyboard.
pub const SPI_GETKEYBOARDCUES: UINT = 0x100A;

/// Retrieves the keyboard repeat-delay setting, which is a value in the range from 0
/// (approximately 250 ms delay) through 3 (approximately 1 second delay). The actual delay
/// associated with each value may vary depending on the hardware. The `pv_param` parameter must
/// point to an integer variable that receives the setting.
pub const SPI_GETKEYBOARDDELAY: UINT = 0x0016;

/// Determines whether the user relies on the keyboard instead of the mouse, and wants applications
/// to display keyboard interfaces that would otherwise be hidden. The `pv_param` parameter must
/// point to a [`BOOL`] variable that receives [`TRUE`] if the user relies on the keyboard; or
/// [`FALSE`] otherwise.
pub const SPI_GETKEYBOARDPREF: UINT = 0x0044;

/// Retrieves the keyboard repeat-speed setting, which is a value in the range from 0
/// (approximately 2.5 repetitions per second) through 31 (approximately 30 repetitions per
/// second). The actual repeat rates are hardware-dependent and may vary from a linear scale by as
/// much as 20%. The `pv_param` parameter must point to a [`DWORD`] variable that receives the
/// setting.
pub const SPI_GETKEYBOARDSPEED: UINT = 0x000A;

/// Retrieves the two mouse threshold values and the mouse acceleration. The `pv_param` parameter
/// must point to an array of three integers that receives these values.
pub const SPI_GETMOUSE: UINT = 0x0003;

/// Retrieves the height, in pixels, of the rectangle within which the mouse pointer has to stay
/// for [`TrackMouseEvent`] to generate a [`WM_MOUSEHOVER`] message. The `pv_param` parameter must
/// point to a [`UINT`] variable that receives the height.
pub const SPI_GETMOUSEHOVERHEIGHT: UINT = 0x0064;

/// Retrieves the time, in milliseconds, that the mouse pointer has to stay in the hover rectangle
/// for [`TrackMouseEvent`] to generate a [`WM_MOUSEHOVER`] message. The `pv_param` parameter must
/// point to a [`UINT`] variable that receives the time.
pub const SPI_GETMOUSEHOVERTIME: UINT = 0x0066;

/// Retrieves the width, in pixels, of the rectangle within which the mouse pointer has to stay for
/// [`TrackMouseEvent`] to generate a [`WM_MOUSEHOVER`] message. The `pv_param` parameter must
/// point to a [`UINT`] variable that receives the width.
pub const SPI_GETMOUSEHOVERWIDTH: UINT = 0x0062;

/// Retrieves the current mouse speed. The mouse speed determines how far the pointer will move
/// based on the distance the mouse moves. The `pv_param` parameter must point to an integer that
/// receives a value which ranges between 1 (slowest) and 20 (fastest). A value of 10 is the
/// default. The value can be set by an end-user using the mouse control panel application or by an
/// application using [`SPI_SETMOUSESPEED`].
pub const SPI_GETMOUSESPEED: UINT = 0x0070;

/// Determines whether the Mouse Trails feature is enabled. This feature improves the visibility of
/// mouse cursor movements by briefly showing a trail of cursors and quickly erasing them.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_GETMOUSETRAILS: UINT = 0x005E;

/// Retrieves the routing setting for mouse wheel input. The routing setting determines whether
/// mouse wheel input is sent to the app with focus (foreground) or the app under the mouse cursor.
///
/// Starting with Windows 10: If the value is 2 ([`MOUSEWHEEL_ROUTING_MOUSE_POS`]), mouse wheel
/// input is delivered to the app under the mouse pointer. This is the new default, and
/// [`MOUSEWHEEL_ROUTING_HYBRID`] is no longer available in Settings.
pub const SPI_GETMOUSEWHEELROUTING: UINT = 0x201C;

/// Retrieves the current pen gesture visualization setting. The `pv_param` parameter must point to
/// a [`ULONG`] variable that receives the setting.
pub const SPI_GETPENVISUALIZATION: UINT = 0x201E;

/// Determines whether the snap-to-default-button feature is enabled. If enabled, the mouse cursor
/// automatically moves to the default button, such as OK or Apply, of a dialog box. The `pv_param`
/// parameter must point to a [`BOOL`] variable that receives [`TRUE`] if the feature is on, or
/// [`FALSE`] if it is off.
pub const SPI_GETSNAPTODEFBUTTON: UINT = 0x005F;

/// Starting with Windows 8: Determines whether the system language bar is enabled or disabled. The
/// `pv_param` parameter must point to a [`BOOL`] variable that receives [`TRUE`] if the language
/// bar is enabled, or [`FALSE`] otherwise.
pub const SPI_GETSYSTEMLANGUAGEBAR: UINT = 0x1050;

/// Starting with Windows 8: Determines whether the active input settings have Local (per-thread,
/// [`TRUE`]) or Global (session, [`FALSE`]) scope. The `pv_param` parameter must point to a
/// [`BOOL`] variable.
pub const SPI_GETTHREADLOCALINPUTSETTINGS: UINT = 0x104E;

/// Starting with Windows 11, version 24H2: Retrieves details about the Precision Touchpad,
/// including user settings and system information related to the touchpad.
///
/// The value of the `version_number` field in the [`TOUCHPAD_PARAMETERS`] structure must be set to
/// the appropriate value for the version of the structure being used.
pub const SPI_GETTOUCHPADPARAMETERS: UINT = 0x00AE;

/// Retrieves the number of characters to scroll when the horizontal mouse wheel is moved. The
/// `pv_param` parameter must point to a [`UINT`] variable that receives the number of lines. The
/// default value is 3.
pub const SPI_GETWHEELSCROLLCHARS: UINT = 0x006C;

/// Retrieves the number of lines to scroll when the vertical mouse wheel is moved. The `pv_param`
/// parameter must point to a [`UINT`] variable that receives the number of lines. The default
/// value is 3.
pub const SPI_GETWHEELSCROLLLINES: UINT = 0x0068;

/// Turns the warning beeper on or off. The `ui_param` parameter specifies [`TRUE`] for on, or
/// [`FALSE`] for off.
pub const SPI_SETBEEP: UINT = 0x0002;

/// Determines whether an application can reset the screensaver's timer by calling the
/// [`SendInput`] function to simulate keyboard or mouse input. The `ui_param` parameter specifies
/// [`TRUE`] if the screensaver will not be deactivated by simulated input, or [`FALSE`] if the
/// screensaver will be deactivated by simulated input.
pub const SPI_SETBLOCKSENDINPUTRESETS: UINT = 0x1027;

/// Sets the current contact visualization setting. The `pv_param` parameter must point to a
/// [`ULONG`] variable that identifies the setting.
///
/// Note: If contact visualizations are disabled, gesture visualizations cannot be enabled.
pub const SPI_SETCONTACTVISUALIZATION: UINT = 0x2019;

/// Sets the default input language for the system shell and applications. The specified language
/// must be displayable using the current system character set. The `pv_param` parameter must point
/// to an HKL variable that contains the input locale identifier for the default language.
pub const SPI_SETDEFAULTINPUTLANG: UINT = 0x005A;

/// Sets the double-click time for the mouse to the value of the `ui_param` parameter. If the
/// `ui_param` value is greater than 5000 milliseconds, the system sets the double-click time to
/// 5000 milliseconds.
///
/// The double-click time is the maximum number of milliseconds that can occur between the first
/// and second clicks of a double-click. You can also call the [`SetDoubleClickTime`] function to
/// set the double-click time. To get the current double-click time, call the
/// [`GetDoubleClickTime`] function.
pub const SPI_SETDOUBLECLICKTIME: UINT = 0x0020;

/// Sets the height of the double-click rectangle to the value of the `ui_param` parameter.
///
/// To retrieve the height of the double-click rectangle, call [`GetSystemMetrics`] with the
/// [`SM_CYDOUBLECLK`] flag.
pub const SPI_SETDOUBLECLKHEIGHT: UINT = 0x001E;

/// Sets the width of the double-click rectangle to the value of the `ui_param` parameter.
///
/// To retrieve the width of the double-click rectangle, call [`GetSystemMetrics`] with the
/// [`SM_CXDOUBLECLK`] flag.
pub const SPI_SETDOUBLECLKWIDTH: UINT = 0x001D;

/// Sets the current gesture visualization setting. The `pv_param` parameter must point to a
/// [`ULONG`] variable that identifies the setting.
///
/// Note: If contact visualizations are disabled, gesture visualizations cannot be enabled.
pub const SPI_SETGESTUREVISUALIZATION: UINT = 0x201B;

/// Sets the underlining of menu access key letters. The `pv_param` parameter is a [`BOOL`]
/// variable. Set `pv_param` to [`TRUE`] to always underline menu access keys, or [`FALSE`] to
/// underline menu access keys only when the menu is activated from the keyboard.
pub const SPI_SETKEYBOARDCUES: UINT = 0x100B;

/// Sets the keyboard repeat-delay setting. The `ui_param` parameter must specify 0, 1, 2, or 3,
/// where zero sets the shortest delay approximately 250 ms) and 3 sets the longest delay
/// (approximately 1 second). The actual delay associated with each value may vary depending on the
/// hardware.
pub const SPI_SETKEYBOARDDELAY: UINT = 0x0017;

/// Sets the keyboard preference. The `ui_param` parameter specifies [`TRUE`] if the user relies on
/// the keyboard instead of the mouse, and wants applications to display keyboard interfaces that
/// would otherwise be hidden; `ui_param` is [`FALSE`] otherwise.
pub const SPI_SETKEYBOARDPREF: UINT = 0x0045;

/// Sets the keyboard repeat-speed setting. The `ui_param` parameter must specify a value in the
/// range from 0 (approximately 2.5 repetitions per second) through 31 (approximately 30
/// repetitions per second). The actual repeat rates are hardware-dependent and may vary from a
/// linear scale by as much as 20%. If `ui_param` is greater than 31, the parameter is set to 31.
pub const SPI_SETKEYBOARDSPEED: UINT = 0x000B;

/// Sets the hot key set for switching between input languages. The `ui_param` and `pv_param`
/// parameters are not used. The value sets the shortcut keys in the keyboard property sheets by
/// reading the registry again. The registry must be set before this flag is used. the path in the
/// registry is "HKEY_CURRENT_USER\Keyboard Layout\Toggle". Valid values are "1" = ALT+SHIFT,
/// "2" = CTRL+SHIFT, and "3" = none.
pub const SPI_SETLANGTOGGLE: UINT = 0x005B;

/// Sets the two mouse threshold values and the mouse acceleration. The `pv_param` parameter must
/// point to an array of three integers that specifies these values.
pub const SPI_SETMOUSE: UINT = 0x0004;

/// Swaps or restores the meaning of the left and right mouse buttons. The `ui_param` parameter
/// specifies [`TRUE`] to swap the meanings of the buttons, or [`FALSE`] to restore their original
/// meanings.
///
/// To retrieve the current setting, call [`GetSystemMetrics`] with the [`SM_SWAPBUTTON`] flag.
pub const SPI_SETMOUSEBUTTONSWAP: UINT = 0x0021;

/// Sets the height, in pixels, of the rectangle within which the mouse pointer has to stay for
/// [`TrackMouseEvent`] to generate a [`WM_MOUSEHOVER`] message. Set the `ui_param` parameter to
/// the new height.
pub const SPI_SETMOUSEHOVERHEIGHT: UINT = 0x0065;

/// Sets the time, in milliseconds, that the mouse pointer has to stay in the hover rectangle for
/// [`TrackMouseEvent`] to generate a [`WM_MOUSEHOVER`] message. This is used only if you pass
/// [`HOVER_DEFAULT`] in the `hover_time` parameter in the call to [`TrackMouseEvent`]. Set the
/// `ui_param` parameter to the new time.
///
/// Windows Server 2003 and Windows XP: The operating system does not enforce the use of
/// [`USER_TIMER_MAXIMUM`] and [`USER_TIMER_MINIMUM`] until Windows Server 2003 with SP1 and
/// Windows XP with SP2.
pub const SPI_SETMOUSEHOVERTIME: UINT = 0x0067;

/// Sets the width, in pixels, of the rectangle within which the mouse pointer has to stay for
/// [`TrackMouseEvent`] to generate a [`WM_MOUSEHOVER`] message. Set the `ui_param` parameter to
/// the new width.
pub const SPI_SETMOUSEHOVERWIDTH: UINT = 0x0063;

/// Sets the current mouse speed. The `pv_param` parameter is an integer between 1 (slowest) and 20
/// (fastest). A value of 10 is the default. This value is typically set using the mouse control
/// panel application.
pub const SPI_SETMOUSESPEED: UINT = 0x0071;

/// Enables or disables the Mouse Trails feature, which improves the visibility of mouse cursor
/// movements by briefly showing a trail of cursors and quickly erasing them.
///
/// Windows 2000: This parameter is not supported.
pub const SPI_SETMOUSETRAILS: UINT = 0x005D;

/// Sets the routing setting for mouse wheel input. The routing setting determines whether mouse
/// wheel input is sent to the app with focus (foreground) or the app under the mouse cursor.
///
/// Starting with Windows 10: If the value is 2 ([`MOUSEWHEEL_ROUTING_MOUSE_POS`]), mouse wheel
/// input is delivered to the app under the mouse pointer. This is the new default, and
/// [`MOUSEWHEEL_ROUTING_HYBRID`] is no longer available in Settings.
pub const SPI_SETMOUSEWHEELROUTING: UINT = 0x201D;

/// Sets the current pen gesture visualization setting. The `pv_param` parameter must point to a
/// [`ULONG`] variable that identifies the setting.
pub const SPI_SETPENVISUALIZATION: UINT = 0x201F;

/// Enables or disables the snap-to-default-button feature. If enabled, the mouse cursor
/// automatically moves to the default button, such as OK or Apply, of a dialog box. Set the
/// `ui_param` parameter to [`TRUE`] to enable the feature, or [`FALSE`] to disable it.
/// Applications should use the ShowWindow function when displaying a dialog box so the dialog
/// manager can position the mouse cursor.
pub const SPI_SETSNAPTODEFBUTTON: UINT = 0x0060;

/// Starting with Windows 8: Turns the legacy language bar feature on or off. The `pv_param`
/// parameter is a pointer to a [`BOOL`] variable. Set `pv_param` to [`TRUE`] to enable the legacy
/// language bar, or [`FALSE`] to disable it. The flag is supported on Windows 8 where the legacy
/// language bar is replaced by Input Switcher and therefore turned off by default. Turning the
/// legacy language bar on is provided for compatibility reasons and has no effect on the Input
/// Switcher.
pub const SPI_SETSYSTEMLANGUAGEBAR: UINT = 0x1051;

/// Starting with Windows 8: Determines whether the active input settings have Local (per-thread,
/// [`TRUE`]) or Global (session, [`FALSE`]) scope. The `pv_param` parameter must be a [`BOOL`]
/// variable, casted by [`PVOID`].
pub const SPI_SETTHREADLOCALINPUTSETTINGS: UINT = 0x104F;

/// Starting with Windows 11, version 24H2: Sets details about the Precision Touchpad, including
/// user settings and system information related to the touchpad.
///
/// The value of the `version_number` field in the [`TOUCHPAD_PARAMETERS`] structure must be set to
/// the appropriate value for the version of the structure being used.
pub const SPI_SETTOUCHPADPARAMETERS: UINT = 0x00AF;

/// Sets the number of characters to scroll when the horizontal mouse wheel is moved. The number of
/// characters is set from the `ui_param` parameter.
pub const SPI_SETWHEELSCROLLCHARS: UINT = 0x006D;

/// Sets the number of lines to scroll when the vertical mouse wheel is moved. The number of lines
/// is set from the `ui_param` parameter.
///
/// The number of lines is the suggested number of lines to scroll when the mouse wheel is rolled
/// without using modifier keys. If the number is 0, then no scrolling should occur. If the number
/// of lines to scroll is greater than the number of lines viewable, and in particular if it is
/// [`WHEEL_PAGESCROLL`] (#defined as [`UINT_MAX`]), the scroll operation should be interpreted as
/// clicking once in the page down or page up regions of the scroll bar.
pub const SPI_SETWHEELSCROLLLINES: UINT = 0x0069;

/// Determines whether pop-up menus are left-aligned or right-aligned, relative to the
/// corresponding menu-bar item. The `pv_param` parameter must point to a [`BOOL`] variable that
/// receives [`TRUE`] if right-aligned, or [`FALSE`] otherwise.
pub const SPI_GETMENUDROPALIGNMENT: UINT = 0x001B;

/// Determines whether menu fade animation is enabled. The `pv_param` parameter must point to a
/// [`BOOL`] variable that receives [`TRUE`] when fade animation is enabled and [`FALSE`] when it
/// is disabled. If fade animation is disabled, menus use slide animation. This flag is ignored
/// unless menu animation is enabled, which you can do using the [`SPI_SETMENUANIMATION`] flag.
pub const SPI_GETMENUFADE: UINT = 0x1012;

/// Retrieves the time, in milliseconds, that the system waits before displaying a shortcut menu
/// when the mouse cursor is over a submenu item. The `pv_param` parameter must point to a
/// [`DWORD`] variable that receives the time of the delay.
pub const SPI_GETMENUSHOWDELAY: UINT = 0x006A;

/// Sets the alignment value of pop-up menus. The `ui_param` parameter specifies [`TRUE`] for right
/// alignment, or [`FALSE`] for left alignment.
pub const SPI_SETMENUDROPALIGNMENT: UINT = 0x001C;

/// Enables or disables menu fade animation. Set `pv_param` to [`TRUE`] to enable the menu fade
/// effect or [`FALSE`] to disable it. If fade animation is disabled, menus use slide animation.
/// The menu fade effect is possible only if the system has a color depth of more than 256 colors.
/// This flag is ignored unless [`SPI_MENUANIMATION`] is also set.
pub const SPI_SETMENUFADE: UINT = 0x1013;

/// Sets `ui_param` to the time, in milliseconds, that the system waits before displaying a
/// shortcut menu when the mouse cursor is over a submenu item.
pub const SPI_SETMENUSHOWDELAY: UINT = 0x006B;

/// This parameter is not supported.
///
/// Windows Server 2003 and Windows XP/2000: Determines whether the low-power phase of screen
/// saving is enabled. The `pv_param` parameter must point to a [`BOOL`] variable that receives
/// [`TRUE`] if enabled, or [`FALSE`] if disabled. This flag is supported for 32-bit applications
/// only.
pub const SPI_GETLOWPOWERACTIVE: UINT = 0x0053;

/// This parameter is not supported.
///
/// Windows Server 2003 and Windows XP/2000: Retrieves the time-out value for the low-power phase
/// of screen saving. The `pv_param` parameter must point to an integer variable that receives the
/// value. This flag is supported for 32-bit applications only.
pub const SPI_GETLOWPOWERTIMEOUT: UINT = 0x004F;

/// This parameter is not supported. When the power-off phase of screen saving is enabled, the
/// [`GUID_VIDEO_POWERDOWN_TIMEOUT`] power setting is greater than zero.
///
/// Windows Server 2003 and Windows XP/2000: Determines whether the power-off phase of screen
/// saving is enabled. The `pv_param` parameter must point to a [`BOOL`] variable that receives
/// [`TRUE`] if enabled, or [`FALSE`] if disabled. This flag is supported for 32-bit applications
/// only.
pub const SPI_GETPOWEROFFACTIVE: UINT = 0x0054;

/// This parameter is not supported. Instead, check the [`GUID_VIDEO_POWERDOWN_TIMEOUT`] power
/// setting.
///
/// Windows Server 2003 and Windows XP/2000: Retrieves the time-out value for the power-off phase
/// of screen saving. The `pv_param` parameter must point to an integer variable that receives the
/// value. This flag is supported for 32-bit applications only.
pub const SPI_GETPOWEROFFTIMEOUT: UINT = 0x0050;

/// This parameter is not supported.
///
/// Windows Server 2003 and Windows XP/2000: Activates or deactivates the low-power phase of screen
/// saving. Set `ui_param` to 1 to activate, or zero to deactivate. The `pv_param` parameter must
/// be [`null_mut`]. This flag is supported for 32-bit applications only.
pub const SPI_SETLOWPOWERACTIVE: UINT = 0x0055;

/// This parameter is not supported.
///
/// Windows Server 2003 and Windows XP/2000: Sets the time-out value, in seconds, for the low-power
/// phase of screen saving. The `ui_param` parameter specifies the new value. The `pv_param`
/// parameter must be [`null_mut`]. This flag is supported for 32-bit applications only.
pub const SPI_SETLOWPOWERTIMEOUT: UINT = 0x0051;

/// This parameter is not supported. Instead, set the [`GUID_VIDEO_POWERDOWN_TIMEOUT`] power
/// setting.
///
/// Windows Server 2003 and Windows XP/2000: Activates or deactivates the power-off phase of screen
/// saving. Set `ui_param` to 1 to activate, or zero to deactivate. The `pv_param` parameter must
/// be [`null_mut`]. This flag is supported for 32-bit applications only.
pub const SPI_SETPOWEROFFACTIVE: UINT = 0x0056;

/// This parameter is not supported. Instead, set the [`GUID_VIDEO_POWERDOWN_TIMEOUT`] power
/// setting to a time-out value.
///
/// Windows Server 2003 and Windows XP/2000: Sets the time-out value, in seconds, for the
/// power-off phase of screen saving. The `ui_param` parameter specifies the new value. The
/// `pv_param` parameter must be [`null_mut`]. This flag is supported for 32-bit applications only.
pub const SPI_SETPOWEROFFTIMEOUT: UINT = 0x0052;

/// Determines whether screen saving is enabled. The `pv_param` parameter must point to a [`BOOL`]
/// variable that receives [`TRUE`] if screen saving is enabled, or [`FALSE`] otherwise.
///
/// Windows 7, Windows Server 2008 R2 and Windows 2000: The function returns [`TRUE`] even when
/// screen saving is not enabled.
pub const SPI_GETSCREENSAVEACTIVE: UINT = 0x0010;

/// Determines whether a screen saver is currently running on the window station of the calling
/// process. The `pv_param` parameter must point to a [`BOOL`] variable that receives [`TRUE`] if a
/// screen saver is currently running, or [`FALSE`] otherwise. Note that only the interactive
/// window station, WinSta0, can have a screen saver running.
pub const SPI_GETSCREENSAVERRUNNING: UINT = 0x0072;

/// Determines whether the screen saver requires a password to display the Windows desktop. The
/// `pv_param` parameter must point to a [`BOOL`] variable that receives [`TRUE`] if the screen
/// saver requires a password, or [`FALSE`] otherwise. The `ui_param` parameter is ignored.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_GETSCREENSAVESECURE: UINT = 0x0076;

/// Retrieves the screen saver time-out value, in seconds. The `pv_param` parameter must point to
/// an integer variable that receives the value.
pub const SPI_GETSCREENSAVETIMEOUT: UINT = 0x000E;

/// Sets the state of the screen saver. The `ui_param` parameter specifies [`TRUE`] to activate
/// screen saving, or [`FALSE`] to deactivate it.
///
/// If the machine has entered power saving mode or system lock state, an
/// [`ERROR_OPERATION_IN_PROGRESS`] exception occurs.
pub const SPI_SETSCREENSAVEACTIVE: UINT = 0x0011;

/// Sets whether the screen saver requires the user to enter a password to display the Windows
/// desktop. The `ui_param` parameter is a [`BOOL`] variable. The `pv_param` parameter is ignored.
/// Set `ui_param` to [`TRUE`] to require a password, or [`FALSE`] to not require a password.
///
/// Windows Server 2003 and Windows XP/2000: This parameter is not supported.
pub const SPI_SETSCREENSAVESECURE: UINT = 0x0077;

/// Sets the screen saver time-out value to the value of the `ui_param` parameter. This value is
/// the amount of time, in seconds, that the system must be idle before the screen saver activates.
///
/// If the machine has entered power saving mode or system lock state, an
/// [`ERROR_OPERATION_IN_PROGRESS`] exception occurs.
pub const SPI_SETSCREENSAVETIMEOUT: UINT = 0x000F;

/// Retrieves the number of milliseconds that a thread can go without dispatching a message before
/// the system considers it unresponsive. The `pv_param` parameter must point to an integer
/// variable that receives the value.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETHUNGAPPTIMEOUT: UINT = 0x0078;

/// Retrieves the number of milliseconds that the system waits before terminating an application
/// that does not respond to a shutdown request. The `pv_param` parameter must point to an integer
/// variable that receives the value.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETWAITTOKILLTIMEOUT: UINT = 0x007A;

/// Retrieves the number of milliseconds that the service control manager waits before terminating
/// a service that does not respond to a shutdown request. The `pv_param` parameter must point to
/// an integer variable that receives the value.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETWAITTOKILLSERVICETIMEOUT: UINT = 0x007C;

/// Sets the hung application time-out to the value of the `ui_param` parameter. This value is the
/// number of milliseconds that a thread can go without dispatching a message before the system
/// considers it unresponsive.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETHUNGAPPTIMEOUT: UINT = 0x0079;

/// Sets the application shutdown request time-out to the value of the `ui_param` parameter. This
/// value is the number of milliseconds that the system waits before terminating an application
/// that does not respond to a shutdown request.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETWAITTOKILLTIMEOUT: UINT = 0x007B;

/// Sets the service shutdown request time-out to the value of the `ui_param` parameter. This value
/// is the number of milliseconds that the system waits before terminating a service that does not
/// respond to a shutdown request.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETWAITTOKILLSERVICETIMEOUT: UINT = 0x007D;

/// Determines whether the slide-open effect for combo boxes is enabled. The `pv_param` parameter
/// must point to a [`BOOL`] variable that receives [`TRUE`] for enabled, or [`FALSE`] for
/// disabled.
pub const SPI_GETCOMBOBOXANIMATION: UINT = 0x1004;

/// Determines whether the cursor has a shadow around it. The `pv_param` parameter must point to a
/// [`BOOL`] variable that receives [`TRUE`] if the shadow is enabled, [`FALSE`] if it is disabled.
/// This effect appears only if the system has a color depth of more than 256 colors.
pub const SPI_GETCURSORSHADOW: UINT = 0x101A;

/// Determines whether the gradient effect for window title bars is enabled. The `pv_param`
/// parameter must point to a [`BOOL`] variable that receives [`TRUE`] for enabled, or [`FALSE`]
/// for disabled. For more information about the gradient effect, see the [`GetSysColor`] function.
pub const SPI_GETGRADIENTCAPTIONS: UINT = 0x1008;

/// Determines whether hot tracking of user-interface elements, such as menu names on menu bars, is
/// enabled. The `pv_param` parameter must point to a [`BOOL`] variable that receives [`TRUE`] for
/// enabled, or [`FALSE`] for disabled.
///
/// Hot tracking means that when the cursor moves over an item, it is highlighted but not selected.
/// You can query this value to decide whether to use hot tracking in the user interface of your
/// application.
pub const SPI_GETHOTTRACKING: UINT = 0x100E;

/// Determines whether the smooth-scrolling effect for list boxes is enabled. The `pv_param`
/// parameter must point to a [`BOOL`] variable that receives [`TRUE`] for enabled, or [`FALSE`]
/// for disabled.
pub const SPI_GETLISTBOXSMOOTHSCROLLING: UINT = 0x1006;

/// Determines whether the menu animation feature is enabled. This master switch must be on to
/// enable menu animation effects. The `pv_param` parameter must point to a [`BOOL`] variable that
/// receives [`TRUE`] if animation is enabled and [`FALSE`] if it is disabled.
///
/// If animation is enabled, SPI_GETMENUFADE indicates whether menus use fade or slide animation.
pub const SPI_GETMENUANIMATION: UINT = 0x1002;

/// Same as SPI_GETKEYBOARDCUES.
pub const SPI_GETMENUUNDERLINES: UINT = 0x100A;

/// Determines whether the selection fade effect is enabled. The `pv_param` parameter must point to
/// a [`BOOL`] variable that receives [`TRUE`] if enabled or [`FALSE`] if disabled.
///
/// The selection fade effect causes the menu item selected by the user to remain on the screen
/// briefly while fading out after the menu is dismissed.
pub const SPI_GETSELECTIONFADE: UINT = 0x1014;

/// Determines whether ToolTip animation is enabled. The `pv_param` parameter must point to a
/// [`BOOL`] variable that receives [`TRUE`] if enabled or [`FALSE`] if disabled. If ToolTip
/// animation is enabled, [`SPI_GETTOOLTIPFADE`] indicates whether ToolTips use fade or slide
/// animation.
pub const SPI_GETTOOLTIPANIMATION: UINT = 0x1016;

/// If [`SPI_SETTOOLTIPANIMATION`] is enabled, [`SPI_GETTOOLTIPFADE`] indicates whether ToolTip
/// animation uses a fade effect or a slide effect. The `pv_param` parameter must point to a
/// [`BOOL`] variable that receives [`TRUE`] for fade animation or [`FALSE`] for slide animation.
/// For more information on slide and fade effects, see [`AnimateWindow`].
pub const SPI_GETTOOLTIPFADE: UINT = 0x1018;

/// Determines whether UI effects are enabled or disabled. The `pv_param` parameter must point to a
/// [`BOOL`] variable that receives [`TRUE`] if all UI effects are enabled, or [`FALSE`] if they
/// are disabled.
pub const SPI_GETUIEFFECTS: UINT = 0x103E;

/// Enables or disables the slide-open effect for combo boxes. Set the `pv_param` parameter to
/// [`TRUE`] to enable the gradient effect, or [`FALSE`] to disable it.
pub const SPI_SETCOMBOBOXANIMATION: UINT = 0x1005;

/// Enables or disables a shadow around the cursor. The `pv_param` parameter is a [`BOOL`]
/// variable. Set `pv_param` to [`TRUE`] to enable the shadow or [`FALSE`] to disable the shadow.
/// This effect appears only if the system has a color depth of more than 256 colors.
pub const SPI_SETCURSORSHADOW: UINT = 0x101B;

/// Enables or disables the gradient effect for window title bars. Set the `pv_param` parameter to
/// [`TRUE`] to enable it, or [`FALSE`] to disable it. The gradient effect is possible only if the
/// system has a color depth of more than 256 colors. For more information about the gradient
/// effect, see the [`GetSysColor`] function.
pub const SPI_SETGRADIENTCAPTIONS: UINT = 0x1009;

/// Enables or disables hot tracking of user-interface elements such as menu names on menu bars.
/// Set the `pv_param` parameter to [`TRUE`] to enable it, or [`FALSE`] to disable it.
///
/// Hot-tracking means that when the cursor moves over an item, it is highlighted but not selected.
pub const SPI_SETHOTTRACKING: UINT = 0x100F;

/// Enables or disables the smooth-scrolling effect for list boxes. Set the `pv_param` parameter to
/// [`TRUE`] to enable the smooth-scrolling effect, or [`FALSE`] to disable it.
pub const SPI_SETLISTBOXSMOOTHSCROLLING: UINT = 0x1007;

/// Enables or disables menu animation. This master switch must be on for any menu animation to
/// occur. The `pv_param` parameter is a [`BOOL`] variable; set `pv_param` to [`TRUE`] to enable
/// animation and [`FALSE`] to disable animation.
///
/// If animation is enabled, [`SPI_GETMENUFADE`] indicates whether menus use fade or slide
/// animation.
pub const SPI_SETMENUANIMATION: UINT = 0x1003;

/// Same as [`SPI_SETKEYBOARDCUES`].
pub const SPI_SETMENUUNDERLINES: UINT = 0x100B;

/// Set `pv_param` to [`TRUE`] to enable the selection fade effect or [`FALSE`] to disable it.
///
/// The selection fade effect causes the menu item selected by the user to remain on the screen
/// briefly while fading out after the menu is dismissed. The selection fade effect is possible
/// only if the system has a color depth of more than 256 colors.
pub const SPI_SETSELECTIONFADE: UINT = 0x1015;

/// Set `pv_param` to [`TRUE`] to enable ToolTip animation or [`FALSE`] to disable it. If enabled,
/// you can use [`SPI_SETTOOLTIPFADE`] to specify fade or slide animation.
pub const SPI_SETTOOLTIPANIMATION: UINT = 0x1017;

/// If the [`SPI_SETTOOLTIPANIMATION`] flag is enabled, use [`SPI_SETTOOLTIPFADE`] to indicate
/// whether ToolTip animation uses a fade effect or a slide effect. Set `pv_param` to [`TRUE`] for
/// fade animation or [`FALSE`] for slide animation. The tooltip fade effect is possible only if
/// the system has a color depth of more than 256 colors. For more information on the slide and
/// fade effects, see the [`AnimateWindow`] function.
pub const SPI_SETTOOLTIPFADE: UINT = 0x1019;

/// Enables or disables UI effects. Set the `pv_param` parameter to [`TRUE`] to enable all UI
/// effects or [`FALSE`] to disable all UI effects.
pub const SPI_SETUIEFFECTS: UINT = 0x103F;

/// Determines whether active window tracking (activating the window the mouse is on) is on or off.
/// The `pv_param` parameter must point to a [`BOOL`] variable that receives [`TRUE`] for on, or
/// [`FALSE`] for off.
pub const SPI_GETACTIVEWINDOWTRACKING: UINT = 0x1000;

/// Determines whether windows activated through active window tracking will be brought to the top.
/// The `pv_param` parameter must point to a [`BOOL`] variable that receives [`TRUE`] for on, or
/// [`FALSE`] for off.
pub const SPI_GETACTIVEWNDTRKZORDER: UINT = 0x100C;

/// Retrieves the active window tracking delay, in milliseconds. The `pv_param` parameter must
/// point to a [`DWORD`] variable that receives the time.
pub const SPI_GETACTIVEWNDTRKTIMEOUT: UINT = 0x2002;

/// Retrieves the animation effects associated with user actions. The `pv_param` parameter must
/// point to an [`ANIMATIONINFO `] structure that receives the information. Set the `size` member
/// of this structure and the `ui_param` parameter to `std::mem::size_of::<ANIMATIONINFO>()`.
pub const SPI_GETANIMATION: UINT = 0x0048;

/// Retrieves the border multiplier factor that determines the width of a window's sizing border.
/// The `pv_param` parameter must point to an integer variable that receives this value.
pub const SPI_GETBORDER: UINT = 0x0005;

/// Retrieves the caret width in edit controls, in pixels. The `pv_param` parameter must point to a
/// [`DWORD`] variable that receives this value.
pub const SPI_GETCARETWIDTH: UINT = 0x2006;

/// Determines whether a window is docked when it is moved to the top, left, or right edges of a
/// monitor or monitor array. The `pv_param` parameter must point to a [`BOOL`] variable that
/// receives [`TRUE`] if enabled, or [`FALSE`] otherwise.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETDOCKMOVING: UINT = 0x0090;

/// Determines whether a maximized window is restored when its caption bar is dragged. The
/// `pv_param` parameter must point to a [`BOOL`] variable that receives [`TRUE`] if enabled, or
/// [`FALSE`] otherwise.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETDRAGFROMMAXIMIZE: UINT = 0x008C;

/// Determines whether dragging of full windows is enabled. The `pv_param` parameter must point to
/// a [`BOOL`] variable that receives [`TRUE`] if enabled, or [`FALSE`] otherwise.
pub const SPI_GETDRAGFULLWINDOWS: UINT = 0x0026;

/// Retrieves the number of times [`SetForegroundWindow`] will flash the taskbar button when
/// rejecting a foreground switch request. The `pv_param` parameter must point to a [`DWORD`]
/// variable that receives the value.
pub const SPI_GETFOREGROUNDFLASHCOUNT: UINT = 0x2004;

/// Retrieves the amount of time following user input, in milliseconds, during which the system
/// will not allow applications to force themselves into the foreground. The `pv_param` parameter
/// must point to a [`DWORD`] variable that receives the time.
pub const SPI_GETFOREGROUNDLOCKTIMEOUT: UINT = 0x2000;

/// Retrieves the metrics associated with minimized windows. The `pv_param` parameter must point to
/// a [`MINIMIZEDMETRICS`] structure that receives the information. Set the `size` member of this
/// structure and the `ui_param` parameter to `std::mem::size_of::<MINIMIZEDMETRICS>()`.
pub const SPI_GETMINIMIZEDMETRICS: UINT = 0x002B;

/// Retrieves the threshold in pixels where docking behavior is triggered by using a mouse to drag
/// a window to the edge of a monitor or monitor array. The default threshold is 1. The `pv_param`
/// parameter must point to a [`DWORD`] variable that receives the value.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETMOUSEDOCKTHRESHOLD: UINT = 0x007E;

/// Retrieves the threshold in pixels where undocking behavior is triggered by using a mouse to
/// drag a window from the edge of a monitor or a monitor array toward the center. The default
/// threshold is 20.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETMOUSEDRAGOUTTHRESHOLD: UINT = 0x0084;

/// Retrieves the threshold in pixels from the top of a monitor or a monitor array where a
/// vertically maximized window is restored when dragged with the mouse. The default threshold is
/// 50.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETMOUSESIDEMOVETHRESHOLD: UINT = 0x0088;

/// Retrieves the metrics associated with the nonclient area of nonminimized windows. The
/// `pv_param` parameter must point to a [`NONCLIENTMETRICS`] structure that receives the
/// information. Set the `size` member of this structure and the `ui_param` parameter to
/// `std::mem::size_of::<NONCLIENTMETRICS>()`.
///
/// Windows Server 2003 and Windows XP/2000: See Remarks for [`NONCLIENTMETRICS`].
pub const SPI_GETNONCLIENTMETRICS: UINT = 0x0029;

/// Retrieves the threshold in pixels where docking behavior is triggered by using a pen to drag a
/// window to the edge of a monitor or monitor array. The default is 30.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETPENDOCKTHRESHOLD: UINT = 0x0080;

/// Retrieves the threshold in pixels where undocking behavior is triggered by using a pen to drag
/// a window from the edge of a monitor or monitor array toward its center. The default threshold
/// is 30.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETPENDRAGOUTTHRESHOLD: UINT = 0x0086;

/// Retrieves the threshold in pixels from the top of a monitor or monitor array where a vertically
/// maximized window is restored when dragged with the mouse. The default threshold is 50.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETPENSIDEMOVETHRESHOLD: UINT = 0x008A;

/// Determines whether the IME status window is visible (on a per-user basis). The `pv_param`
/// parameter must point to a [`BOOL`] variable that receives [`TRUE`] if the status window is
/// visible, or [`FALSE`] if it is not.
pub const SPI_GETSHOWIMEUI: UINT = 0x006E;

/// Determines whether a window is vertically maximized when it is sized to the top or bottom of a
/// monitor or monitor array. The `pv_param` parameter must point to a [`BOOL`] variable that
/// receives [`TRUE`] if enabled, or [`FALSE`] otherwise.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETSNAPSIZING: UINT = 0x008E;

/// Determines whether window arrangement is enabled. The `pv_param` parameter must point to a
/// [`BOOL`] variable that receives [`TRUE`] if enabled, or [`FALSE`] otherwise.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_GETWINARRANGING: UINT = 0x0082;

/// Sets active window tracking (activating the window the mouse is on) either on or off. Set
/// `pv_param` to [`TRUE`] for on or [`FALSE`] for off.
pub const SPI_SETACTIVEWINDOWTRACKING: UINT = 0x1001;

/// Determines whether or not windows activated through active window tracking should be brought to
/// the top. Set `pv_param` to [`TRUE`] for on or [`FALSE`] for off.
pub const SPI_SETACTIVEWNDTRKZORDER: UINT = 0x100D;

/// Sets the active window tracking delay. Set `pv_param` to the number of milliseconds to delay
/// before activating the window under the mouse pointer.
pub const SPI_SETACTIVEWNDTRKTIMEOUT: UINT = 0x2003;

/// Sets the animation effects associated with user actions. The `pv_param` parameter must point to
/// an [`ANIMATIONINFO `] structure that contains the new parameters. Set the `size` member of this
/// structure and the `ui_param` parameter to `std::mem::size_of::<ANIMATIONINFO>()`.
pub const SPI_SETANIMATION: UINT = 0x0049;

/// Sets the border multiplier factor that determines the width of a window's sizing border. The
/// `ui_param` parameter specifies the new value.
pub const SPI_SETBORDER: UINT = 0x0006;

/// Sets the caret width in edit controls. Set `pv_param` to the desired width, in pixels. The
/// default and minimum value is 1.
pub const SPI_SETCARETWIDTH: UINT = 0x2007;

/// Sets whether a window is docked when it is moved to the top, left, or right docking targets on
/// a monitor or monitor array. Set `pv_param` to [`TRUE`] for on or [`FALSE`] for off.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETDOCKMOVING: UINT = 0x0091;

/// Sets whether a maximized window is restored when its caption bar is dragged. Set `pv_param` to
/// [`TRUE`] for on or [`FALSE`] for off.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETDRAGFROMMAXIMIZE: UINT = 0x008D;

/// Sets dragging of full windows either on or off. The `ui_param` parameter specifies [`TRUE`] for
/// on, or [`FALSE`] for off.
pub const SPI_SETDRAGFULLWINDOWS: UINT = 0x0025;

/// Sets the height, in pixels, of the rectangle used to detect the start of a drag operation. Set
/// `ui_param` to the new value. To retrieve the drag height, call [`GetSystemMetrics`] with the
/// [`SM_CYDRAG`] flag.
pub const SPI_SETDRAGHEIGHT: UINT = 0x004D;

/// Sets the width, in pixels, of the rectangle used to detect the start of a drag operation. Set
/// `ui_param` to the new value. To retrieve the drag width, call [`GetSystemMetrics`] with the
/// [`SM_CXDRAG`] flag.
pub const SPI_SETDRAGWIDTH: UINT = 0x004C;

/// Sets the number of times SetForegroundWindow will flash the taskbar button when rejecting a
/// foreground switch request. Set `pv_param` to the number of times to flash.
pub const SPI_SETFOREGROUNDFLASHCOUNT: UINT = 0x2005;

/// Sets the amount of time following user input, in milliseconds, during which the system does not
/// allow applications to force themselves into the foreground. Set `pv_param` to the new time-out
/// value.
///
/// The calling thread must be able to change the foreground window, otherwise the call fails.
pub const SPI_SETFOREGROUNDLOCKTIMEOUT: UINT = 0x2001;

/// Sets the metrics associated with minimized windows. The `pv_param` parameter must point to a
/// [`MINIMIZEDMETRICS`] structure that contains the new parameters. Set the `size` member of this
/// structure and the `ui_param` parameter to `std::mem::size_of::<MINIMIZEDMETRICS>()`.
pub const SPI_SETMINIMIZEDMETRICS: UINT = 0x002C;

/// Sets the threshold in pixels where docking behavior is triggered by using a mouse to drag a
/// window to the edge of a monitor or monitor array. The default threshold is 1. The `pv_param`
/// parameter must point to a [`DWORD`] variable that contains the new value.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETMOUSEDOCKTHRESHOLD: UINT = 0x007F;

/// Sets the threshold in pixels where undocking behavior is triggered by using a mouse to drag a
/// window from the edge of a monitor or monitor array to its center. The default threshold is 20.
/// The `pv_param` parameter must point to a [`DWORD`] variable that contains the new value.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETMOUSEDRAGOUTTHRESHOLD: UINT = 0x0085;

/// Sets the threshold in pixels from the top of the monitor where a vertically maximized window is
/// restored when dragged with the mouse. The default threshold is 50. The `pv_param` parameter
/// must point to a [`DWORD`] variable that contains the new value.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETMOUSESIDEMOVETHRESHOLD: UINT = 0x0089;

/// Sets the metrics associated with the nonclient area of nonminimized windows. The `pv_param`
/// parameter must point to a [`NONCLIENTMETRICS`] structure that contains the new parameters. Set
/// the `size` member of this structure and the `ui_param` parameter to
/// `std::mem::size_of::<NONCLIENTMETRICS>()`. Also, the `height` member of the [`LOGFONT`]
/// structure must be a negative value.
pub const SPI_SETNONCLIENTMETRICS: UINT = 0x002A;

/// Sets the threshold in pixels where docking behavior is triggered by using a pen to drag a
/// window to the edge of a monitor or monitor array. The default threshold is 30. The `pv_param`
/// parameter must point to a [`DWORD`] variable that contains the new value.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETPENDOCKTHRESHOLD: UINT = 0x0081;

/// Sets the threshold in pixels where undocking behavior is triggered by using a pen to drag a
/// window from the edge of a monitor or monitor array to its center. The default threshold is 30.
/// The `pv_param` parameter must point to a [`DWORD`] variable that contains the new value.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETPENDRAGOUTTHRESHOLD: UINT = 0x0087;

/// Sets the threshold in pixels from the top of the monitor where a vertically maximized window is
/// restored when dragged with a pen. The default threshold is 50. The `pv_param` parameter must
/// point to a [`DWORD`] variable that contains the new value.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETPENSIDEMOVETHRESHOLD: UINT = 0x008B;

/// Sets whether the IME status window is visible or not on a per-user basis. The `ui_param`
/// parameter specifies [`TRUE`] for on or [`FALSE`] for off.
pub const SPI_SETSHOWIMEUI: UINT = 0x006F;

/// Sets whether a window is vertically maximized when it is sized to the top or bottom of the
/// monitor. Set `pv_param` to [`TRUE`] for on or [`FALSE`] for off.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETSNAPSIZING: UINT = 0x008F;

/// Sets whether window arrangement is enabled. Set `pv_param` to [`TRUE`] for on or [`FALSE`] for
/// off.
///
/// Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP/2000: This parameter is
/// not supported.
pub const SPI_SETWINARRANGING: UINT = 0x0083;
