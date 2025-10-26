use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, ID3D11DeviceChildTrait},
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// A domain-shader interface manages an executable program (a domain shader) that controls the
    /// domain-shader stage.
    ///
    /// # Remarks
    /// The domain-shader interface has no methods; use HLSL to implement your shader
    /// functionality. All shaders are implemented from a common set of features referred to as the
    /// common-shader core.
    ///
    /// To create a domain-shader interface, call [`ID3D11Device::create_domain_shader`]. Before
    /// using a domain shader you must bind it to the device by calling
    /// [`ID3D11DeviceContext::ds_set_shader`].
    pub abstract ID3D11DomainShader(ID3D11DomainShaderVTable/ID3D11DomainShaderTrait):
        ID3D11DeviceChild/ID3D11DeviceChildTrait(device_child) +
        IUnknown/IUnknownTrait(device_child.unknown) {
        const IID = 0xF582C508-0x0F36-0x490C-0x9977-0x31EECE268CFA;
    }
);
