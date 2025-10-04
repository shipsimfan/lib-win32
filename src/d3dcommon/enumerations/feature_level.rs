/// Describes the set of features targeted by a Direct3D device.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D_FEATURE_LEVEL {
    /// Allows Microsoft Compute Driver Model (MCDM) devices to be used, or more feature-rich
    /// devices (such as traditional GPUs) that support a superset of the functionality. MCDM is
    /// the overall driver model for compute-only; it's a scaled-down peer of the larger scoped
    /// Windows Device Driver Model (WDDM).
    _1_0_Core = 0x1000,

    /// Targets features supported by feature level 9.1, including shader model 2.
    _9_1 = 0x9100,

    /// Targets features supported by feature level 9.2, including shader model 2.
    _9_2 = 0x9200,

    /// Targets features supported by feature level 9.3, including shader model 2.0b.
    _9_3 = 0x9300,

    /// Targets features supported by Direct3D 10.0, including shader model 4.
    _10_0 = 0xA000,

    /// Targets features supported by Direct3D 10.1, including shader model 4.
    _10_1 = 0xA100,

    /// Targets features supported by Direct3D 11.0, including shader model 5.
    _11_0 = 0xB000,

    /// Targets features supported by Direct3D 11.1, including shader model 5 and logical blend
    /// operations. This feature level requires a display driver that is at least implemented to
    /// WDDM for Windows 8 (WDDM 1.2).
    _11_1 = 0xB100,

    /// Targets features supported by Direct3D 12.0, including shader model 5.
    _12_0 = 0xC000,

    /// Targets features supported by Direct3D 12.1, including shader model 5.
    _12_1 = 0xC100,

    /// Targets features supported by Direct3D 12.2, including shader model 6.5. Feature level 12_2
    /// is available in Windows SDK builds 20170 and later.
    _12_2 = 0xC200,
}
