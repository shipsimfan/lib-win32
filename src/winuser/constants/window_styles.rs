use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    IsDialogMessage, SetWindowLong, SetWindowPos, ShowWindow, WM_PAINT, WS_EX_CONTEXTHELP,
};

/// The window has a thin-line border
pub const WS_BORDER: DWORD = 0x00800000;

/// The window has a title bar (includes the [`WS_BORDER`] style).
pub const WS_CAPTION: DWORD = 0x00C00000;

/// The window is a child window. A window with this style cannot have a menu bar. This style
/// cannot be used with the [`WS_POPUP`] style.
pub const WS_CHILD: DWORD = 0x40000000;

/// Same as the [`WS_CHILD`] style.
pub const WS_CHILDWINDOW: DWORD = 0x40000000;

/// Excludes the area occupied by child windows when drawing occurs within the parent window. This
/// style is used when creating the parent window.
pub const WS_CLIPCHILDREN: DWORD = 0x02000000;

/// Clips child windows relative to each other; that is, when a particular child window receives a
/// [`WM_PAINT`] message, the [`WS_CLIPSIBLINGS`] style clips all other overlapping child windows
/// out of the region of the child window to be updated. If [`WS_CLIPSIBLINGS`] is not specified
/// and child windows overlap, it is possible, when drawing within the client area of a child
/// window, to draw within the client area of a neighboring child window.
pub const WS_CLIPSIBLINGS: DWORD = 0x04000000;

/// The window is initially disabled. A disabled window cannot receive input from the user. To
/// change this after a window has been created, use the [`EnableWindow`] function.
pub const WS_DISABLED: DWORD = 0x08000000;

/// The window has a border of a style typically used with dialog boxes. A window with this style
/// cannot have a title bar.
pub const WS_DLGFRAME: DWORD = 0x00400000;

/// The window is the first control of a group of controls. The group consists of this first
/// control and all controls defined after it, up to the next control with the [`WS_GROUP`] style.
/// The first control in each group usually has the [`WS_TABSTOP`] style so that the user can move
/// from group to group. The user can subsequently change the keyboard focus from one control in
/// the group to the next control in the group by using the direction keys.
///
/// You can turn this style on and off to change dialog box navigation. To change this style after
/// a window has been created, use the [`SetWindowLong`] function.
pub const WS_GROUP: DWORD = 0x00020000;

/// The window has a horizontal scroll bar.
pub const WS_HSCROLL: DWORD = 0x00100000;

/// The window is initially minimized. Same as the [`WS_MINIMIZE`] style.
pub const WS_ICONIC: DWORD = 0x20000000;

/// The window is initially maximized.
pub const WS_MAXIMIZE: DWORD = 0x01000000;

/// The window has a maximize button. Cannot be combined with the [`WS_EX_CONTEXTHELP`] style. The
/// [`WS_SYSMENU`] style must also be specified.
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;

/// The window is initially minimized. Same as the [`WS_ICONIC`] style.
pub const WS_MINIMIZE: DWORD = 0x20000000;

/// The window has a minimize button. Cannot be combined with the [`WS_EX_CONTEXTHELP`] style. The
/// [`WS_SYSMENU`] style must also be specified.
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;

/// The window is an overlapped window. An overlapped window has a title bar and a border. Same as
/// the [`WS_TILED`] style.
pub const WS_OVERLAPPED: DWORD = 0x00000000;

/// The window is an overlapped window. Same as the [`WS_TILEDWINDOW`] style.
pub const WS_OVERLAPPEDWINDOW: DWORD =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

/// The window is a pop-up window. This style cannot be used with the [`WS_CHILD`] style.
pub const WS_POPUP: DWORD = 0x80000000;

/// The window is a pop-up window. The [`WS_CAPTION`] and [`WS_POPUPWINDOW`] styles must be
/// combined to make the window menu visible.
pub const WS_POPUPWINDOW: DWORD = WS_POPUP | WS_BORDER | WS_SYSMENU;

/// The window has a sizing border. Same as the [`WS_THICKFRAME`] style.
pub const WS_SIZEBOX: DWORD = 0x00040000;

/// The window has a window menu on its title bar. The [`WS_CAPTION`] style must also be specified.
pub const WS_SYSMENU: DWORD = 0x00080000;

/// The window is a control that can receive the keyboard focus when the user presses the `TAB`
/// key. Pressing the `TAB` key changes the keyboard focus to the next control with the
/// [`WS_TABSTOP`] style.
///
/// You can turn this style on and off to change dialog box navigation. To change this style after
/// a window has been created, use the [`SetWindowLong`] function. For user-created windows and
/// modeless dialogs to work with tab stops, alter the message loop to call the [`IsDialogMessage`]
/// function.
pub const WS_TABSTOP: DWORD = 0x00010000;

/// The window has a sizing border. Same as the [`WS_SIZEBOX`] style.
pub const WS_THICKFRAME: DWORD = 0x00040000;

/// The window is an overlapped window. An overlapped window has a title bar and a border. Same as
/// the [`WS_OVERLAPPED`] style.
pub const WS_TILED: DWORD = 0x00000000;

/// The window is an overlapped window. Same as the [`WS_OVERLAPPEDWINDOW`] style.
pub const WS_TILEDWINDOW: DWORD =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

/// The window is initially visible.
///
/// This style can be turned on and off by using the [`ShowWindow`] or [`SetWindowPos`] function.
pub const WS_VISIBLE: DWORD = 0x10000000;

/// The window has a vertical scroll bar.
pub const WS_VSCROLL: DWORD = 0x00200000;
