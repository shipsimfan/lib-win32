// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_2::{IDXGIAdapter2, DXGI_ADAPTER_DESC2};

/// Identifies the granularity at which the graphics processing unit (GPU) can be preempted from
/// performing its current graphics rendering task.
///
/// # Remarks
/// You call the [`IDXGIAdapter2::get_desc2`] method to retrieve the granularity level at which the
/// GPU can be preempted from performing its current graphics rendering task. The operating system
/// specifies the graphics granularity level in the `graphics_preemption_granularity` member of the
/// [`DXGI_ADAPTER_DESC2`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    /// Indicates the preemption granularity as a DMA buffer.
    DmaBufferBoundary = 0,

    /// Indicates the preemption granularity as a graphics primitive. A primitive is a section in a
    /// DMA buffer and can be a group of triangles.
    PrimitiveBoundary = 1,

    /// Indicates the preemption granularity as a triangle. A triangle is a part of a primitive.
    TriangleBoundary = 2,

    /// Indicates the preemption granularity as a pixel. A pixel is a part of a triangle.
    PixelBoundary = 3,

    /// Indicates the preemption granularity as a graphics instruction. A graphics instruction
    /// operates on a pixel.
    InstructionBoundary = 4,
}
