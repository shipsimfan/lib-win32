use crate::{HANDLE, UINT};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::WM_COMMAND;

/// Contains information about the menu and first multiple-document interface (MDI) child window of
/// an MDI client window. An application passes a pointer to this structure as the `param`
/// parameter of the [`CreateWindow`] function when creating an MDI client window.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CLIENTCREATESTRUCT {
    /// A handle to the MDI application's window menu. An MDI application can retrieve this handle
    /// from the menu of the MDI frame window by using the [`GetSubMenu`] function.
    pub window_menu: HANDLE,

    /// The child window identifier of the first MDI child window created. The system increments
    /// the identifier for each additional MDI child window the application creates, and reassigns
    /// identifiers when the application destroys a window to keep the range of identifiers
    /// contiguous. These identifiers are used in [`WM_COMMAND`] messages sent to the application's
    /// MDI frame window when a child window is chosen from the window menu; they should not
    /// conflict with any other command identifiers.
    pub id_first_child: UINT,
}

impl Default for CLIENTCREATESTRUCT {
    fn default() -> Self {
        CLIENTCREATESTRUCT {
            window_menu: null_mut(),
            id_first_child: 0,
        }
    }
}
