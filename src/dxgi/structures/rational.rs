use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::DXGI_MODE_DESC;

/// Represents a rational number.
///
/// # Remarks
/// This structure is a member of the [`DXGI_MODE_DESC`] structure.
///
/// The [`DXGI_RATIONAL`] structure operates under the following rules:
///  - 0/0 is legal and will be interpreted as 0/1.
///  - 0/anything is interpreted as zero.
///  - If you are representing a whole number, the denominator should be 1.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_RATIONAL {
    /// An unsigned integer value representing the top of the rational number.
    pub numerator: UINT,

    /// An unsigned integer value representing the bottom of the rational number.
    pub denominator: UINT,
}

impl Default for DXGI_RATIONAL {
    fn default() -> Self {
        DXGI_RATIONAL {
            numerator: 0,
            denominator: 1,
        }
    }
}
