/// Flags that indicate how the back buffers should be rotated to fit the physical rotation of a
/// monitor.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DXGI_MODE_ROTATION {
    /// Unspecified rotation.
    Unspecified = 0,

    /// Specifies no rotation.
    Identity = 1,

    /// Specifies 90 degrees of rotation.
    Rotate90 = 2,

    /// Specifies 180 degrees of rotation.
    Rotate180 = 3,

    /// Specifies 270 degrees of rotation.
    Rotate270 = 4,
}
