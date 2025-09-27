use crate::UINT64;

/// Contains an initialization vector (IV) for 128-bit Advanced Encryption Standard CTR mode
/// (AES-CTR) block cipher encryption.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AES_CTR_IV {
    /// The IV, in big-endian format.
    pub iv: UINT64,

    /// The block count, in big-endian format.
    pub count: UINT64,
}

impl Default for D3D11_AES_CTR_IV {
    fn default() -> Self {
        D3D11_AES_CTR_IV { iv: 0, count: 0 }
    }
}
