use crate::{
    d3dcommon::{D3D_MIN_PRECISION, D3D_NAME, D3D_REGISTER_COMPONENT_TYPE},
    BYTE, LPCSTR, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11shader::ID3D11ShaderReflection;

/// Describes a shader signature.
///
/// # Remarks
/// A shader can take n inputs and can produce m outputs. The order of the input (or output)
/// parameters, their associated types, and any attached semantics make up the shader signature.
/// Each shader has an input and an output signature.
///
/// When compiling a shader or an effect, some API calls validate shader signatures That is, they
/// compare the output signature of one shader (like a vertex shader) with the input signature of
/// another shader (like a pixel shader). This ensures that a shader outputs data that is
/// compatible with a downstream shader that is consuming that data. Compatible means that a shader
/// signature is a exact-match subset of the preceding shader stage. Exact match means parameter
/// types and semantics must exactly match. Subset means that a parameter that is not required by a
/// downstream stage, does not need to include that parameter in its shader signature.
///
/// Get a shader-signature from a shader or an effect by calling APIs such as
/// [`ID3D11ShaderReflection::get_input_parameter_desc`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_SIGNATURE_PARAMETER_DESC {
    /// A per-parameter string that identifies how the data will be used.
    pub semantic_name: LPCSTR,

    /// Semantic index that modifies the semantic. Used to differentiate different parameters that
    /// use the same semantic.
    pub semantic_index: UINT,

    /// The register that will contain this variable's data.
    pub register: UINT,

    /// A [`D3D_NAME`]-typed value that identifies a predefined string that determines the
    /// functionality of certain pipeline stages.
    pub system_value_type: D3D_NAME,

    /// A [`D3D_REGISTER_COMPONENT_TYPE`]-typed value that identifies the per-component-data type
    /// that is stored in a register. Each register can store up to four-components of data.
    pub component_type: D3D_REGISTER_COMPONENT_TYPE,

    /// Mask which indicates which components of a register are used.
    pub mask: BYTE,

    /// Mask which indicates whether a given component is never written (if the signature is an
    /// output signature) or always read (if the signature is an input signature).
    pub read_write_mask: BYTE,

    /// Indicates which stream the geometry shader is using for the signature parameter.
    pub stream: UINT,

    /// A [`D3D_MIN_PRECISION`]-typed value that indicates the minimum desired interpolation
    /// precision.
    pub min_precision: D3D_MIN_PRECISION,
}
