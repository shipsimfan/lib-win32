/// Specifies the type of Microsoft Direct3D authenticated channel.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_AUTHENTICATED_CHANNEL_TYPE {
    /// Direct3D 11 channel. This channel provides communication with the Direct3D runtime.
    D3D11 = 1,

    /// Software driver channel. This channel provides communication with a driver that implements
    /// content protection mechanisms in software.
    Software = 2,

    /// Hardware driver channel. This channel provides communication with a driver that implements
    /// content protection mechanisms in the GPU hardware.
    Hardware = 3,
}
