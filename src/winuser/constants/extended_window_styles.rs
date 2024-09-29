use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{WM_HELP, WM_PARENTNOTIFY, WS_CAPTION, WS_MAXIMIZEBOX, WS_MINIMIZEBOX};

/// The window accepts drag-drop files.
pub const WS_EX_ACCEPTFILES: DWORD = 0x00000010;

/// Forces a top-level window onto the taskbar when the window is visible.
pub const WS_EX_APPWINDOW: DWORD = 0x00040000;

/// The window has a border with a sunken edge.
pub const WS_EX_CLIENTEDGE: DWORD = 0x00000200;

/// Windows 2000: This style is not supported.
pub const WS_EX_COMPOSITED: DWORD = 0x02000000;

/// The title bar of the window includes a question mark. When the user clicks the question mark,
/// the cursor changes to a question mark with a pointer. If the user then clicks a child window,
/// the child receives a [`WM_HELP`] message. The child window should pass the message to the
/// parent window procedure, which should call the [`WinHelp`] function using the [`HELP_WM_HELP`]
/// command. The Help application displays a pop-up window that typically contains help for the
/// child window.
///
/// [`WS_EX_CONTEXTHELP`] cannot be used with the [`WS_MAXIMIZEBOX`] or [`WS_MINIMIZEBOX`] styles.
pub const WS_EX_CONTEXTHELP: DWORD = 0x00000400;

/// The window itself contains child windows that should take part in dialog box navigation. If
/// this style is specified, the dialog manager recurses into children of this window when
/// performing navigation operations such as handling the `TAB` key, an arrow key, or a keyboard
/// mnemonic.
pub const WS_EX_CONTROLPARENT: DWORD = 0x00010000;

/// The window has a double border; the window can, optionally, be created with a title bar by
/// specifying the [`WS_CAPTION`] style in the `style` parameter.
pub const WS_EX_DLGMODALFRAME: DWORD = 0x00000001;

/// Windows 8: The [`WS_EX_LAYERED`] style is supported for top-level windows and child windows.
/// Previous Windows versions support [`WS_EX_LAYERED`] only for top-level windows.
pub const WS_EX_LAYERED: DWORD = 0x00080000;

/// If the shell language is Hebrew, Arabic, or another language that supports reading order
/// alignment, the horizontal origin of the window is on the right edge. Increasing horizontal
/// values advance to the left.
pub const WS_EX_LAYOUTRTL: DWORD = 0x00400000;

/// The window has generic left-aligned properties. This is the default.
pub const WS_EX_LEFT: DWORD = 0x00000000;

/// If the shell language is Hebrew, Arabic, or another language that supports reading order
/// alignment, the vertical scroll bar (if present) is to the left of the client area. For other
/// languages, the style is ignored.
pub const WS_EX_LEFTSCROLLBAR: DWORD = 0x00004000;

/// The window text is displayed using left-to-right reading-order properties. This is the default.
pub const WS_EX_LTRREADING: DWORD = 0x00000000;

/// The window is a MDI child window.
pub const WS_EX_MDICHILD: DWORD = 0x00000040;

/// The window does not appear on the taskbar by default. To force the window to appear on the
/// taskbar, use the [`WS_EX_APPWINDOW`] style.
pub const WS_EX_NOACTIVATE: DWORD = 0x08000000;

/// The window does not pass its window layout to its child windows.
pub const WS_EX_NOINHERITLAYOUT: DWORD = 0x00100000;

/// The child window created with this style does not send the [`WM_PARENTNOTIFY`] message to its
/// parent window when it is created or destroyed.
pub const WS_EX_NOPARENTNOTIFY: DWORD = 0x00000004;

/// The window does not render to a redirection surface. This is for windows that do not have
/// visible content or that use mechanisms other than surfaces to provide their visual.
pub const WS_EX_NOREDIRECTIONBITMAP: DWORD = 0x00200000;

/// The window is an overlapped window.
pub const WS_EX_OVERLAPPEDWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;

/// The window is palette window, which is a modeless dialog box that presents an array of
/// commands.
pub const WS_EX_PALETTEWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;

/// Using the [`WS_EX_RIGHT`] style for static or edit controls has the same effect as using the
/// [`SS_RIGHT`] or [`ES_RIGHT`] style, respectively. Using this style with button controls has the
/// same effect as using [`BS_RIGHT`] and [`BS_RIGHTBUTTON`] styles.
pub const WS_EX_RIGHT: DWORD = 0x00001000;

/// The vertical scroll bar (if present) is to the right of the client area. This is the default.
pub const WS_EX_RIGHTSCROLLBAR: DWORD = 0x00000000;

/// If the shell language is Hebrew, Arabic, or another language that supports reading-order
/// alignment, the window text is displayed using right-to-left reading-order properties. For other
/// languages, the style is ignored.
pub const WS_EX_RTLREADING: DWORD = 0x00002000;

/// The window has a three-dimensional border style intended to be used for items that do not
/// accept user input.
pub const WS_EX_STATICEDGE: DWORD = 0x00020000;

/// The window is intended to be used as a floating toolbar. A tool window has a title bar that is
/// shorter than a normal title bar, and the window title is drawn using a smaller font. A tool
/// window does not appear in the taskbar or in the dialog that appears when the user presses
/// `ALT+TAB`. If a tool window has a system menu, its icon is not displayed on the title bar.
/// However, you can display the system menu by right-clicking or by typing `ALT+SPACE`.
pub const WS_EX_TOOLWINDOW: DWORD = 0x00000080;

/// The window should be placed above all non-topmost windows and should stay above them, even when
/// the window is deactivated. To add or remove this style, use the [`SetWindowPos`] function.
pub const WS_EX_TOPMOST: DWORD = 0x00000008;

/// To achieve transparency without these restrictions, use the [`SetWindowRgn`] function.
pub const WS_EX_TRANSPARENT: DWORD = 0x00000020;

/// The window has a border with a raised edge.
pub const WS_EX_WINDOWEDGE: DWORD = 0x00000100;
