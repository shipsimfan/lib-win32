/// Specifies if the hardware and driver support conservative rasterization and at what tier level.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_CONSERVATIVE_RASTERIZATION_TIER {
    /// Conservative rasterization isn't supported.
    NotSupported = 0,

    /// Tier 1 conservative rasterization is supported.
    _1 = 1,

    /// Tier 2 conservative rasterization is supported.
    _2 = 2,

    /// Tier 3 conservative rasterization is supported.
    _3 = 3,
}
