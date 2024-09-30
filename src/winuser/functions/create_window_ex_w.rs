use crate::{DWORD, HINSTANCE, HMENU, HWND, LPCWSTR, LPVOID};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateWindow, CreateWindowEx, GetLastError, RegisterClass, RegisterClassEx, ShowWindow,
    CLIENTCREATESTRUCT, CREATESTRUCT, CW_USEDEFAULT, FALSE, HWND_MESSAGE, MDICREATESTRUCT, SW_SHOW,
    WM_CREATE, WM_MOUSEACTIVATE, WM_NCACTIVATE, WM_NCCALCSIZE, WM_NCCREATE, WS_EX_COMPOSITED,
    WS_EX_NOACTIVATE, WS_EX_TRANSPARENT, WS_VISIBLE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// Creates an overlapped, pop-up, or child window with an extended window style; otherwise,
    /// this function is identical to the [`CreateWindow`] function.
    ///
    /// # Parameters
    ///  * `ex_style` - The extended window style of the window being created.
    ///  * `class_name` - A null-terminated string or a class atom created by a previous call to
    ///                   the [`RegisterClass`] or [`RegisterClassEx`] function. The atom must be
    ///                   in the low-order word of `class_name`; the high-order word must be zero.
    ///                   If `class_name` is a string, it specifies the window class name. The
    ///                   class name can be any name registered with [`RegisterClass`] or
    ///                   [`RegisterClassEx`], provided that the module that registers the class
    ///                   is also the module that creates the window. The class name can also be
    ///                   any of the predefined system class names.
    ///  * `window_name` - The window name. If the window style specifies a title bar, the window
    ///                    title pointed to by `window_name` is displayed in the title bar. When
    ///                    using [`CreateWindow`] to create controls, such as buttons, check boxes,
    ///                    and static controls, use `window_name` to specify the text of the
    ///                    control. When creating a static control with the [`SS_ICON`] style, use
    ///                    `window_name` to specify the icon name or identifier. To specify an
    ///                    identifier, use the syntax "#num".
    ///  * `style` - The style of the window being created. This parameter can be a combination of
    ///              the window style values, plus the control styles indicated in the Remarks
    ///              section.
    ///  * `x` - The initial horizontal position of the window. For an overlapped or pop-up window,
    ///          the `x` parameter is the initial x-coordinate of the window's upper-left corner,
    ///          in screen coordinates. For a child window, `x` is the x-coordinate of the
    ///          upper-left corner of the window relative to the upper-left corner of the parent
    ///          window's client area. If `x` is set to [`CW_USEDEFAULT`], the system selects the
    ///          default position for the window's upper-left corner and ignores the `y` parameter.
    ///          [`CW_USEDEFAULT`] is valid only for overlapped windows; if it is specified for a
    ///          pop-up or child window, the `x` and `y` parameters are set to zero.
    ///  * `y` - The initial vertical position of the window. For an overlapped or pop-up window,
    ///          the `y` parameter is the initial y-coordinate of the window's upper-left corner,
    ///          in screen coordinates. For a child window, `y` is the initial y-coordinate of the
    ///          upper-left corner of the child window relative to the upper-left corner of the
    ///          parent window's client area. For a list box `y` is the initial y-coordinate of the
    ///          upper-left corner of the list box's client area relative to the upper-left corner
    ///          of the parent window's client area. If an overlapped window is created with the
    ///          [`WS_VISIBLE`] style bit set and the `x` parameter is set to [`CW_USEDEFAULT`],
    ///          then the `y` parameter determines how the window is shown. If the `y` parameter is
    ///          [`CW_USEDEFAULT`], then the window manager calls [`ShowWindow`] with the
    ///          [`SW_SHOW`] flag after the window has been created. If the `y` parameter is some
    ///          other value, then the window manager calls [`ShowWindow`] with that value as the
    ///          `cmd_show` parameter.
    ///  * `width` - The width, in device units, of the window. For overlapped windows, `width` is
    ///              the window's width, in screen coordinates, or [`CW_USEDEFAULT`]. If `width` is
    ///              [`CW_USEDEFAULT`], the system selects a default width and height for the
    ///              window; the default width extends from the initial x-coordinates to the right
    ///              edge of the screen; the default height extends from the initial y-coordinate
    ///              to the top of the icon area. [`CW_USEDEFAULT`] is valid only for overlapped
    ///              windows; if [`CW_USEDEFAULT`] is specified for a pop-up or child window, the
    ///              `width` and `height` parameter are set to zero.
    ///  * `height` - The height, in device units, of the window. For overlapped windows, `height`
    ///               is the window's height, in screen coordinates. If the `width` parameter is
    ///               set to [`CW_USEDEFAULT`], the system ignores `height`.
    ///  * `parent` - A handle to the parent or owner window of the window being created. To create
    ///               a child window or an owned window, supply a valid window handle. This
    ///               parameter is optional for pop-up windows. To create a message-only window,
    ///               supply [`HWND_MESSAGE`] or a handle to an existing message-only window.
    ///  * `menu` - A handle to a menu, or specifies a child-window identifier, depending on the
    ///             window style. For an overlapped or pop-up window, `menu` identifies the menu to
    ///             be used with the window; it can be [`null_mut`] if the class menu is to be
    ///             used. For a child window, `menu` specifies the child-window identifier, an
    ///             integer value used by a dialog box control to notify its parent about events.
    ///             The application determines the child-window identifier; it must be unique for
    ///             all child windows with the same parent window.
    ///  * `instance` - A handle to the instance of the module to be associated with the window.
    ///  * `param` - Pointer to a value to be passed to the window through the [`CREATESTRUCT`]
    ///              structure (`create_params` member) pointed to by the `l_param` param of the
    ///              [`WM_CREATE`] message. This message is sent to the created window by this
    ///              function before it returns. If an application calls [`CreateWindow`] to create
    ///              a MDI client window, `param` should point to a [`CLIENTCREATESTRUCT`]
    ///              structure. If an MDI client window calls [`CreateWindow`] to create an MDI
    ///              child window, `param` should point to a [`MDICREATESTRUCT`] structure. `param`
    ///              may be [`null_mut`] if no additional data is needed.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the new window.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// The [`CreateWindowEx`] function sends [`WM_NCCREATE`], [`WM_NCCALCSIZE`], and [`WM_CREATE`]
    /// messages to the window being created.
    ///
    /// If the created window is a child window, its default position is at the bottom of the
    /// Z-order. If the created window is a top-level window, its default position is at the top of
    /// the Z-order (but beneath all topmost windows unless the created window is itself topmost).
    ///
    /// The [`WS_EX_NOACTIVATE`] value for dwExStyle prevents foreground activation by the system.
    /// To prevent queue activation when the user clicks on the window, you must process the
    /// [`WM_MOUSEACTIVATE`] message appropriately. To bring the window to the foreground or to
    /// activate it programmatically, use [`SetForegroundWindow`] or [`SetActiveWindow`]. Returning
    /// [`FALSE`] to [`WM_NCACTIVATE`] prevents the window from losing queue activation. However,
    /// the return value is ignored at activation time.
    ///
    /// With [`WS_EX_COMPOSITED`] set, all descendants of a window get bottom-to-top painting order
    /// using double-buffering. Bottom-to-top painting order allows a descendent window to have
    /// translucency (alpha) and transparency (color-key) effects, but only if the descendent
    /// window also has the [`WS_EX_TRANSPARENT`] bit set. Double-buffering allows the window and
    /// its descendents to be painted without flicker.
    pub fn CreateWindowExW(
        ex_style: DWORD,
        class_name: LPCWSTR,
        window_name: LPCWSTR,
        style: DWORD,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        parent: HWND,
        menu: HMENU,
        instance: HINSTANCE,
        param: LPVOID,
    ) -> HWND;
}
