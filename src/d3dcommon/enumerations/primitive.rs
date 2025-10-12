// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11shader::ID3D11ShaderReflection, d3dcommon::D3D11_PRIMITIVE};

/// Indicates how the pipeline interprets geometry or hull shader input primitives.
///
/// # Remarks
/// The [`ID3D11ShaderReflection::get_gs_input_primitive`] method returns a
/// [`D3D11_PRIMITIVE`]-typed value.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D_PRIMITIVE {
    #[allow(missing_docs)]
    Undefined = 0,

    #[allow(missing_docs)]
    Point = 1,

    #[allow(missing_docs)]
    Line = 2,

    #[allow(missing_docs)]
    Triangle = 3,

    #[allow(missing_docs)]
    LineAdj = 6,

    #[allow(missing_docs)]
    TriangleAdj = 7,

    #[allow(missing_docs)]
    _1ControlPointPatch = 8,

    #[allow(missing_docs)]
    _2ControlPointPatch = 9,

    #[allow(missing_docs)]
    _3ControlPointPatch = 10,

    #[allow(missing_docs)]
    _4ControlPointPatch = 11,

    #[allow(missing_docs)]
    _5ControlPointPatch = 12,

    #[allow(missing_docs)]
    _6ControlPointPatch = 13,

    #[allow(missing_docs)]
    _7ControlPointPatch = 14,

    #[allow(missing_docs)]
    _8ControlPointPatch = 15,

    #[allow(missing_docs)]
    _9ControlPointPatch = 16,

    #[allow(missing_docs)]
    _10ControlPointPatch = 17,

    #[allow(missing_docs)]
    _11ControlPointPatch = 18,

    #[allow(missing_docs)]
    _12ControlPointPatch = 19,

    #[allow(missing_docs)]
    _13ControlPointPatch = 20,

    #[allow(missing_docs)]
    _14ControlPointPatch = 21,

    #[allow(missing_docs)]
    _15ControlPointPatch = 22,

    #[allow(missing_docs)]
    _16ControlPointPatch = 23,

    #[allow(missing_docs)]
    _17ControlPointPatch = 24,

    #[allow(missing_docs)]
    _18ControlPointPatch = 25,

    #[allow(missing_docs)]
    _19ControlPointPatch = 26,

    #[allow(missing_docs)]
    _20ControlPointPatch = 27,

    #[allow(missing_docs)]
    _21ControlPointPatch = 28,

    #[allow(missing_docs)]
    _22ControlPointPatch = 29,

    #[allow(missing_docs)]
    _23ControlPointPatch = 30,

    #[allow(missing_docs)]
    _24ControlPointPatch = 31,

    #[allow(missing_docs)]
    _25ControlPointPatch = 32,

    #[allow(missing_docs)]
    _26ControlPointPatch = 33,

    #[allow(missing_docs)]
    _27ControlPointPatch = 34,

    #[allow(missing_docs)]
    _28ControlPointPatch = 35,

    #[allow(missing_docs)]
    _29ControlPointPatch = 36,

    #[allow(missing_docs)]
    _30ControlPointPatch = 37,

    #[allow(missing_docs)]
    _31ControlPointPatch = 38,

    #[allow(missing_docs)]
    _32ControlPointPatch = 39,

    #[allow(missing_docs)]
    D3D10Undefined,

    #[allow(missing_docs)]
    D3D10Point,

    #[allow(missing_docs)]
    D3D10Line,

    #[allow(missing_docs)]
    D3D10Triangle,

    #[allow(missing_docs)]
    D3D10LineAdj,

    #[allow(missing_docs)]
    D3D10TriangleAdj,

    /// The shader has not been initialized with an input primitive type.
    D3D11Undefined,

    /// Interpret the input primitive as a point.
    D3D11Point,

    /// Interpret the input primitive as a line.
    D3D11Line,

    /// Interpret the input primitive as a triangle.
    D3D11Triangle,

    /// Interpret the input primitive as a line with adjacency data.
    D3D11LineAdj,

    /// Interpret the input primitive as a triangle with adjacency data.
    D3D11TriangleAdj,

    /// Interpret the input primitive as a control point patch.
    D3D11_1ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_2ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_3ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_4ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_5ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_6ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_7ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_8ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_9ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_10ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_11ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_12ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_13ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_14ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_15ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_16ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_17ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_18ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_19ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_20ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_21ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_22ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_23ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_24ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_25ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_26ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_27ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_28ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_29ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_30ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_31ControlPointPatch,

    /// Interpret the input primitive as a control point patch.
    D3D11_32ControlPointPatch,
}
