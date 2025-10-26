use crate::{BYTE, LPCSTR, UINT};
use std::ptr::null;

/// Description of a vertex element in a vertex buffer in an output slot.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_SO_DECLARATION_ENTRY {
    /// Zero-based, stream number.
    pub stream: UINT,

    /// Type of output element; possible values include: "POSITION", "NORMAL", or "TEXCOORD0". Note
    /// that if `semantic_name` is [`null`] then `component_count` can be greater than 4 and the
    /// described entry will be a gap in the stream out where no data will be written.
    pub semantic_name: LPCSTR,

    /// Output element's zero-based index. Should be used if, for example, you have more than one
    /// texture coordinate stored in each vertex.
    pub semantic_index: UINT,

    /// Which component of the entry to begin writing out to. Valid values are 0 to 3. For example,
    /// if you only wish to output to the y and z components of a position, then `start_component`
    /// should be 1 and `component_count` should be 2.
    pub start_component: BYTE,

    /// The number of components of the entry to write out to. Valid values are 1 to 4. For
    /// example, if you only wish to output to the y and z components of a position, then
    /// `start_component` should be 1 and `component_count` should be 2. Note that if
    /// `semantic_name` is [`null`] then `component_count` can be greater than 4 and the described
    /// entry will be a gap in the stream out where no data will be written.
    pub component_count: BYTE,

    /// The associated stream output buffer that is bound to the pipeline (see
    /// [`ID3D11DeviceContext::so_set_targets`]). The valid range for `output_slot` is 0 to 3.
    pub output_slot: BYTE,
}

impl Default for D3D11_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        D3D11_SO_DECLARATION_ENTRY {
            stream: 0,
            semantic_name: null(),
            semantic_index: 0,
            start_component: 0,
            component_count: 0,
            output_slot: 0,
        }
    }
}
