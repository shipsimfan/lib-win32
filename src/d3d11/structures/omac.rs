use crate::BYTE;

/// Contains a Message Authentication Code (MAC).
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_OMAC {
    /// A byte array that contains the cryptographic MAC value of the message.
    pub omac: [BYTE; 16],
}

impl Default for D3D11_OMAC {
    fn default() -> Self {
        D3D11_OMAC { omac: [0; 16] }
    }
}
