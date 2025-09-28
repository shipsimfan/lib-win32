// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::DXGI_ADAPTER_DESC2;

/// Identifies the granularity at which the graphics processing unit (GPU) can be preempted from
/// performing its current compute task.
///
/// # Remarks
/// You call the [`IDXGIAdapter2::get_desc2`] method to retrieve the granularity level at which the
/// GPU can be preempted from performing its current compute task. The operating system specifies
/// the compute granularity level in the `compute_preemption_granularity` member of the
/// [`DXGI_ADAPTER_DESC2`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    /// Indicates the preemption granularity as a compute packet.
    DmaBufferBoundary = 0,

    /// Indicates the preemption granularity as a dispatch (for example, a call to the
    /// [`ID3D11DeviceContext::dispatch`] method). A dispatch is a part of a compute packet.
    DispatchBoundary = 1,

    /// Indicates the preemption granularity as a thread group. A thread group is a part of a
    /// dispatch.
    ThreadGroupBoundary = 2,

    /// Indicates the preemption granularity as a thread in a thread group. A thread is a part of a
    /// thread group.
    ThreadBoundary = 3,

    /// Indicates the preemption granularity as a compute instruction in a thread.
    InstructionBoundary = 4,
}
