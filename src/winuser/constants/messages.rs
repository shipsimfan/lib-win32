#![allow(missing_docs)]

use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    BROADCAST_QUERY_DENY, CREATESTRUCT, CreateWindow, CreateWindowEx, DefWindowProc,
    GET_RAWINPUT_CODE_WPARAM, GIDC_ARRIVAL, GIDC_REMOVAL, GetMessage, GetRawInputData,
    GetRawInputDeviceInfo, HANDLE, HRAWINPUT, MAKEPOINTS, MINMAXINFO, POINTS, PeekMessage,
    PostMessage, PostQuitMessage, RAWINPUT, RECT, RIDEV_DEVNOTIFY, RIM_INPUT, RIM_INPUTSINK,
    RegisterRawInputDevices, SIZE_MAXHIDE, SIZE_MAXIMIZED, SIZE_MAXSHOW, SIZE_MINIMIZED,
    SIZE_RESTORED, SetWindowPos, TRUE, USER_DEFAULT_SCREEN_DPI, WINDOWPOS,
    dbt::{
        DBT_CONFIGCHANGECANCELED, DBT_CONFIGCHANGED, DBT_CUSTOMEVENT, DBT_DEVICEARRIVAL,
        DBT_DEVICEQUERYREMOVE, DBT_DEVICEQUERYREMOVEFAILED, DBT_DEVICEREMOVECOMPLETE,
        DBT_DEVICEREMOVEPENDING, DBT_DEVICETYPESPECIFIC, DBT_DEVNODES_CHANGED,
        DBT_QUERYCHANGECONFIG, DBT_USERDEFINED,
    },
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Performs no operation. An application sends the [`WM_NULL`] message if it wants to post a
/// message that the recipient window will ignore.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `w_param` - This parameter is not used.
///  * `l_param` - This parameter is not used.
///
/// # Return Value
/// An application returns zero if it processes this message.
///
/// # Remarks
/// For example, if an application has installed a [`WH_GETMESSAGE`] hook and wants to prevent a
/// message from being processed, the [`GetMsgProc`] callback function can change the message
/// number to [`WM_NULL`] so the recipient will ignore it.
///
/// As another example, an application can check if a window is responding to messages by sending
/// the [`WM_NULL`] message with the [`SendMessageTimeout`] function.
pub const WM_NULL: UINT = 0x0000;

/// Sent when an application requests that a window be created by calling the [`CreateWindowEx`] or
/// [`CreateWindow`] function. (The message is sent before the function returns.) The window
/// procedure of the new window receives this message after the window is created, but before the
/// window becomes visible.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `w_param` - This parameter is not used.
///  * `l_param` - A pointer to a [`CREATESTRUCT`] structure that contains information about the
///                window being created.
///
/// # Return Value
/// If an application processes this message, it should return zero to continue creation of the
/// window. If the application returns –1, the window is destroyed and the [`CreateWindowEx`] or
/// [`CreateWindow`] function returns a [`null_mut`] handle.
pub const WM_CREATE: UINT = 0x0001;

/// Sent when a window is being destroyed. It is sent to the window procedure of the window being
/// destroyed after the window is removed from the screen.
///
/// This message is sent first to the window being destroyed and then to the child windows (if any)
/// as they are destroyed. During the processing of the message, it can be assumed that all child
/// windows still exist.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `w_param` - This parameter is not used.
///  * `l_param` - This parameter is not used.
///
/// # Return Value
/// If an application processes this message, it should return zero.
///
/// # Remarks
/// If the window being destroyed is part of the clipboard viewer chain (set by calling the
/// [`SetClipboardViewer`] function), the window must remove itself from the chain by processing
/// the [`ChangeClipboardChain`] function before returning from the [`WM_DESTROY`] message.
pub const WM_DESTROY: UINT = 0x0002;

