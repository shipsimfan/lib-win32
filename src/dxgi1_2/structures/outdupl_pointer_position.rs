use crate::{BOOL, POINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{FALSE, TRUE};

/// The [`DXGI_OUTDUPL_POINTER_POSITION`] structure describes the position of the hardware cursor.
///
/// # Remarks
/// The Position member is valid only if the Visible member’s value is set to [`TRUE`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    /// The position of the hardware cursor relative to the top-left of the adapter output.
    pub position: POINT,

    /// Specifies whether the hardware cursor is visible. [`TRUE`] if visible; otherwise,
    /// [`FALSE`]. If the hardware cursor is not visible, the calling application does not display
    /// the cursor in the client.
    pub visible: BOOL,
}

impl Default for DXGI_OUTDUPL_POINTER_POSITION {
    fn default() -> Self {
        DXGI_OUTDUPL_POINTER_POSITION {
            position: POINT::default(),
            visible: 0,
        }
    }
}
