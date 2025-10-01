use crate::FLOAT;

/// Represents a 3x2 matrix. Used with [`IDXGISwapChain2::get_matrix_transform`] and
/// [`IDXGISwapChain2::set_matrix_transform`] to indicate the scaling and translation transform for
/// `SwapChainPanel` swap chains.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_MATRIX_3X2_F {
    /// The value in the first row and first column of the matrix.
    pub _11: FLOAT,

    /// The value in the first row and second column of the matrix.
    pub _12: FLOAT,

    /// The value in the second row and first column of the matrix.
    pub _21: FLOAT,

    /// The value in the second row and second column of the matrix.
    pub _22: FLOAT,

    /// The value in the third row and first column of the matrix.
    pub _31: FLOAT,

    /// The value in the third row and second column of the matrix.
    pub _32: FLOAT,
}

impl Default for DXGI_MATRIX_3X2_F {
    fn default() -> Self {
        DXGI_MATRIX_3X2_F {
            _11: 0.,
            _12: 0.,
            _21: 0.,
            _22: 0.,
            _31: 0.,
            _32: 0.,
        }
    }
}
