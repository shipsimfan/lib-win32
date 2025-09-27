use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    SetForegroundWindow, WM_ENTERIDLE, WM_HELP, WM_SETFONT, WS_CAPTION, WS_EX_CONTEXTHELP,
    WS_EX_TOPMOST, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_SYSMENU,
};

/// Obsolete. The system automatically applies the three-dimensional look to dialog boxes created
/// by applications.
pub const DS_3DLOOK: DWORD = 0x0004;

/// Indicates that the coordinates of the dialog box are screen coordinates. If this style is not
/// specified, the coordinates are client coordinates.
pub const DS_ABSALIGN: DWORD = 0x01;

/// Centers the dialog box in the working area of the monitor that contains the owner window. If no
/// owner window is specified, the dialog box is centered in the working area of a monitor
/// determined by the system. The working area is the area not obscured by the taskbar or any
/// appbars.
pub const DS_CENTER: DWORD = 0x0800;

/// Centers the dialog box on the mouse cursor.
pub const DS_CENTERMOUSE: DWORD = 0x1000;

/// Includes a question mark in the title bar of the dialog box. When the user clicks the question
/// mark, the cursor changes to a question mark with a pointer. If the user then clicks a control
/// in the dialog box, the control receives a [`WM_HELP`] message. The control should pass the
/// message to the dialog box procedure, which should call the function using the `HELP_WM_HELP`
/// command. The help application displays a pop-up window that typically contains help for the
/// control.
///
/// Note that [`DS_CONTEXTHELP`] is only a placeholder. When the dialog box is created, the system
/// checks for [`DS_CONTEXTHELP`] and, if it is there, adds [`WS_EX_CONTEXTHELP`] to the extended
/// style of the dialog box. [`WS_EX_CONTEXTHELP`] cannot be used with the [`WS_MAXIMIZEBOX`] or
/// [`WS_MINIMIZEBOX`] styles.
pub const DS_CONTEXTHELP: DWORD = 0x2000;

/// Creates a dialog box that works well as a child window of another dialog box, much like a page
/// in a property sheet. This style allows the user to tab among the control windows of a child
/// dialog box, use its accelerator keys, and so on.
pub const DS_CONTROL: DWORD = 0x0400;

/// Causes the dialog box to use the SYSTEM_FIXED_FONT instead of the default SYSTEM_FONT. This is
/// a monospace font compatible with the System font in 16-bit versions of Windows earlier than
/// 3.0.
pub const DS_FIXEDSYS: DWORD = 0x0008;

/// Applies to 16-bit applications only. This style directs edit controls in the dialog box to
/// allocate memory from the application's data segment. Otherwise, edit controls allocate storage
/// from a global memory object.
pub const DS_LOCALEDIT: DWORD = 0x20;

/// Creates a dialog box with a modal dialog-box frame that can be combined with a title bar and
/// window menu by specifying the [`WS_CAPTION`] and [`WS_SYSMENU`] styles.
pub const DS_MODALFRAME: DWORD = 0x80;

/// Creates the dialog box even if errors occur for example, if a child window cannot be created or
/// if the system cannot create a special data segment for an edit control.
pub const DS_NOFAILCREATE: DWORD = 0x0010;

/// Suppresses [`WM_ENTERIDLE`] messages that the system would otherwise send to the owner of the
/// dialog box while the dialog box is displayed.
pub const DS_NOIDLEMSG: DWORD = 0x100;

/// Indicates that the header of the dialog box template (either standard or extended) contains
/// additional data specifying the font to use for text in the client area and controls of the
/// dialog box. If possible, the system selects a font according to the specified font data. The
/// system passes a handle to the font to the dialog box and to each control by sending them the
/// [`WM_SETFONT`] message.
///
/// If neither [`DS_SETFONT`] nor [`DS_SHELLFONT`] is specified, the dialog box template does not
/// include the font data.
pub const DS_SETFONT: DWORD = 0x40;

/// Causes the system to use the [`SetForegroundWindow`] function to bring the dialog box to the
/// foreground. This style is useful for modal dialog boxes that require immediate attention from
/// the user regardless of whether the owner window is the foreground window.
///
/// The system restricts which processes can set the foreground window.
pub const DS_SETFOREGROUND: DWORD = 0x200;

/// Indicates that the dialog box should use the system font. The typeface member of the extended
/// dialog box template must be set to MS Shell Dlg. Otherwise, this style has no effect. It is
/// also recommended that you use the DIALOGEX Resource, rather than the DIALOG Resource.
///
/// If neither [`DS_SHELLFONT`] nor [`DS_SETFONT`] is specified, the extended dialog box template
/// does not include the font data.
pub const DS_SHELLFONT: DWORD = DS_SETFONT | DS_FIXEDSYS;

/// This style is obsolete and is included for compatibility with 16-bit versions of Windows. If
/// you specify this style, the system creates the dialog box with the [`WS_EX_TOPMOST`] style.
/// This style does not prevent the user from accessing other windows on the desktop.
///
/// Do not combine this style with the [`DS_CONTROL`] style.
pub const DS_SYSMODAL: DWORD = 0x02;
