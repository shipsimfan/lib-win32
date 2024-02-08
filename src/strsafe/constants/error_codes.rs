use crate::HRESULT;

/// No error occurred
pub const S_OK: HRESULT = 0;

/// The buffer is not large enough
pub const STRSAFE_E_INSUFFICIENT_BUFFER: HRESULT = 0x8007007Au32 as HRESULT;

/// A parameter is invalid
pub const STRSAFE_E_INVALID_PARAMETER: HRESULT = 0x80070057u32 as HRESULT;

/// The end of the file has been reached
pub const STRSAFE_E_END_OF_FILE: HRESULT = 0x80070026u32 as HRESULT;