/// Sent after a window has been moved.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `w_param` - This parameter is not used.
///  * `l_param` - The x and y coordinates of the upper-left corner of the client area of the
///                window. The low-order word contains the x-coordinate while the high-order word
///                contains the y coordinate.
///
/// # Return Value
/// If an application processes this message, it should return zero.
///
/// # Remarks
/// The parameters are given in screen coordinates for overlapped and pop-up windows and in
/// parent-client coordinates for child windows.
///
/// You can also use the [`MAKEPOINTS`] macro to convert the `l_param` parameter to a [`POINTS`]
/// structure.
///
/// The [`DefWindowProc`] function sends the [`WM_SIZE`] and [`WM_MOVE`] messages when it processes
/// the [`WM_WINDOWPOSCHANGED`] message. The [`WM_SIZE`] and [`WM_MOVE`] messages are not sent if
/// an application handles the [`WM_WINDOWPOSCHANGED`] message without calling [`DefWindowProc`].
pub const WM_MOVE: UINT = 0x0003;

/// Sent to a window after its size has changed.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `w_param` - The type of resizing requested. This parameter can be one of the following
///                values:
///    * [`SIZE_MAXHIDE`] - Message is sent to all pop-up windows when some other window is
///                         maximized.
///    * [`SIZE_MAXIMIZED`] - The window has been maximized.
///    * [`SIZE_MAXSHOW`] - Message is sent to all pop-up windows when some other window has been
///                         restored to its former size.
///    * [`SIZE_MINIMIZED`] - The window has been minimized.
///    * [`SIZE_RESTORED`] - The window has been resized, but neither the [`SIZE_MINIMIZED`] nor
///                          [`SIZE_MAXIMIZED`] value applies.
///  * `l_param` - The low-order word of `l_param` specifies the new width of the client area. The
///                high-order word of `l_param` specifies the new height of the client area.
///
/// # Return Value
/// If an application processes this message, it should return zero.
///
/// # Remarks
/// If the [`SetScrollPos`] or [`MoveWindow`] function is called for a child window as a result of
/// the [`WM_SIZE`] message, the `redraw` or `repaint` parameter should be nonzero to cause the
/// window to be repainted.
///
/// Although the width and height of a window are 32-bit values, the `l_param` parameter contains
/// only the low-order 16 bits of each.
///
/// The [`DefWindowProc`] function sends the [`WM_SIZE`] and [`WM_MOVE`] messages when it processes
/// the [`WM_WINDOWPOSCHANGED`] message. The [`WM_SIZE`] and [`WM_MOVE`] messages are not sent if
/// an application handles the [`WM_WINDOWPOSCHANGED`] message without calling [`DefWindowProc`].
pub const WM_SIZE: UINT = 0x0005;
pub const WM_ACTIVATE: UINT = 0x0006;
pub const WM_SETFOCUS: UINT = 0x0007;
pub const WM_KILLFOCUS: UINT = 0x0008;
pub const WM_ENABLE: UINT = 0x000A;
pub const WM_SETREDRAW: UINT = 0x000B;
pub const WM_SETTEXT: UINT = 0x000C;
pub const WM_GETTEXT: UINT = 0x000D;
pub const WM_GETTEXTLENGTH: UINT = 0x000E;
pub const WM_PAINT: UINT = 0x000F;

/// Sent as a signal that a window or an application should terminate.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `w_param` - This parameter is not used.
///  * `l_param` - This parameter is not used.
///
/// # Return Value
/// If an application processes this message, it should return zero.
pub const WM_CLOSE: UINT = 0x0010;
pub const WM_QUERYENDSESSION: UINT = 0x0011;
pub const WM_QUERYOPEN: UINT = 0x0013;
pub const WM_ENDSESSION: UINT = 0x0016;

