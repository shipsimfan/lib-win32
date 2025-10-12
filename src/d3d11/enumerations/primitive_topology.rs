/// How the pipeline interprets vertex data that is bound to the input-assembler stage. These
/// primitive topology values determine how the vertex data is rendered on screen.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_PRIMITIVE_TOPOLOGY {
    /// The IA stage has not been initialized with a primitive topology. The IA stage will not
    /// function properly unless a primitive topology is defined.
    Undefined,

    /// Interpret the vertex data as a list of points.
    PointList,

    /// Interpret the vertex data as a list of lines.
    LineList,

    /// Interpret the vertex data as a line strip.
    LineStrip,

    /// Interpret the vertex data as a list of triangles.
    TriangleList,

    /// Interpret the vertex data as a triangle strip.
    TriangleStrip,

    /// Interpret the vertex data as list of lines with adjacency data.
    LineListAdj,

    /// Interpret the vertex data as line strip with adjacency data.
    LineStripAdj,

    /// Interpret the vertex data as list of triangles with adjacency data.
    TriangleListAdj,

    /// Interpret the vertex data as triangle strip with adjacency data.
    TriangleStripAdj,

    /// Interpret the vertex data as a patch list.
    _1ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _2ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _3ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _4ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _5ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _6ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _7ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _8ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _9ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _10ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _11ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _12ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _13ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _14ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _15ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _16ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _17ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _18ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _19ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _20ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _21ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _22ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _23ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _24ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _25ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _26ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _27ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _28ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _29ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _30ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _31ControlPointPatchList,

    /// Interpret the vertex data as a patch list.
    _32ControlPointPatchList,
}
