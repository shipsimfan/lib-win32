use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, ID3D11DeviceChildTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// A vertex-shader interface manages an executable program (a vertex shader) that controls the
    /// vertex-shader stage.
    ///
    /// # Remarks
    /// The vertex-shader interface has no methods; use HLSL to implement your shader
    /// functionality. All shaders are implemented from a common set of features referred to as the
    /// common-shader core..
    ///
    /// To create a vertex shader interface, call [`ID3D11Device::create_vertex_shader`]. Before
    /// using a vertex shader you must bind it to the device by calling
    /// [`ID3D11DeviceContext::vs_set_shader`].
    pub abstract ID3D11VertexShader(ID3D11VertexShaderVTable/ID3D11VertexShaderTrait):
        ID3D11DeviceChild/ID3D11DeviceChildTrait(device_child) +
        IUnknown/IUnknownTrait(device_child.unknown) {
        const IID = 0x3B301D64-0xD678-0x4289-0x8897-0x22F8928B72F3;
    }
);
