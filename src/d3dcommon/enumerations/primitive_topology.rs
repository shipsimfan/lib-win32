// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11DeviceContext;

/// Values that indicate how the pipeline interprets vertex data that is bound to the
/// input-assembler stage. These primitive topology values determine how the vertex data is
/// rendered on screen.
///
/// # Remarks
/// Use the [`ID3D11DeviceContext::ia_set_primitive_topology`] method and a value from
/// [`D3D_PRIMITIVE_TOPOLOGY`] to bind a primitive topology to the input-assembler stage. Use the
/// [`ID3D11DeviceContext::ia_get_primitive_topology`] method to retrieve the primitive topology
/// for the input-assembler stage.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D_PRIMITIVE_TOPOLOGY {
    /// The IA stage has not been initialized with a primitive topology. The IA stage will not
    /// function properly unless a primitive topology is defined.
    Undefined = 0,

    /// Interpret the vertex data as a list of points.
    Pointlist = 1,

    /// Interpret the vertex data as a list of lines.
    Linelist = 2,

    /// Interpret the vertex data as a line strip.
    Linestrip = 3,

    /// Interpret the vertex data as a list of triangles.
    Trianglelist = 4,

    /// Interpret the vertex data as a triangle strip.
    Trianglestrip = 5,

    /// Interpret the vertex data as a list of lines with adjacency data.
    LinelistAdj = 10,

    /// Interpret the vertex data as a line strip with adjacency data.
    LinestripAdj = 11,

    /// Interpret the vertex data as a list of triangles with adjacency data.
    TrianglelistAdj = 12,

    /// Interpret the vertex data as a triangle strip with adjacency data.
    TrianglestripAdj = 13,

    /// Interpret the vertex data as a patch list.
    _1ControlPointPatchlist = 33,

    /// Interpret the vertex data as a patch list.
    _2ControlPointPatchlist = 34,

    /// Interpret the vertex data as a patch list.
    _3ControlPointPatchlist = 35,

    /// Interpret the vertex data as a patch list.
    _4ControlPointPatchlist = 36,

    /// Interpret the vertex data as a patch list.
    _5ControlPointPatchlist = 37,

    /// Interpret the vertex data as a patch list.
    _6ControlPointPatchlist = 38,

    /// Interpret the vertex data as a patch list.
    _7ControlPointPatchlist = 39,

    /// Interpret the vertex data as a patch list.
    _8ControlPointPatchlist = 40,

    /// Interpret the vertex data as a patch list.
    _9ControlPointPatchlist = 41,

    /// Interpret the vertex data as a patch list.
    _10ControlPointPatchlist = 42,

    /// Interpret the vertex data as a patch list.
    _11ControlPointPatchlist = 43,

    /// Interpret the vertex data as a patch list.
    _12ControlPointPatchlist = 44,

    /// Interpret the vertex data as a patch list.
    _13ControlPointPatchlist = 45,

    /// Interpret the vertex data as a patch list.
    _14ControlPointPatchlist = 46,

    /// Interpret the vertex data as a patch list.
    _15ControlPointPatchlist = 47,

    /// Interpret the vertex data as a patch list.
    _16ControlPointPatchlist = 48,

    /// Interpret the vertex data as a patch list.
    _17ControlPointPatchlist = 49,

    /// Interpret the vertex data as a patch list.
    _18ControlPointPatchlist = 50,

    /// Interpret the vertex data as a patch list.
    _19ControlPointPatchlist = 51,

    /// Interpret the vertex data as a patch list.
    _20ControlPointPatchlist = 52,

    /// Interpret the vertex data as a patch list.
    _21ControlPointPatchlist = 53,

    /// Interpret the vertex data as a patch list.
    _22ControlPointPatchlist = 54,

    /// Interpret the vertex data as a patch list.
    _23ControlPointPatchlist = 55,

    /// Interpret the vertex data as a patch list.
    _24ControlPointPatchlist = 56,

    /// Interpret the vertex data as a patch list.
    _25ControlPointPatchlist = 57,

    /// Interpret the vertex data as a patch list.
    _26ControlPointPatchlist = 58,

    /// Interpret the vertex data as a patch list.
    _27ControlPointPatchlist = 59,

    /// Interpret the vertex data as a patch list.
    _28ControlPointPatchlist = 60,

    /// Interpret the vertex data as a patch list.
    _29ControlPointPatchlist = 61,

    /// Interpret the vertex data as a patch list.
    _30ControlPointPatchlist = 62,

    /// Interpret the vertex data as a patch list.
    _31ControlPointPatchlist = 63,

    /// Interpret the vertex data as a patch list.
    _32ControlPointPatchlist = 64,

    /// The IA stage has not been initialized with a primitive topology. The IA stage will not
    /// function properly unless a primitive topology is defined.
    D3D10Undefined,

    /// Interpret the vertex data as a list of points.
    D3D10Pointlist,

    /// Interpret the vertex data as a list of lines.
    D3D10Linelist,

    /// Interpret the vertex data as a line strip.
    D3D10Linestrip,

    /// Interpret the vertex data as a list of triangles.
    D3D10Trianglelist,

    /// Interpret the vertex data as a triangle strip.
    D3D10Trianglestrip,

    /// Interpret the vertex data as a list of lines with adjacency data.
    D3D10LinelistAdj,

    /// Interpret the vertex data as a line strip with adjacency data.
    D3D10LinestripAdj,

    /// Interpret the vertex data as a list of triangles with adjacency data.
    D3D10TrianglelistAdj,

    /// Interpret the vertex data as a triangle strip with adjacency data.
    D3D10TrianglestripAdj,

    /// The IA stage has not been initialized with a primitive topology. The IA stage will not
    /// function properly unless a primitive topology is defined.
    D3D11Undefined,

    /// Interpret the vertex data as a list of points.
    D3D11Pointlist,

    /// Interpret the vertex data as a list of lines.
    D3D11Linelist,

    /// Interpret the vertex data as a line strip.
    D3D11Linestrip,

    /// Interpret the vertex data as a list of triangles.
    D3D11Trianglelist,

    /// Interpret the vertex data as a triangle strip.
    D3D11Trianglestrip,

    /// Interpret the vertex data as a list of lines with adjacency data.
    D3D11LinelistAdj,

    /// Interpret the vertex data as a line strip with adjacency data.
    D3D11LinestripAdj,

    /// Interpret the vertex data as a list of triangles with adjacency data.
    D3D11TrianglelistAdj,

    /// Interpret the vertex data as a triangle strip with adjacency data.
    D3D11TrianglestripAdj,

    /// Interpret the vertex data as a patch list.
    D3D11_1ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_2ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_3ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_4ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_5ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_6ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_7ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_8ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_9ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_10ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_11ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_12ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_13ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_14ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_15ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_16ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_17ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_18ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_19ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_20ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_21ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_22ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_23ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_24ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_25ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_26ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_27ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_28ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_29ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_30ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_31ControlPointPatchlist,

    /// Interpret the vertex data as a patch list.
    D3D11_32ControlPointPatchlist,
}
