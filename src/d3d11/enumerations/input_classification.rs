// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_INPUT_ELEMENT_DESC;

/// Type of data contained in an input slot.
///
/// # Remarks
/// Use these values to specify the type of data for a particular input element (see
/// [`D3D11_INPUT_ELEMENT_DESC`]) of an input-layout object.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_INPUT_CLASSIFICATION {
    /// Input data is per-vertex data.
    PerVertexData = 0,

    /// Input data is per-instance data.
    PerInstanceData = 1,
}
