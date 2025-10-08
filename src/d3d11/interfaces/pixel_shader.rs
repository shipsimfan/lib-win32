use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, ID3D11DeviceChildTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// A pixel-shader interface manages an executable program (a pixel shader) that controls the
    /// pixel-shader stage.
    ///
    /// # Remarks
    /// The pixel-shader interface has no methods; use HLSL to implement your shader functionality.
    /// All shaders in are implemented from a common set of features referred to as the
    /// common-shader core.
    ///
    /// To create a pixel shader interface, call [`ID3D11Device::create_pixel_shader`]. Before
    /// using a pixel shader you must bind it to the device by calling
    /// [`ID3D11DeviceContext::ps_set_shader`].
    pub abstract ID3D11PixelShader(ID3D11PixelShaderVTable/ID3D11PixelShaderTrait):
        ID3D11DeviceChild/ID3D11DeviceChildTrait(device_child) +
        IUnknown/IUnknownTrait(device_child.unknown) {
        const IID = 0xEA82E40D-0x51DC-0x4F33-0x93D4-0xDB7C9125AE8C;
    }
);