/// Indicates a request to terminate an application, and is generated when the application calls
/// the [`PostQuitMessage`] function. This message causes the [`GetMessage`] function to return zero.
///
/// # Parameters
///  * `w_param` - The exit code given in the [`PostQuitMessage`] function.
///  * `l_param` - This parameter is not used.
///
/// # Return Value
/// This message does not have a return value because it causes the message loop to terminate
/// before the message is sent to the application's window procedure.
///
/// # Remarks
/// The [`WM_QUIT`] message is not associated with a window and therefore will never be received
/// through a window's window procedure. It is retrieved only by the [`GetMessage`] or
/// [`PeekMessage`] functions.
///
/// Do not post the [`WM_QUIT`] message using the [`PostMessage`] function; use
/// [`PostQuitMessage`].
pub const WM_QUIT: UINT = 0x0012;
pub const WM_ERASEBKGND: UINT = 0x0014;
pub const WM_SYSCOLORCHANGE: UINT = 0x0015;
pub const WM_SHOWWINDOW: UINT = 0x0018;
pub const WM_WININICHANGE: UINT = 0x001A;
pub const WM_SETTINGCHANGE: UINT = WM_WININICHANGE;
pub const WM_DEVMODECHANGE: UINT = 0x001B;
pub const WM_ACTIVATEAPP: UINT = 0x001C;
pub const WM_FONTCHANGE: UINT = 0x001D;
pub const WM_TIMECHANGE: UINT = 0x001E;
pub const WM_CANCELMODE: UINT = 0x001F;
pub const WM_SETCURSOR: UINT = 0x0020;
pub const WM_MOUSEACTIVATE: UINT = 0x0021;
pub const WM_CHILDACTIVATE: UINT = 0x0022;
pub const WM_QUEUESYNC: UINT = 0x0023;

/// Sent to a window when the size or position of the window is about to change. An application can
/// use this message to override the window's default maximized size and position, or its default
/// minimum or maximum tracking size.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `w_param` - This parameter is not used.
///  * `l_param` - A pointer to a [`MINMAXINFO`] structure that contains the default maximized
///                position and dimensions, and the default minimum and maximum tracking sizes. An
///                application can override the defaults by setting the members of this structure.
///
/// # Return Value
/// If an application processes this message, it should return zero.
///
/// # Remarks
/// The maximum tracking size is the largest window size that can be produced by using the borders
/// to size the window. The minimum tracking size is the smallest window size that can be produced
/// by using the borders to size the window.
pub const WM_GETMINMAXINFO: UINT = 0x0024;
pub const WM_PAINTICON: UINT = 0x0026;
pub const WM_ICONERASEBKGND: UINT = 0x0027;
pub const WM_NEXTDLGCTL: UINT = 0x0028;
pub const WM_SPOOLERSTATUS: UINT = 0x002A;
pub const WM_DRAWITEM: UINT = 0x002B;
pub const WM_MEASUREITEM: UINT = 0x002C;
pub const WM_DELETEITEM: UINT = 0x002D;
pub const WM_VKEYTOITEM: UINT = 0x002E;
pub const WM_CHARTOITEM: UINT = 0x002F;
pub const WM_SETFONT: UINT = 0x0030;
pub const WM_GETFONT: UINT = 0x0031;
pub const WM_SETHOTKEY: UINT = 0x0032;
pub const WM_GETHOTKEY: UINT = 0x0033;
pub const WM_QUERYDRAGICON: UINT = 0x0037;
pub const WM_COMPAREITEM: UINT = 0x0039;
pub const WM_GETOBJECT: UINT = 0x003D;
pub const WM_COMPACTING: UINT = 0x0041;
pub const WM_COMMNOTIFY: UINT = 0x0044; /* no longer suported */
pub const WM_WINDOWPOSCHANGING: UINT = 0x0046;

/// Sent to a window whose size, position, or place in the Z order has changed as a result of a
/// call to the [`SetWindowPos`] function or another window-management function.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `w_param` - This parameter is not used.
///  * `l_param` - A pointer to a [`WINDOWPOS`] structure that contains information about the
///                window's new size and position.
///
/// # Return Value
/// If an application processes this message, it should return zero.
///
/// # Remarks
/// By default, the [`DefWindowProc`] function sends the [`WM_SIZE`] and [`WM_MOVE`] messages to
/// the window. The [`WM_SIZE`] and [`WM_MOVE`] messages are not sent if an application handles the
/// [`WM_WINDOWPOSCHANGED`] message without calling [`DefWindowProc`]. It is more efficient to
/// perform any move or size change processing during the [`WM_WINDOWPOSCHANGED`] message without
/// calling [`DefWindowProc`].
pub const WM_WINDOWPOSCHANGED: UINT = 0x0047;
pub const WM_POWER: UINT = 0x0048;
pub const WM_COPYDATA: UINT = 0x004A;
pub const WM_CANCELJOURNAL: UINT = 0x004B;
pub const WM_NOTIFY: UINT = 0x004E;
pub const WM_INPUTLANGCHANGEREQUEST: UINT = 0x0050;
pub const WM_INPUTLANGCHANGE: UINT = 0x0051;
pub const WM_TCARD: UINT = 0x0052;
pub const WM_HELP: UINT = 0x0053;
pub const WM_USERCHANGED: UINT = 0x0054;
pub const WM_NOTIFYFORMAT: UINT = 0x0055;
pub const WM_CONTEXTMENU: UINT = 0x007B;
pub const WM_STYLECHANGING: UINT = 0x007C;
pub const WM_STYLECHANGED: UINT = 0x007D;

