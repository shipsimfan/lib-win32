use crate::{dxgi::DXGI_MODE_ROTATION, BOOL, HMONITOR, RECT, WCHAR};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::IDXGIOutput;

/// Describes an output or physical connection between the adapter (video card) and a device.
///
/// The [`DXGI_OUTPUT_DESC`] structure is initialized by the [`IDXGIOutput::get_desc`] method.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_OUTPUT_DESC {
    /// A string that contains the name of the output device.
    pub device_name: [WCHAR; 32],

    /// A [`RECT`] structure containing the bounds of the output in desktop coordinates. Desktop
    /// coordinates depend on the dots per inch (DPI) of the desktop.
    pub desktop_coordinates: RECT,

    /// True if the output is attached to the desktop; otherwise, false.
    pub attached_to_desktop: BOOL,

    /// A member of the [`DXGI_MODE_ROTATION`] enumerated type describing on how an image is
    /// rotated by the output.
    pub rotation: DXGI_MODE_ROTATION,

    /// An [`HMONITOR`] handle that represents the display monitor.
    pub monitor: HMONITOR,
}

impl Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        DXGI_OUTPUT_DESC {
            device_name: [0; 32],
            desktop_coordinates: RECT::default(),
            attached_to_desktop: 0,
            rotation: DXGI_MODE_ROTATION::Unspecified,
            monitor: null_mut(),
        }
    }
}
