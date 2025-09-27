use crate::{d3d11::D3D11_AUTHENTICATED_CONFIGURE_INPUT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_CONFIGURE_PROTECTION;

/// Contains input data for a [`D3D11_AUTHENTICATED_CONFIGURE_PROTECTION`] command.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    /// A [`D3D11_AUTHENTICATED_CONFIGURE_INPUT`] structure that contains the command GUID and
    /// other data.
    pub parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,

    /// A [`D3D11_AUTHENTICATED_PROTECTION_FLAGS`] union that specifies the protection level.
    pub protections: D3D11_AUTHENTICATED_PROTECTION_FLAGS,
}

impl Default for D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
            parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT::default(),
            protections: D3D11_AUTHENTICATED_PROTECTION_FLAGS::default(),
        }
    }
}

/// Specifies the protection level for video content.
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union D3D11_AUTHENTICATED_PROTECTION_FLAGS {
    /// Use this member to access all of the bits in the union.
    pub value: UINT,
}

impl std::fmt::Debug for D3D11_AUTHENTICATED_PROTECTION_FLAGS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { self.value }.fmt(f)
    }
}

impl Default for D3D11_AUTHENTICATED_PROTECTION_FLAGS {
    fn default() -> Self {
        D3D11_AUTHENTICATED_PROTECTION_FLAGS { value: 0 }
    }
}
