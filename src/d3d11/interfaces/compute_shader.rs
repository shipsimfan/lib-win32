use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild},
    unknwn::{IUnknown},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// A compute-shader interface manages an executable program (a compute shader) that controls
    /// the compute-shader stage.
    ///
    /// # Remarks
    /// The compute-shader interface has no methods; use HLSL to implement your shader
    /// functionality. All shaders are implemented from a common set of features referred to as the
    /// common-shader core.
    ///
    /// To create a compute-shader interface, call [`ID3D11Device::create_compute_shader`]. Before
    /// using a compute shader you must bind it to the device by calling
    /// [`ID3D11DeviceContext::cs_set_shader`].
    pub abstract ID3D11ComputeShader(ID3D11ComputeShaderVTable):
        ID3D11DeviceChild(device_child) +
        IUnknown {
        const IID = 0x4F5B196E-0xC2BD-0x495E-0xBD01-0x1FDED38E4969;
    }
);
