use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild},
    unknwn::{IUnknown},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Device;

com_interface!(
    /// A hull-shader interface manages an executable program (a hull shader) that controls the
    /// hull-shader stage.
    ///
    /// # Remarks
    /// The hull-shader interface has no methods; use HLSL to implement your shader functionality.
    /// All shaders are implemented from a common set of features referred to as the common-shader
    /// core.
    ///
    /// To create a hull-shader interface, call [`ID3D11Device::create_hull_shader`]. Before using
    /// a hull shader you must bind it to the device by calling
    /// [`ID3D11DeviceContext::hs_set_shader`].
    pub abstract ID3D11HullShader(ID3D11HullShaderVTable):
        ID3D11DeviceChild(device_child) +
        IUnknown {
        const IID = 0x8E5C6061-0x628A-0x4C8E-0x8264-0xBBE45CB3D5DD;
    }
);
