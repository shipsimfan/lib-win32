use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{FALSE, TRUE};

/// Describes double data type support in the current graphics driver.
///
/// # Remarks
/// If the runtime sets `double_precision_float_shader_ops` to [`TRUE`], the hardware and driver
/// support the following Shader Model 5 instructions:
///  - `dadd`
///  - `dmax`
///  - `dmin`
///  - `dmul`
///  - `deq`
///  - `dge`
///  - `dlt`
///  - `dne`
///  - `dmov`
///  - `dmovc`
///  - `dtof`
///  - `ftod`
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_DOUBLES {
    /// Specifies whether double types are allowed. If [`TRUE`], double types are allowed;
    /// otherwise [`FALSE`]. The runtime must set `double_precision_float_shader_ops` to [`TRUE`]
    /// in order for you to use any HLSL shader that is compiled with a double type.
    pub double_precision_float_shader_ops: BOOL,
}

impl Default for D3D11_FEATURE_DATA_DOUBLES {
    fn default() -> Self {
        D3D11_FEATURE_DATA_DOUBLES {
            double_precision_float_shader_ops: 0,
        }
    }
}
