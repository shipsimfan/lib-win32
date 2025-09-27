// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateWindow, CreateWindowEx, DestroyWindow, GetLastError, RegisterClass, RegisterClassEx,
    ShowWindow, CLIENTCREATESTRUCT, CREATESTRUCT, CW_USEDEFAULT, HWND_MESSAGE, MDICREATESTRUCT,
    SW_SHOW, WM_CREATE, WM_GETMINMAXINFO, WM_NCCREATE, WS_VISIBLE, SS_ICON
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Creates an overlapped, pop-up, or child window. It specifies the window class, window title, 
/// window style, and (optionally) the initial position and size of the window. The function also 
/// specifies the window's parent or owner, if any, and the window's menu.
///
/// To use extended window styles in addition to the styles supported by [`CreateWindow`], use the 
/// [`CreateWindowEx`] function.
/// 
/// # Parameters
///  * `class_name` - A null-terminated string or a class atom created by a previous call to the 
///                   [`RegisterClass`] or [`RegisterClassEx`] function. The atom must be in the 
///                   low-order word of `class_name`; the high-order word must be zero. If 
///                   `class_name` is a string, it specifies the window class name. The class name 
///                   can be any name registered with [`RegisterClass`] or [`RegisterClassEx`], 
///                   provided that the module that registers the class is also the module that 
///                   creates the window. The class name can also be any of the predefined system 
///                   class names.
///  * `window_name` - The window name. If the window style specifies a title bar, the window title
///                    pointed to by `window_name` is displayed in the title bar. When using 
///                    [`CreateWindow`] to create controls, such as buttons, check boxes, and 
///                    static controls, use `window_name` to specify the text of the control. When 
///                    creating a static control with the [`SS_ICON`] style, use `window_name` to 
///                    specify the icon name or identifier. To specify an identifier, use the 
///                    syntax "#num".
///  * `style` - The style of the window being created.
///  * `x` - The initial horizontal position of the window. For an overlapped or pop-up window, the
///          x parameter is the initial x-coordinate of the window's upper-left corner, in screen 
///          coordinates. For a child window, x is the x-coordinate of the upper-left corner of the
///          window relative to the upper-left corner of the parent window's client area. If this 
///          parameter is set to [`CW_USEDEFAULT`], the system selects the default position for the
///          window's upper-left corner and ignores the y parameter. [`CW_USEDEFAULT`] is valid 
///          only for overlapped windows; if it is specified for a pop-up or child window, the x 
///          and y parameters are set to zero.
///  * `y` - The initial vertical position of the window. For an overlapped or pop-up window, the y
///          parameter is the initial y-coordinate of the window's upper-left corner, in screen 
///          coordinates. For a child window, y is the initial y-coordinate of the upper-left 
///          corner of the child window relative to the upper-left corner of the parent window's 
///          client area. For a list box, y is the initial y-coordinate of the upper-left corner of
///          the list box's client area relative to the upper-left corner of the parent window's 
///          client area. If an overlapped window is created with the [`WS_VISIBLE`] style bit set 
///          and the x parameter is set to [`CW_USEDEFAULT`], then the y parameter determines how 
///          the window is shown. If the y parameter is [`CW_USEDEFAULT`], then the window manager 
///          calls [`ShowWindow`] with the [`SW_SHOW`] flag after the window has been created. If 
///          the y parameter is some other value, then the window manager calls [`ShowWindow`] with
///          that value as the `cmd_show` parameter.
///  * `width` - The width, in device units, of the window. For overlapped windows, `width` is 
///              either the window's width, in screen coordinates, or [`CW_USEDEFAULT`]. If `width`
///              is [`CW_USEDEFAULT`], the system selects a default width and height for the 
///              window; the default width extends from the initial x-coordinate to the right edge 
///              of the screen, and the default height extends from the initial y-coordinate to the
///              top of the icon area. [`CW_USEDEFAULT`] is valid only for overlapped windows; if 
///              [`CW_USEDEFAULT`] is specified for a pop-up or child window, `width` and `height` 
///              are set to zero.
///  * `height` - The height, in device units, of the window. For overlapped windows, `height` is 
///               the window's height, in screen coordinates. If `width` is set to 
///               [`CW_USEDEFAULT`], the system ignores `height`.
///  * `parent` - A handle to the parent or owner window of the window being created. To create a
///               child window or an owned window, supply a valid window handle. This parameter is 
///               optional for pop-up windows. To create a message-only window, supply 
///               [`HWND_MESSAGE`] or a handle to an existing message-only window.
///  * `menu` - A handle to a menu, or specifies a child-window identifier depending on the window
///             style. For an overlapped or pop-up window, `menu` identifies the menu to be used 
///             with the window; it can be [`null_mut`] if the class menu is to be used. For a 
///             child window, `menu` specifies the child-window identifier, an integer value used 
///             by a dialog box control to notify its parent about events. The application 
///             determines the child-window identifier; it must be unique for all child windows 
///             with the same parent window.
///  * `instance` - A handle to the instance of the module to be associated with the window.
///  * `param` - A pointer to a value to be passed to the window through the [`CREATESTRUCT`] 
///              structure (`create_params` member) pointed to by the `param` param of the 
///              [`WM_CREATE`] message. This message is sent to the created window by this function
///              before it returns. If an application calls [`CreateWindow`] to create a MDI client
///              window, `param` should point to a [`CLIENTCREATESTRUCT`] structure. If an MDI 
///              client window calls [`CreateWindow`] to create an MDI child window, `param` should
///              point to a [`MDICREATESTRUCT`] structure. `param` may be [`null_mut`] if no 
///              additional data is needed.
/// 
/// # Return Value
/// If the function succeeds, the return value is a handle to the new window.
/// 
/// If the function fails, the return value is [`null_mut`]. To get extended error information, 
/// call [`GetLastError`].
/// 
/// # Remarks
/// Before returning, [`CreateWindow`] sends a [`WM_CREATE`] message to the window procedure. For 
/// overlapped, pop-up, and child windows, [`CreateWindow`] sends [`WM_CREATE`], 
/// [`WM_GETMINMAXINFO`], and [`WM_NCCREATE`] messages to the window. The `param` parameter of the 
/// [`WM_CREATE`] message contains a pointer to a [`CREATESTRUCT`] structure. If the [`WS_VISIBLE`]
/// style is specified, [`CreateWindow`] sends the window all the messages required to activate and
/// show the window.
/// 
/// If the created window is a child window, its default position is at the bottom of the Z-order. 
/// If the created window is a top-level window, its default position is at the top of the Z-order 
/// (but beneath all topmost windows unless the created window is itself topmost).
/// 
/// For information on removing a window, see the [`DestroyWindow`] function.
#[macro_export]
macro_rules! CreateWindowA {
    (
        $class_name: expr, 
        $window_name: expr, 
        $style: expr, 
        $x: expr, 
        $y: expr, 
        $width: expr, 
        $height: expr, 
        $parent: expr, 
        $menu: expr, 
        $instance: expr, 
        $param: expr
    ) => {
        $crate::CreateWindowExA(
            0, 
            $class_name, 
            $window_name, 
            $style, 
            $x, 
            $y, 
            $width, 
            $height, 
            $parent, 
            $menu, 
            $instance, 
            $param
        )
    };
}
