use crate::{DefWindowProc, HBRUSH, HCURSOR, HICON, HINSTANCE, LPCWSTR, UINT, WNDPROC};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    BeginPaint, RegisterClass, RegisterClassEx, UnregisterClass, DLGWINDOWEXTRA, MAKEINTRESOURCE,
    PAINTSTRUCT, WM_ERASEBKGND, WNDCLASSEX,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

/// Contains window class information. It is used with the [`RegisterClassEx`] and
/// [`GetClassInfoEx`] functions.
///
/// The [`WNDCLASSEX`] structure is similar to the [`WNDCLASS`] structure. There are two
/// differences. [`WNDCLASSEX`] includes the size member, which specifies the size of the
/// structure, and the `icon_sm` member, which contains a handle to a small icon associated with
/// the window class.
#[repr(C)]
pub struct WNDCLASSEXW {
    /// The size, in bytes, of this structure. Set this member to
    /// `std::mem::size_of::<WNDCLASSEXW>()`. Be sure to set this member before calling the
    /// [`GetClassInfoEx`] function.
    pub size: UINT,

    /// The class style(s). This member can be any combination of the Class Styles.
    pub style: UINT,

    /// A pointer to the window procedure. You must use the [`CallWindowProc`] function to call the
    /// window procedure.
    pub wnd_proc: WNDPROC,

    /// The number of extra bytes to allocate following the window-class structure. The system
    /// initializes the bytes to zero.
    pub cls_extra: c_int,

    /// The number of extra bytes to allocate following the window instance. The system initializes
    /// the bytes to zero. If an application uses [`WNDCLASSEX`] to register a dialog box created
    /// by using the `CLASS` directive in the resource file, it must set this member to
    /// [`DLGWINDOWEXTRA`].
    pub wnd_extra: c_int,

    /// A handle to the instance that contains the window procedure for the class.
    pub instance: HINSTANCE,

    /// A handle to the class icon. This member must be a handle to an icon resource. If this
    /// member is [`null_mut`], the system provides a default icon.
    pub icon: HICON,

    /// A handle to the class cursor. This member must be a handle to a cursor resource. If this
    /// member is [`null_mut`], an application must explicitly set the cursor shape whenever the
    /// mouse moves into the application's window.
    pub cursor: HCURSOR,

    /// A handle to the class background brush. This member can be a handle to the brush to be used
    /// for painting the background, or it can be a color value.
    ///
    /// The system automatically deletes class background brushes when the class is unregistered by
    /// using [`UnregisterClass`]. An application should not delete these brushes.
    ///
    /// When this member is [`null_mut`], an application must paint its own background whenever it
    /// is requested to paint in its client area. To determine whether the background must be
    /// painted, an application can either process the [`WM_ERASEBKGND`] message or test the
    /// `erase` member of the [`PAINTSTRUCT`] structure filled by the [`BeginPaint`] function.
    pub background: HBRUSH,

    /// Pointer to a null-terminated character string that specifies the resource name of the class
    /// menu, as the name appears in the resource file. If you use an integer to identify the menu,
    /// use the [`MAKEINTRESOURCE`] macro. If this member is [`null`], windows belonging to this
    /// class have no default menu.
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
    /// length, the [`RegisterClassEx`] function will fail.
    pub class_name: LPCWSTR,

    /// A handle to a small icon that is associated with the window class. If this member is
    /// [`null_mut`], the system searches the icon resource specified by the `icon` member for an
    /// icon of the appropriate size to use as the small icon.
    pub icon_sm: HICON,
}

impl Default for WNDCLASSEXW {
    fn default() -> Self {
        WNDCLASSEXW {
            size: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            wnd_proc: DefWindowProc,
            cls_extra: 0,
            wnd_extra: 0,
            instance: null_mut(),
            icon: null_mut(),
            cursor: null_mut(),
            background: null_mut(),
            menu_name: null(),
            class_name: null(),
            icon_sm: null_mut(),
        }
    }
}
