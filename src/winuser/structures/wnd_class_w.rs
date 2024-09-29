use crate::{DefWindowProcW, HBRUSH, HCURSOR, HICON, HINSTANCE, LPCWSTR, UINT, WNDPROC};
use std::{
    ffi::c_int,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    BeginPaint, RegisterClass, RegisterClassEx, UnregisterClass, MAKEINTRESOURCE, PAINTSTRUCT,
    WM_ERASEBKGND, WNDCLASS, WNDCLASSEX,
};

/// Contains the window class attributes that are registered by the [`RegisterClass`] function.
///
/// This structure has been superseded by the [`WNDCLASSEX`] structure used with the
/// [`RegisterClassEx`] function. You can still use [`WNDCLASS`] and [`RegisterClass`] if you do
/// not need to set the small icon associated with the window class.
#[repr(C)]
pub struct WNDCLASSW {
    /// The class style(s). This member can be any combination of the Class Styles.
    pub style: UINT,

    /// A pointer to the window procedure. You must use the [`CallWindowProc`] function to call the
    /// window procedure.
    pub wnd_proc: WNDPROC,

    /// The number of extra bytes to allocate following the window-class structure. The system
    /// initializes the bytes to zero.
    pub cls_extra: c_int,

    /// The number of extra bytes to allocate following the window instance. The system initializes
    /// the bytes to zero. If an application uses [`WNDCLASS`] to register a dialog box created by
    /// using the CLASS directive in the resource file, it must set this member to
    /// [`DLGWINDOWEXTRA`].
    pub wnd_extra: c_int,

    /// A handle to the instance that contains the window procedure for the class.
    pub instance: HINSTANCE,

    /// A handle to the class icon. This member must be a handle to an icon resource. If this
    /// member is [`null_mut`], the system provides a default icon.
    pub icon: HICON,

    /// The resource name of the class menu, as the name appears in the resource file. If you use
    /// an integer to identify the menu, use the [`MAKEINTRESOURCE`] macro. If this member is
    /// [`null_mut`], windows belonging to this class have no default menu.
    pub cursor: HCURSOR,

    /// A handle to the class background brush. This member can be a handle to the physical brush
    /// to be used for painting the background, or it can be a color value. A color value must be
    /// one of the following standard system colors (the value 1 must be added to the chosen
    /// color). If a color value is given, you must convert it to one of the following [`HBRUSH`]
    /// types:
    /// - [`COLOR_ACTIVEBORDER`]
    /// - [`COLOR_ACTIVECAPTION`]
    /// - [`COLOR_APPWORKSPACE`]
    /// - [`COLOR_BACKGROUND`]
    /// - [`COLOR_BTNFACE`]
    /// - [`COLOR_BTNSHADOW`]
    /// - [`COLOR_BTNTEXT`]
    /// - [`COLOR_CAPTIONTEXT`]
    /// - [`COLOR_GRAYTEXT`]
    /// - [`COLOR_HIGHLIGHT`]
    /// - [`COLOR_HIGHLIGHTTEXT`]
    /// - [`COLOR_INACTIVEBORDER`]
    /// - [`COLOR_INACTIVECAPTION`]
    /// - [`COLOR_MENU`]
    /// - [`COLOR_MENUTEXT`]
    /// - [`COLOR_SCROLLBAR`]
    /// - [`COLOR_WINDOW`]
    /// - [`COLOR_WINDOWFRAME`]
    /// - [`COLOR_WINDOWTEXT`]
    ///
    /// The system automatically deletes class background brushes when the class is unregistered by
    /// using [`UnregisterClass`]. An application should not delete these brushes.
    ///
    /// When this member is [`null_mut`], an application must paint its own background whenever it
    /// is requested to paint in its client area. To determine whether the background must be
    /// painted, an application can either process the [`WM_ERASEBKGND`] message or test the
    /// `erase` member of the [`PAINTSTRUCT`] structure filled by the [`BeginPaint`] function.
    pub background: HBRUSH,

    /// The resource name of the class menu, as the name appears in the resource file. If you use
    /// an integer to identify the menu, use the [`MAKEINTRESOURCE`] macro. If this member is
    /// [`null`], windows belonging to this class have no default menu.
    pub menu_name: LPCWSTR,

    /// A pointer to a null-terminated string or is an atom. If this parameter is an atom, it must
    /// be a class atom created by a previous call to the [`RegisterClass`] or [`RegisterClassEx`]
    /// function. The atom must be in the low-order word of `class_name`; the high-order word must
    /// be zero.
    ///
    /// If `class_name` is a string, it specifies the window class name. The class name can be any
    /// name registered with [`RegisterClass`] or [`RegisterClassEx`], or any of the predefined
    /// control-class names.
    ///
    /// The maximum length for `class_name` is 256. If `class_name` is greater than the maximum
    /// length, the [`RegisterClass`] function will fail.
    pub class_name: LPCWSTR,
}

impl Default for WNDCLASSW {
    fn default() -> Self {
        WNDCLASSW {
            style: 0,
            wnd_proc: DefWindowProcW,
            cls_extra: 0,
            wnd_extra: 0,
            instance: null_mut(),
            icon: null_mut(),
            cursor: null_mut(),
            background: null_mut(),
            menu_name: null(),
            class_name: null(),
        }
    }
}
