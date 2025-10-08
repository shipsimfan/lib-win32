use crate::d3d11::{D3D11_COMPARISON_FUNC, D3D11_STENCIL_OP};

/// Stencil operations that can be performed based on the results of stencil test.
///
/// # Remarks
/// All stencil operations are specified as a [`D3D11_STENCIL_OP`]. The stencil operation can be
/// set differently based on the outcome of the stencil test (which is referred to as
/// `stencil_func` in the stencil test portion of depth-stencil testing.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    /// The stencil operation to perform when stencil testing fails.
    pub stencil_fail_op: D3D11_STENCIL_OP,

    /// The stencil operation to perform when stencil testing passes and depth testing fails.
    pub stencil_depth_fail_op: D3D11_STENCIL_OP,

    /// The stencil operation to perform when stencil testing and depth testing both pass.
    pub stencil_pass_op: D3D11_STENCIL_OP,

    /// A function that compares stencil data against existing stencil data. The function options
    /// are listed in [`D3D11_COMPARISON_FUNC`].
    pub stencil_func: D3D11_COMPARISON_FUNC,
}

impl Default for D3D11_DEPTH_STENCILOP_DESC {
    fn default() -> Self {
        D3D11_DEPTH_STENCILOP_DESC {
            stencil_fail_op: D3D11_STENCIL_OP::Keep,
            stencil_depth_fail_op: D3D11_STENCIL_OP::Keep,
            stencil_pass_op: D3D11_STENCIL_OP::Keep,
            stencil_func: D3D11_COMPARISON_FUNC::Always,
        }
    }
}
