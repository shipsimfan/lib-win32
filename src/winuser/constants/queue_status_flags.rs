use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetMessage, PeekMessage, PostMessage, WM_HOTKEY, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONUP,
    WM_MOUSEMOVE, WM_PAINT, WM_RBUTTONDOWN, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_TIMER,
};

/// A [`WM_KEYUP`], [`WM_KEYDOWN`], [`WM_SYSKEYUP`], or [`WM_SYSKEYDOWN`] message is in the queue.
pub const QS_KEY: UINT = 0x0001;

/// A [`WM_MOUSEMOVE`] message is in the queue.
pub const QS_MOUSEMOVE: UINT = 0x0002;

/// A mouse-button message ([`WM_LBUTTONUP`], [`WM_RBUTTONDOWN`], and so on).
pub const QS_MOUSEBUTTON: UINT = 0x0004;

/// A posted message (other than those listed here) is in the queue. For more information, see
/// [`PostMessage`]. This value is cleared when you call [`GetMessage`] or [`PeekMessage`], whether
/// or not you are filtering messages.
pub const QS_POSTMESSAGE: UINT = 0x0008;

/// A [`WM_TIMER`] message is in the queue.
pub const QS_TIMER: UINT = 0x0010;

/// A [`WM_PAINT`] message is in the queue.
pub const QS_PAINT: UINT = 0x0020;

/// A message sent by another thread or application is in the queue. For more information, see
/// [`SendMessage`].
pub const QS_SENDMESSAGE: UINT = 0x0040;

/// A [`WM_HOTKEY`] message is in the queue.
pub const QS_HOTKEY: UINT = 0x0080;

/// A posted message (other than those listed here) is in the queue. For more information, see
/// [`PostMessage`]. This value is cleared when you call [`GetMessage`] or [`PeekMessage`] without
/// filtering messages.
pub const QS_ALLPOSTMESSAGE: UINT = 0x0100;

/// Windows XP and newer: A raw input message is in the queue.
pub const QS_RAWINPUT: UINT = 0x0400;

/// Windows 8 and newer: A touch input message is in the queue.
pub const QS_TOUCH: UINT = 0x0800;

/// Windows 8 and newer: A pointer input message is in the queue.
pub const QS_POINTER: UINT = 0x1000;

/// A [`WM_MOUSEMOVE`] message or mouse-button message ([`WM_LBUTTONUP`], [`WM_RBUTTONDOWN`], and
/// so on).
pub const QS_MOUSE: UINT = QS_MOUSEMOVE | QS_MOUSEBUTTON;

/// An input message is in the queue.
pub const QS_INPUT: UINT = QS_MOUSE | QS_KEY | QS_RAWINPUT | QS_TOUCH | QS_POINTER;

/// An input, [`WM_TIMER`], [`WM_PAINT`], [`WM_HOTKEY`], or posted message is in the queue.
pub const QS_ALLEVENTS: UINT = QS_INPUT | QS_POSTMESSAGE | QS_TIMER | QS_PAINT | QS_HOTKEY;

/// Any message is in the queue.
pub const QS_ALLINPUT: UINT =
    QS_INPUT | QS_POSTMESSAGE | QS_TIMER | QS_PAINT | QS_HOTKEY | QS_SENDMESSAGE;
