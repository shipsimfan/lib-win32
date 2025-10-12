use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, ID3D11DeviceChildTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// A geometry-shader interface manages an executable program (a geometry shader) that controls
    /// the geometry-shader stage.
    ///
    /// # Remarks
    /// The geometry-shader interface has no methods; use HLSL to implement your shader
    /// functionality. All shaders are implemented from a common set of features referred to as the
    /// common-shader core..
    ///
    /// To create a geometry shader interface, call either [`ID3D11Device::create_geometry_shader`]
    /// or [`ID3D11Device::create_geometry_shader_with_stream_output`]. Before using a geometry
    /// shader you must bind it to the device by calling [`ID3D11DeviceContext::gs_set_shader`].
    pub abstract ID3D11GeometryShader(ID3D11GeometryShaderVTable/ID3D11GeometryShaderTrait):
        ID3D11DeviceChild/ID3D11DeviceChildTrait(device_child) +
        IUnknown/IUnknownTrait(device_child.unknown) {
        const IID = 0x38325B96-0xEFFB-0x4022-0xBA02-0x2E795B70275C;
    }
);
