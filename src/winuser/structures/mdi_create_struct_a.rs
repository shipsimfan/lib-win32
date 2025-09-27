use crate::{DWORD, HANDLE, LPARAM, LPCSTR};
use std::{
    ffi::c_int,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CW_USEDEFAULT, MDIS_ALLCHILDSTYLES, WS_HSCROLL, WS_MAXIMIZE, WS_MINIMIZE, WS_VSCROLL};

/// Contains information about the class, title, owner, location, and size of a multiple-document
/// interface (MDI) child window.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MDICREATESTRUCTA {
    /// Contains information about the class, title, owner, location, and size of a multiple-
    /// document interface (MDI) child window.
    pub class: LPCSTR,

    /// The title of the MDI child window. The system displays the title in the child window's
    /// title bar.
    pub title: LPCSTR,

    /// A handle to the instance of the application creating the MDI client window.
    pub owner: HANDLE,

    /// The initial horizontal position, in client coordinates, of the MDI child window. If this
    /// member is [`CW_USEDEFAULT`], the MDI child window is assigned the default horizontal
    /// position.
    pub x: c_int,

    /// The initial vertical position, in client coordinates, of the MDI child window. If this
    /// member is [`CW_USEDEFAULT`], the MDI child window is assigned the default vertical
    /// position.
    pub y: c_int,

    /// The initial width, in device units, of the MDI child window. If this member is
    /// [`CW_USEDEFAULT`], the MDI child window is assigned the default width.
    pub cx: c_int,

    /// The initial height, in device units, of the MDI child window. If this member is set to
    /// [`CW_USEDEFAULT`], the MDI child window is assigned the default height.
    pub cy: c_int,

    /// The style of the MDI child window. If the MDI client window was created with the
    /// [`MDIS_ALLCHILDSTYLES`] window style, this member can be any combination of the window
    /// styles listed in the Window Styles page. Otherwise, this member can be one or more of the
    /// following values:
    ///  * [`WS_MINIMIZE`] - Creates an MDI child window that is initially minimized.
    ///  * [`WS_MAXIMIZE`] - Creates an MDI child window that is initially maximized.
    ///  * [`WS_HSCROLL`] - Creates an MDI child window that has a horizontal scroll bar.
    ///  * [`WS_VSCROLL`] - Creates an MDI child window that has a vertical scroll bar.
    pub style: DWORD,

    /// An application-defined value.
    pub l_param: LPARAM,
}

impl Default for MDICREATESTRUCTA {
    fn default() -> Self {
        MDICREATESTRUCTA {
            class: null(),
            title: null(),
            owner: null_mut(),
            x: 0,
            y: 0,
            cx: 0,
            cy: 0,
            style: 0,
            l_param: 0,
        }
    }
}