/// The [`WM_DISPLAYCHANGE`] message is sent to all windows when the display resolution has
/// changed.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `w_param` - The new image depth of the display, in bits per pixel.
///  * `l_param` - The low-order word specifies the horizontal resolution of the screen. The
///                high-order word specifies the vertical resolution of the screen
///
/// # Remarks
/// This message is only sent to top-level windows. For all other windows it is posted.
pub const WM_DISPLAYCHANGE: UINT = 0x007E;
pub const WM_GETICON: UINT = 0x007F;
pub const WM_SETICON: UINT = 0x0080;
pub const WM_NCCREATE: UINT = 0x0081;
pub const WM_NCDESTROY: UINT = 0x0082;
pub const WM_NCCALCSIZE: UINT = 0x0083;
pub const WM_NCHITTEST: UINT = 0x0084;
pub const WM_NCPAINT: UINT = 0x0085;
pub const WM_NCACTIVATE: UINT = 0x0086;
pub const WM_GETDLGCODE: UINT = 0x0087;
pub const WM_SYNCPAINT: UINT = 0x0088;
pub const WM_NCMOUSEMOVE: UINT = 0x00A0;
pub const WM_NCLBUTTONDOWN: UINT = 0x00A1;
pub const WM_NCLBUTTONUP: UINT = 0x00A2;
pub const WM_NCLBUTTONDBLCLK: UINT = 0x00A3;
pub const WM_NCRBUTTONDOWN: UINT = 0x00A4;
pub const WM_NCRBUTTONUP: UINT = 0x00A5;
pub const WM_NCRBUTTONDBLCLK: UINT = 0x00A6;
pub const WM_NCMBUTTONDOWN: UINT = 0x00A7;
pub const WM_NCMBUTTONUP: UINT = 0x00A8;
pub const WM_NCMBUTTONDBLCLK: UINT = 0x00A9;
pub const WM_NCXBUTTONDOWN: UINT = 0x00AB;
pub const WM_NCXBUTTONUP: UINT = 0x00AC;
pub const WM_NCXBUTTONDBLCLK: UINT = 0x00AD;

/// Sent to the window that registered to receive raw input.
///
/// Raw input notifications are available only after the application calls
/// [`RegisterRawInputDevices`] with [`RIDEV_DEVNOTIFY`] flag.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `w_param` - This parameter can be one of the following values:
///    * [`GIDC_ARRIVAL`] - A new device has been added to the system. You can call
///                         [`GetRawInputDeviceInfo`] to get more information regarding the device.
///    * [`GIDC_REMOVAL`] - A device has been removed from the system.
///  * `l_param` - The [`HANDLE`] to the raw input device.
///
/// # Return Value
/// If an application processes this message, it should return zero.
pub const WM_INPUT_DEVICE_CHANGE: UINT = 0x00FE;

