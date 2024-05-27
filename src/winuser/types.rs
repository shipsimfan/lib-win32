use crate::{HWND, LPARAM, LRESULT, UINT, WPARAM};

/// A callback function, which you define in your application, that processes messages sent to a
/// window. The [`WNDPROC`] type defines a pointer to this callback function.
pub type WNDPROC =
    extern "system" fn(wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;
