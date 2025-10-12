// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11shader::D3D11_SIGNATURE_PARAMETER_DESC;

/// Values that identify shader parameters that use system-value semantics.
///
/// # Remarks
/// The [`D3D_NAME`] values identify shader parameters that have predefined system-value semantics.
/// These values are used in a shader-signature description. For more information about
/// shader-signature description, see [`D3D11_SIGNATURE_PARAMETER_DESC`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D_NAME {
    /// This parameter does not use a predefined system-value semantic.
    Undefined = 0,

    /// This parameter contains position data.
    Position = 1,

    /// This parameter contains clip-distance data.
    ClipDistance = 2,

    /// This parameter contains cull-distance data.
    CullDistance = 3,

    /// This parameter contains a render-target-array index.
    RenderTargetArrayIndex = 4,

    /// This parameter contains a viewport-array index.
    ViewportArrayIndex = 5,

    /// This parameter contains a vertex ID.
    VertexId = 6,

    /// This parameter contains a primitive ID.
    PrimitiveId = 7,

    /// This parameter contains an instance ID.
    InstanceId = 8,

    /// This parameter contains data that identifies whether or not the primitive faces the camera.
    IsFrontFace = 9,

    /// This parameter contains a sampler-array index.
    SampleIndex = 10,

    /// This parameter contains one of four tessellation factors that correspond to the amount of
    /// parts that a quad patch is broken into along the given edge. This flag is used to
    /// tessellate a quad patch.
    FinalQuadEdgeTessfactor = 11,

    /// This parameter contains one of two tessellation factors that correspond to the amount of
    /// parts that a quad patch is broken into vertically and horizontally within the patch. This
    /// flag is used to tessellate a quad patch.
    FinalQuadInsideTessfactor = 12,

    /// This parameter contains one of three tessellation factors that correspond to the amount of
    /// parts that a tri patch is broken into along the given edge. This flag is used to tessellate
    /// a tri patch.
    FinalTriEdgeTessfactor = 13,

    /// This parameter contains the tessellation factor that corresponds to the amount of parts
    /// that a tri patch is broken into within the patch. This flag is used to tessellate a tri
    /// patch.
    FinalTriInsideTessfactor = 14,

    /// This parameter contains the tessellation factor that corresponds to the number of lines
    /// broken into within the patch. This flag is used to tessellate an isolines patch.
    FinalLineDetailTessfactor = 15,

    /// This parameter contains the tessellation factor that corresponds to the number of lines
    /// that are created within the patch. This flag is used to tessellate an isolines patch.
    FinalLineDensityTessfactor = 16,

    /// This parameter contains barycentric coordinate data.
    Barycentrics = 23,

    /// This parameter contains render-target data.
    Target = 64,

    /// This parameter contains depth data.
    Depth = 65,

    /// This parameter contains alpha-coverage data.
    Coverage = 66,

    /// This parameter signifies that the value is greater than or equal to a reference value. This
    /// flag is used to specify conservative depth for a pixel shader.
    DepthGreaterEqual = 67,

    /// This parameter signifies that the value is less than or equal to a reference value. This
    /// flag is used to specify conservative depth for a pixel shader.
    DepthLessEqual = 68,

    /// This parameter contains a stencil reference.
    StencilRef = 69,

    /// This parameter contains inner input coverage data.
    InnerCoverage = 70,

    /// This parameter does not use a predefined system-value semantic.
    D3D10Undefined,

    /// This parameter contains position data.
    D3D10Position,

    /// This parameter contains clip-distance data.
    D3D10ClipDistance,

    /// This parameter contains cull-distance data.
    D3D10CullDistance,

    /// This parameter contains a render-target-array index.
    D3D10RenderTargetArrayIndex,

    /// This parameter contains a viewport-array index.
    D3D10ViewportArrayIndex,

    /// This parameter contains a vertex ID.
    D3D10VertexId,

    /// This parameter contains a primitive ID.
    D3D10PrimitiveId,

    /// This parameter contains a instance ID.
    D3D10InstanceId,

    /// This parameter contains data that identifies whether or not the primitive faces the camera.
    D3D10IsFrontFace,

    /// This parameter contains a sampler-array index.
    D3D10SampleIndex,

    /// This parameter contains render-target data.
    D3D10Target,

    /// This parameter contains depth data.
    D3D10Depth,

    /// This parameter contains alpha-coverage data.
    D3D10Coverage,

    /// This parameter contains one of four tessellation factors that correspond to the amount of
    /// parts that a quad patch is broken into along the given edge. This flag is used to
    /// tessellate a quad patch.
    D3D11FinalQuadEdgeTessfactor,

    /// This parameter contains one of two tessellation factors that correspond to the amount of
    /// parts that a quad patch is broken into vertically and horizontally within the patch. This
    /// flag is used to tessellate a quad patch.
    D3D11FinalQuadInsideTessfactor,

    /// This parameter contains one of three tessellation factors that correspond to the amount of
    /// parts that a tri patch is broken into along the given edge. This flag is used to tessellate
    /// a tri patch.
    D3D11FinalTriEdgeTessfactor,

    /// This parameter contains the tessellation factor that corresponds to the amount of parts
    /// that a tri patch is broken into within the patch. This flag is used to tessellate a tri
    /// patch.
    D3D11FinalTriInsideTessfactor,

    /// This parameter contains the tessellation factor that corresponds to the amount of lines
    /// broken into within the patch. This flag is used to tessellate an isolines patch.
    D3D11FinalLineDetailTessfactor,

    /// This parameter contains the tessellation factor that corresponds to the amount of lines
    /// that are created within the patch. This flag is used to tessellate an isolines patch.
    D3D11FinalLineDensityTessfactor,

    /// This parameter signifies that the value is greater than or equal to a reference value. This
    /// flag is used to specify conservative depth for a pixel shader.
    D3D11DepthGreaterEqual,

    /// This parameter signifies that the value is less than or equal to a reference value. This
    /// flag is used to specify conservative depth for a pixel shader.
    D3D11DepthLessEqual,

    /// This parameter contains a stencil reference.
    D3D11StencilRef,

    /// This parameter contains inner input coverage data.
    D3D11InnerCoverage,

    /// This parameter contains barycentric coordinate data.
    D3D12Barycentrics,
}
