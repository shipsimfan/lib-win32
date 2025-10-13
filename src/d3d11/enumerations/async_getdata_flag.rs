// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::ID3D11DeviceContext, S_OK};

/// Optional flags that control the behavior of [`ID3D11DeviceContext::get_data`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_ASYNC_GETDATA_FLAG {
    /// Do not flush the command buffer. This can potentially cause an infinite loop if
    /// [`ID3D11DeviceContext::get_data`] is continually called until it returns [`S_OK`] as there
    /// may still be commands in the command buffer that need to be processed in order for
    /// [`ID3D11DeviceContext::get_data`] to return [`S_OK`]. Since the commands in the command
    /// buffer are not flushed they will not be processed and therefore
    /// [`ID3D11DeviceContext::get_data`] will never return [`S_OK`].
    DoNotFlush = 0x1,
}
