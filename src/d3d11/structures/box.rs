use crate::UINT;

/// Defines a 3D box.
///
/// # Remarks
/// Coordinates of a box are in bytes for buffers and in texels for textures.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_BOX {
    /// The x position of the left hand side of the box.
    pub left: UINT,

    /// The y position of the top of the box.
    pub top: UINT,

    /// The z position of the front of the box.
    pub front: UINT,

    /// The x position of the right hand side of the box.
    pub right: UINT,

    /// The y position of the bottom of the box.
    pub bottom: UINT,

    /// The z position of the back of the box.
    pub back: UINT,
}

impl Default for D3D11_BOX {
    fn default() -> Self {
        D3D11_BOX {
            left: 0,
            top: 0,
            front: 0,
            right: 0,
            bottom: 0,
            back: 0,
        }
    }
}
