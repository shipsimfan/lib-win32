use crate::{BOOL, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11ClassInstance, ID3D11ClassLinkage};

/// Describes an HLSL class instance.
///
/// # Remarks
/// The [`D3D11_CLASS_INSTANCE_DESC`] structure is returned by the
/// [`ID3D11ClassInstance::get_desc`] method.
///
/// The members of this structure except `instance_index` are valid (non default values) if they
/// describe a class instance acquired using [`ID3D11ClassLinkage::create_class_instance`]. The
/// `instance_index` member is only valid when the class instance is aquired using
/// [`ID3D11ClassLinkage::get_class_instance`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_CLASS_INSTANCE_DESC {
    /// The instance ID of an HLSL class; the default value is 0.
    pub instance_id: UINT,

    /// The instance index of an HLSL class; the default value is 0.
    pub instance_index: UINT,

    /// The type ID of an HLSL class; the default value is 0.
    pub type_id: UINT,

    /// Describes the constant buffer associated with an HLSL class; the default value is 0.
    pub constant_buffer: UINT,

    /// The base constant buffer offset associated with an HLSL class; the default value is 0.
    pub base_constant_buffer_offset: UINT,

    /// The base texture associated with an HLSL class; the default value is 127.
    pub base_texture: UINT,

    /// The base sampler associated with an HLSL class; the default value is 15.
    pub base_sampler: UINT,

    /// True if the class was created; the default value is false.
    pub created: BOOL,
}

impl Default for D3D11_CLASS_INSTANCE_DESC {
    fn default() -> Self {
        D3D11_CLASS_INSTANCE_DESC {
            instance_id: 0,
            instance_index: 0,
            type_id: 0,
            constant_buffer: 0,
            base_constant_buffer_offset: 0,
            base_texture: 0,
            base_sampler: 0,
            created: 0,
        }
    }
}
