use crate::{d3d11::D3D11_INPUT_CLASSIFICATION, dxgi::DXGI_FORMAT, LPCSTR, UINT};
use std::ptr::null;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, D3D11_APPEND_ALIGNED_ELEMENT};

/// A description of a single element for the input-assembler stage.
///
/// # Remarks
/// An input-layout object contains an array of structures, each structure defines one element
/// being read from an input slot. Create an input-layout object by calling
/// [`ID3D11Device::create_input_layout`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    /// The HLSL semantic associated with this element in a shader input-signature.
    pub semantic_name: LPCSTR,

    /// The semantic index for the element. A semantic index modifies a semantic, with an integer
    /// index number. A semantic index is only needed in a case where there is more than one
    /// element with the same semantic.
    pub semantic_index: UINT,

    /// The data type of the element data. See [`DXGI_FORMAT`].
    pub format: DXGI_FORMAT,

    /// An integer value that identifies the input-assembler (see input slot). Valid values are
    /// between 0 and 15.
    pub input_slot: UINT,

    /// Optional. Offset (in bytes) from the start of the vertex. Use
    /// [`D3D11_APPEND_ALIGNED_ELEMENT`] for convenience to define the current element directly
    /// after the previous one, including any packing if necessary.
    pub aligned_byte_offset: UINT,

    /// Identifies the input data class for a single input slot (see
    /// [`D3D11_INPUT_CLASSIFICATION`]).
    pub input_slot_class: D3D11_INPUT_CLASSIFICATION,

    /// The number of instances to draw using the same per-instance data before advancing in the
    /// buffer by one element. This value must be 0 for an element that contains per-vertex data
    /// (the slot class is set to `D3D11_INPUT_PER_VERTEX_DATA`).
    pub instance_data_step_rate: UINT,
}

impl Default for D3D11_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        D3D11_INPUT_ELEMENT_DESC {
            semantic_name: null(),
            semantic_index: 0,
            format: DXGI_FORMAT::Unknown,
            input_slot: 0,
            aligned_byte_offset: 0,
            input_slot_class: D3D11_INPUT_CLASSIFICATION::PerVertexData,
            instance_data_step_rate: 0,
        }
    }
}