/// Sent to the window that is getting raw input.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `w_param` - The input code. Use [`GET_RAWINPUT_CODE_WPARAM`] macro to get the value. Can be
///                one of the following values:
///    * [`RIM_INPUT`] - Input occurred while the application was in the foreground. The
///                      application must call [`DefWindowProc`] so the system can perform cleanup.
///    * [`RIM_INPUTSINK`] - Input occurred while the application was not in the foreground.
///  * `l_param` - A [`HRAWINPUT`] handle to the [`RAWINPUT`] structure that contains the raw input
///                from the device. To get the raw data, use this handle in the call to
///                [`GetRawInputData`].
///
/// # Return Value
/// If an application processes this message, it should return zero.
///
/// # Remarks
/// Raw input is available only when the application calls [`RegisterRawInputDevices`] with valid
/// device specifications.
pub const WM_INPUT: UINT = 0x00FF;
pub const WM_KEYFIRST: UINT = 0x0100;
pub const WM_KEYDOWN: UINT = 0x0100;
pub const WM_KEYUP: UINT = 0x0101;
pub const WM_CHAR: UINT = 0x0102;
pub const WM_DEADCHAR: UINT = 0x0103;
pub const WM_SYSKEYDOWN: UINT = 0x0104;
pub const WM_SYSKEYUP: UINT = 0x0105;
pub const WM_SYSCHAR: UINT = 0x0106;
pub const WM_SYSDEADCHAR: UINT = 0x0107;
pub const WM_UNICHAR: UINT = 0x0109;
pub const WM_KEYLAST: UINT = 0x0109;
pub const WM_IME_STARTCOMPOSITION: UINT = 0x010D;
pub const WM_IME_ENDCOMPOSITION: UINT = 0x010E;
pub const WM_IME_COMPOSITION: UINT = 0x010F;
pub const WM_IME_KEYLAST: UINT = 0x010F;
pub const WM_INITDIALOG: UINT = 0x0110;
pub const WM_COMMAND: UINT = 0x0111;
pub const WM_SYSCOMMAND: UINT = 0x0112;
pub const WM_TIMER: UINT = 0x0113;
pub const WM_HSCROLL: UINT = 0x0114;
pub const WM_VSCROLL: UINT = 0x0115;
pub const WM_INITMENU: UINT = 0x0116;
pub const WM_INITMENUPOPUP: UINT = 0x0117;
pub const WM_GESTURE: UINT = 0x0119;
pub const WM_GESTURENOTIFY: UINT = 0x011A;
pub const WM_MENUSELECT: UINT = 0x011F;
pub const WM_MENUCHAR: UINT = 0x0120;
pub const WM_ENTERIDLE: UINT = 0x0121;
pub const WM_MENURBUTTONUP: UINT = 0x0122;
pub const WM_MENUDRAG: UINT = 0x0123;
pub const WM_MENUGETOBJECT: UINT = 0x0124;
pub const WM_UNINITMENUPOPUP: UINT = 0x0125;
pub const WM_MENUCOMMAND: UINT = 0x0126;
pub const WM_CHANGEUISTATE: UINT = 0x0127;
pub const WM_UPDATEUISTATE: UINT = 0x0128;
pub const WM_QUERYUISTATE: UINT = 0x0129;
pub const WM_CTLCOLORMSGBOX: UINT = 0x0132;
pub const WM_CTLCOLOREDIT: UINT = 0x0133;
pub const WM_CTLCOLORLISTBOX: UINT = 0x0134;
pub const WM_CTLCOLORBTN: UINT = 0x0135;
pub const WM_CTLCOLORDLG: UINT = 0x0136;
pub const WM_CTLCOLORSCROLLBAR: UINT = 0x0137;
pub const WM_CTLCOLORSTATIC: UINT = 0x0138;
pub const WM_MOUSEFIRST: UINT = 0x0200;
pub const WM_MOUSEMOVE: UINT = 0x0200;
pub const WM_LBUTTONDOWN: UINT = 0x0201;
pub const WM_LBUTTONUP: UINT = 0x0202;
pub const WM_LBUTTONDBLCLK: UINT = 0x0203;
pub const WM_RBUTTONDOWN: UINT = 0x0204;
pub const WM_RBUTTONUP: UINT = 0x0205;
pub const WM_RBUTTONDBLCLK: UINT = 0x0206;
pub const WM_MBUTTONDOWN: UINT = 0x0207;
pub const WM_MBUTTONUP: UINT = 0x0208;
pub const WM_MBUTTONDBLCLK: UINT = 0x0209;
pub const WM_MOUSEWHEEL: UINT = 0x020A;
pub const WM_XBUTTONDOWN: UINT = 0x020B;
pub const WM_XBUTTONUP: UINT = 0x020C;
pub const WM_XBUTTONDBLCLK: UINT = 0x020D;
pub const WM_MOUSEHWHEEL: UINT = 0x020E;
pub const WM_MOUSELAST: UINT = 0x020E;
pub const WM_PARENTNOTIFY: UINT = 0x0210;
pub const WM_ENTERMENULOOP: UINT = 0x0211;
pub const WM_EXITMENULOOP: UINT = 0x0212;
pub const WM_NEXTMENU: UINT = 0x0213;
pub const WM_SIZING: UINT = 0x0214;
pub const WM_CAPTURECHANGED: UINT = 0x0215;
pub const WM_MOVING: UINT = 0x0216;
pub const WM_POWERBROADCAST: UINT = 0x0218;

