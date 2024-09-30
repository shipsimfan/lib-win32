use crate::{DWORD, HINSTANCE, HMENU, HWND, LONG, LPCWSTR, LPVOID};
use std::{
    ffi::c_int,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateWindow, CreateWindowEx, CLIENTCREATESTRUCT, MDICREATESTRUCT, SHORT};

/// Defines the initialization parameters passed to the window procedure of an application. These
/// members are identical to the parameters of the [`CreateWindowEx`] function.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CREATESTRUCTW {
    /// Contains additional data which may be used to create the window. If the window is being
    /// created as a result of a call to the [`CreateWindow`] or [`CreateWindowEx`] function, this
    /// member contains the value of the `param` parameter specified in the function call.
    ///
    /// If the window being created is a MDI client window, this member contains a pointer to a
    /// [`CLIENTCREATESTRUCT`] structure. If the window being created is a MDI child window, this
    /// member contains a pointer to an [`MDICREATESTRUCT`] structure.
    ///
    /// If the window is being created from a dialog template, this member is the address of a
    /// [`SHORT`] value that specifies the size, in bytes, of the window creation data. The value
    /// is immediately followed by the creation data.
    pub create_params: LPVOID,

    /// A handle to the module that owns the new window.
    pub instance: HINSTANCE,

    /// A handle to the menu to be used by the new window.
    pub menu: HMENU,

    /// A handle to the parent window, if the window is a child window. If the window is owned,
    /// this member identifies the owner window. If the window is not a child or owned window, this
    /// member is [`null_mut`].
    pub parent: HWND,

    /// The height of the new window, in pixels.
    pub cy: c_int,

    /// The width of the new window, in pixels.
    pub cx: c_int,

    /// The y-coordinate of the upper left corner of the new window. If the new window is a child
    /// window, coordinates are relative to the parent window. Otherwise, the coordinates are
    /// relative to the screen origin.
    pub y: c_int,

    /// The x-coordinate of the upper left corner of the new window. If the new window is a child
    /// window, coordinates are relative to the parent window. Otherwise, the coordinates are
    /// relative to the screen origin.
    pub x: c_int,

    /// The style for the new window.
    pub style: LONG,

    /// The name of the new window.
    pub name: LPCWSTR,

    /// A pointer to a null-terminated string or an atom that specifies the class name of the new
    /// window.
    pub class: LPCWSTR,

    /// The extended window style for the new window.
    pub ex_style: DWORD,
}

impl Default for CREATESTRUCTW {
    fn default() -> Self {
        CREATESTRUCTW {
            create_params: null_mut(),
            instance: null_mut(),
            menu: null_mut(),
            parent: null_mut(),
            cy: 0,
            cx: 0,
            y: 0,
            x: 0,
            style: 0,
            name: null(),
            class: null(),
            ex_style: 0,
        }
    }
}
