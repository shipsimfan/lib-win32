use std::ffi::c_int;

/// Hides the window and activates another window.
pub const SW_HIDE: c_int = 0;

/// Activates and displays a window. If the window is minimized, maximized, or arranged, the system
/// restores it to its original size and position. An application should specify this flag when
/// displaying the window for the first time.
pub const SW_SHOWNORMAL: c_int = 1;

/// Activates and displays a window. If the window is minimized, maximized, or arranged, the system
/// restores it to its original size and position. An application should specify this flag when
/// displaying the window for the first time.
pub const SW_NORMAL: c_int = 1;

/// Activates the window and displays it as a minimized window.
pub const SW_SHOWMINIMIZED: c_int = 2;

///Activates the window and displays it as a maximized window.
pub const SW_SHOWMAXIMIZED: c_int = 3;

///Activates the window and displays it as a maximized window.
pub const SW_MAXIMIZE: c_int = 3;

/// Displays a window in its most recent size and position. This value is similar to
/// [`SW_SHOWNORMAL`], except that the window is not activated.
pub const SW_SHOWNOACTIVATE: c_int = 4;

/// Activates the window and displays it in its current size and position.
pub const SW_SHOW: c_int = 5;

/// Minimizes the specified window and activates the next top-level window in the Z order.
pub const SW_MINIMIZE: c_int = 6;

/// Displays the window as a minimized window. This value is similar to [`SW_SHOWMINIMIZED`],
/// except the window is not activated.
pub const SW_SHOWMINNOACTIVE: c_int = 7;

/// Displays the window in its current size and position. This value is similar to [`SW_SHOW`],
/// except that the window is not activated.
pub const SW_SHOWNA: c_int = 8;

/// Activates and displays the window. If the window is minimized, maximized, or arranged, the
/// system restores it to its original size and position. An application should specify this flag
/// when restoring a minimized window.
pub const SW_RESTORE: c_int = 9;

/// Sets the show state based on the `SW_` value specified in the [`STARTUPINFO`] structure passed
/// to the [`CreateProcess`] function by the program that started the application.
pub const SW_SHOWDEFAULT: c_int = 10;

/// Minimizes a window, even if the thread that owns the window is not responding. This flag should
/// only be used when minimizing windows from a different thread.
pub const SW_FORCEMINIMIZE: c_int = 11;
