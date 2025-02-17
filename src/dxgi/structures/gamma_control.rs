use crate::dxgi::DXGI_RGB;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::IDXGIOutput;

/// Controls the settings of a gamma curve.
///
/// The [`DXGI_GAMMA_CONTROL`] structure is used by the [`IDXGIOutput::set_gamma_control`] method.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_GAMMA_CONTROL {
    /// A [`DXGI_RGB`] structure with scalar values that are applied to rgb values before being
    /// sent to the gamma look up table.
    pub scale: DXGI_RGB,

    /// A [`DXGI_RGB`] structure with offset values that are applied to the rgb values before being
    /// sent to the gamma look up table.
    pub offset: DXGI_RGB,

    /// An array of [`DXGI_RGB`] structures that control the points of a gamma curve.
    pub gamma_curve: [DXGI_RGB; 1025],
}

impl Default for DXGI_GAMMA_CONTROL {
    fn default() -> Self {
        DXGI_GAMMA_CONTROL {
            scale: DXGI_RGB::default(),
            offset: DXGI_RGB::default(),
            gamma_curve: [DXGI_RGB::default(); 1025],
        }
    }
}
