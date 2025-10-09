/// Values that specify minimum precision levels at shader stages.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_SHADER_MIN_PRECISION_SUPPORT {
    /// Minimum precision level is 10-bit.
    _10Bit = 0x1,

    /// Minimum precision level is 16-bit.
    _16Bit = 0x2,
}
