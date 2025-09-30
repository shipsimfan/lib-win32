use crate::{
    dxgi::{DXGI_MODE_DESC, DXGI_MODE_ROTATION},
    BOOL,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi::DXGI_FORMAT, dxgi1_2::IDXGIOutputDuplication, FALSE, TRUE};

/// The [`DXGI_OUTDUPL_DESC`] structure describes the dimension of the output and the surface that
/// contains the desktop image. The format of the desktop image is always
/// [`DXGI_FORMAT::B8G8R8A8UNorm`].
///
/// # Remarks
/// This structure is used by [`IDXGIOutputDuplication::get_desc`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_OUTDUPL_DESC {
    /// A [`DXGI_MODE_DESC`] structure that describes the display mode of the duplicated output.
    pub mode_desc: DXGI_MODE_DESC,

    /// A member of the [`DXGI_MODE_ROTATION`] enumerated type that describes how the duplicated
    /// output rotates an image.
    pub rotation: DXGI_MODE_ROTATION,

    /// Specifies whether the resource that contains the desktop image is already located in system
    /// memory. [`TRUE`] if the resource is in system memory; otherwise, [`FALSE`]. If this value
    /// is [`TRUE`] and the application requires CPU access, it can use the
    /// [`IDXGIOutputDuplication::map_desktop_surface`] and
    /// [`IDXGIOutputDuplication::unmap_desktop_surface`] methods to avoid copying the data into a
    /// staging buffer.
    pub desktop_image_in_system_memory: BOOL,
}

impl Default for DXGI_OUTDUPL_DESC {
    fn default() -> Self {
        DXGI_OUTDUPL_DESC {
            mode_desc: DXGI_MODE_DESC::default(),
            rotation: DXGI_MODE_ROTATION::Unspecified,
            desktop_image_in_system_memory: 0,
        }
    }
}