/// Notifies an application of a change to the hardware configuration of a device or the computer.
///
/// A window receives this message through its `wnd_proc` function.
///
/// # Parameters
///  * `wnd` - A handle to the window.
///  * `msg` - The [`WM_DEVICECHANGE`] identifier.
///  * `w_param` - The event that has occurred. This parameter can be one of the following values:
///    * [`DBT_DEVNODES_CHANGED`] - A device has been added to or removed from the system.
///    * [`DBT_QUERYCHANGECONFIG`] - Permission is requested to change the current configuration
///                                  (dock or undock).
///    * [`DBT_CONFIGCHANGED`] - The current configuration has changed, due to a dock or undock.
///    * [`DBT_CONFIGCHANGECANCELED`] - A request to change the current configuration (dock or
///                                     undock) has been canceled.
///    * [`DBT_DEVICEARRIVAL`] - A device or piece of media has been inserted and is now available.
///    * [`DBT_DEVICEQUERYREMOVE`] - Permission is requested to remove a device or piece of media.
///                                  Any application can deny this request and cancel the removal.
///    * [`DBT_DEVICEQUERYREMOVEFAILED`] - A request to remove a device or piece of media has been
///                                        canceled.
///    * [`DBT_DEVICEREMOVEPENDING`] - A device or piece of media is about to be removed. Cannot be
///                                    denied.
///    * [`DBT_DEVICEREMOVECOMPLETE`] - A device or piece of media has been removed.
///    * [`DBT_DEVICETYPESPECIFIC`] - A device-specific event has occurred.
///    * [`DBT_CUSTOMEVENT`] - A custom event has occurred.
///    * [`DBT_USERDEFINED`] - The meaning of this message is user-defined.
///  * `l_param` - A pointer to a structure that contains event-specific data. Its format depends
///                on the value of the wParam parameter.
///
/// # Return Value
/// Return [`TRUE`] to grant the request.
///
/// Return [`BROADCAST_QUERY_DENY`] to deny the request.
///
/// # Remarks
/// For devices that offer software-controllable features, such as ejection and locking, the system
/// typically sends a [`DBT_DEVICEREMOVEPENDING`] message to let applications and device drivers
/// end their use of the device gracefully. If the system forcibly removes a device, it may not
/// send a [`DBT_DEVICEQUERYREMOVE`] message before doing so.
pub const WM_DEVICECHANGE: UINT = 0x0219;
pub const WM_MDICREATE: UINT = 0x0220;
pub const WM_MDIDESTROY: UINT = 0x0221;
pub const WM_MDIACTIVATE: UINT = 0x0222;
pub const WM_MDIRESTORE: UINT = 0x0223;
pub const WM_MDINEXT: UINT = 0x0224;
pub const WM_MDIMAXIMIZE: UINT = 0x0225;
pub const WM_MDITILE: UINT = 0x0226;
pub const WM_MDICASCADE: UINT = 0x0227;
pub const WM_MDIICONARRANGE: UINT = 0x0228;
pub const WM_MDIGETACTIVE: UINT = 0x0229;
pub const WM_MDISETMENU: UINT = 0x0230;
pub const WM_ENTERSIZEMOVE: UINT = 0x0231;
pub const WM_EXITSIZEMOVE: UINT = 0x0232;
pub const WM_DROPFILES: UINT = 0x0233;
pub const WM_MDIREFRESHMENU: UINT = 0x0234;
pub const WM_POINTERDEVICECHANGE: UINT = 0x238;
pub const WM_POINTERDEVICEINRANGE: UINT = 0x239;
pub const WM_POINTERDEVICEOUTOFRANGE: UINT = 0x23A;
pub const WM_TOUCH: UINT = 0x0240;
pub const WM_NCPOINTERUPDATE: UINT = 0x0241;
pub const WM_NCPOINTERDOWN: UINT = 0x0242;
pub const WM_NCPOINTERUP: UINT = 0x0243;
pub const WM_POINTERUPDATE: UINT = 0x0245;
pub const WM_POINTERDOWN: UINT = 0x0246;
pub const WM_POINTERUP: UINT = 0x0247;
pub const WM_POINTERENTER: UINT = 0x0249;
pub const WM_POINTERLEAVE: UINT = 0x024A;
pub const WM_POINTERACTIVATE: UINT = 0x024B;
pub const WM_POINTERCAPTURECHANGED: UINT = 0x024C;
pub const WM_TOUCHHITTESTING: UINT = 0x024D;
pub const WM_POINTERWHEEL: UINT = 0x024E;
pub const WM_POINTERHWHEEL: UINT = 0x024F;
pub const WM_POINTERROUTEDTO: UINT = 0x0251;
pub const WM_POINTERROUTEDAWAY: UINT = 0x0252;
pub const WM_POINTERROUTEDRELEASED: UINT = 0x0253;
pub const WM_IME_SETCONTEXT: UINT = 0x0281;
pub const WM_IME_NOTIFY: UINT = 0x0282;
pub const WM_IME_CONTROL: UINT = 0x0283;
pub const WM_IME_COMPOSITIONFULL: UINT = 0x0284;
pub const WM_IME_SELECT: UINT = 0x0285;
pub const WM_IME_CHAR: UINT = 0x0286;
pub const WM_IME_REQUEST: UINT = 0x0288;
pub const WM_IME_KEYDOWN: UINT = 0x0290;
pub const WM_IME_KEYUP: UINT = 0x0291;
pub const WM_MOUSEHOVER: UINT = 0x02A1;
pub const WM_MOUSELEAVE: UINT = 0x02A3;
pub const WM_NCMOUSEHOVER: UINT = 0x02A0;
pub const WM_NCMOUSELEAVE: UINT = 0x02A2;
pub const WM_WTSSESSION_CHANGE: UINT = 0x02B1;
pub const WM_TABLET_FIRST: UINT = 0x02C0;
pub const WM_TABLET_LAST: UINT = 0x02DF;

/// Sent when the effective dots per inch (dpi) for a window has changed. The DPI is the scale
/// factor for a window. There are multiple events that can cause the DPI to change. The following
/// list indicates the possible causes for the change in DPI.
///  - The window is moved to a new monitor that has a different DPI.
///  - The DPI of the monitor hosting the window changes.
///
/// The current DPI for a window always equals the last DPI sent by [`WM_DPICHANGED`]. This is the
/// scale factor that the window should be scaling to for threads that are aware of DPI changes.
///
/// # Parameters
///  * `w_param` - The [`HIWORD`] of the `w_param` contains the Y-axis value of the new dpi of the
///                window. The [`LOWORD`] of the `w_param` contains the X-axis value of the new DPI
///                of the window. For example, 96, 120, 144, or 192. The values of the X-axis and
///                the Y-axis are identical for Windows apps.
///  * `l_param` - A pointer to a [`RECT`] structure that provides a suggested size and position of
///                the current window scaled for the new DPI. The expectation is that apps will
///                reposition and resize windows based on the suggestions provided by `l_param`
///                when handling this message.
///
/// # Return Value
/// If an application processes this message, it should return zero.
///
/// # Remarks
/// This message is only relevant for [`PROCESS_PER_MONITOR_DPI_AWARE`] applications or
/// [`DPI_AWARENESS_PER_MONITOR_AWARE`] threads. It may be received on certain DPI changes if your
/// top-level window or process is running as DPI unaware or system DPI aware, but in those
/// situations it can be safely ignored. For more information about the different types of
/// awareness, see [`PROCESS_DPI_AWARENESS`] and [`DPI_AWARENESS`]. Older versions of Windows
/// required DPI awareness to be tied at the level of an application. Those apps use
/// [`PROCESS_DPI_AWARENESS`]. Currently, DPI awareness is tied to threads and individual windows
/// rather than the entire application. These apps use [`DPI_AWARENESS`].
///
/// You only need to use either the X-axis or the Y-axis value when scaling your application since
/// they are the same.
///
/// In order to handle this message correctly, you will need to resize and reposition your window
/// based on the suggestions provided by `l_param` and using [`SetWindowPos`]. If you do not do
/// this, your window will grow or shrink with respect to everything else on the new monitor. For
/// example, if a user is using multiple monitors and drags your window from a 96 DPI monitor to a
/// 192 DPI monitor, your window will appear to be half as large with respect to other items on the
/// 192 DPI monitor.
///
/// The base value of DPI is defined as [`USER_DEFAULT_SCREEN_DPI`] which is set to 96. To determine
/// the scaling factor for a monitor, take the DPI value and divide by [`USER_DEFAULT_SCREEN_DPI`].
pub const WM_DPICHANGED: UINT = 0x02E0;
pub const WM_DPICHANGED_BEFOREPARENT: UINT = 0x02E2;
pub const WM_DPICHANGED_AFTERPARENT: UINT = 0x02E3;
pub const WM_GETDPISCALEDSIZE: UINT = 0x02E4;
pub const WM_CUT: UINT = 0x0300;
pub const WM_COPY: UINT = 0x0301;
pub const WM_PASTE: UINT = 0x0302;
pub const WM_CLEAR: UINT = 0x0303;
pub const WM_UNDO: UINT = 0x0304;
pub const WM_RENDERFORMAT: UINT = 0x0305;
pub const WM_RENDERALLFORMATS: UINT = 0x0306;
pub const WM_DESTROYCLIPBOARD: UINT = 0x0307;
pub const WM_DRAWCLIPBOARD: UINT = 0x0308;
pub const WM_PAINTCLIPBOARD: UINT = 0x0309;
pub const WM_VSCROLLCLIPBOARD: UINT = 0x030A;
pub const WM_SIZECLIPBOARD: UINT = 0x030B;
pub const WM_ASKCBFORMATNAME: UINT = 0x030C;
pub const WM_CHANGECBCHAIN: UINT = 0x030D;
pub const WM_HSCROLLCLIPBOARD: UINT = 0x030E;
pub const WM_QUERYNEWPALETTE: UINT = 0x030F;
pub const WM_PALETTEISCHANGING: UINT = 0x0310;
pub const WM_PALETTECHANGED: UINT = 0x0311;
pub const WM_HOTKEY: UINT = 0x0312;
pub const WM_PRINT: UINT = 0x0317;
pub const WM_PRINTCLIENT: UINT = 0x0318;
pub const WM_APPCOMMAND: UINT = 0x0319;
pub const WM_THEMECHANGED: UINT = 0x031A;
pub const WM_CLIPBOARDUPDATE: UINT = 0x031D;
pub const WM_DWMCOMPOSITIONCHANGED: UINT = 0x031E;
pub const WM_DWMNCRENDERINGCHANGED: UINT = 0x031F;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: UINT = 0x0320;
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: UINT = 0x0321;
pub const WM_DWMSENDICONICTHUMBNAIL: UINT = 0x0323;
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: UINT = 0x0326;
pub const WM_GETTITLEBARINFOEX: UINT = 0x033F;
pub const WM_HANDHELDFIRST: UINT = 0x0358;
pub const WM_HANDHELDLAST: UINT = 0x035F;
pub const WM_AFXFIRST: UINT = 0x0360;
pub const WM_AFXLAST: UINT = 0x037F;
pub const WM_PENWINFIRST: UINT = 0x0380;
pub const WM_PENWINLAST: UINT = 0x038F;
pub const WM_APP: UINT = 0x8000;
pub const WM_USER: UINT = 0x0400;
pub const WM_TOOLTIPDISMISS: UINT = 0x0345;

pub const DM_GETDEFID: UINT = WM_USER + 0;
pub const DM_SETDEFID: UINT = WM_USER + 1;
pub const DM_REPOSITION: UINT = WM_USER + 2;
