use crate::{
    d3d11::{ID3D11Device, ID3D11DeviceContext},
    d3dcommon::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL},
    dxgi::{IDXGIAdapter, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC},
    HMODULE, HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11_1")]
use crate::d3d11_1::ID3D11Device1;
#[allow(unused_imports)]
#[cfg(feature = "d3d11_2")]
use crate::d3d11_2::ID3D11Device2;
#[allow(unused_imports)]
use crate::{
    d3d11::{D3D11CreateDevice, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION},
    dxgi::IDXGIFactory,
    unknwn::IUnknown,
    GetModuleHandle, LoadLibrary, LoadLibraryEx, DXGI_ERROR_NOT_CURRENTLY_AVAILABLE,
    DXGI_ERROR_SDK_COMPONENT_MISSING, E_INVALIDARG,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "D3D11")]
unsafe extern "system" {
    /// Creates a device that represents the display adapter and a swap chain used for rendering.
    ///
    /// # Parameters
    ///  * `adapter` - A pointer to the video adapter to use when creating a device. Pass
    ///                [`null_mut`] to use the default adapter, which is the first adapter
    ///                enumerated by [`IDXGIFactory::enum_adapters`].
    ///  * `driver_type` - The [`D3D_DRIVER_TYPE`], which represents the driver type to create.
    ///  * `software` - A handle to a DLL that implements a software rasterizer. If `driver_type`
    ///                 is [`D3D_DRIVER_TYPE::Software`], `software` must not be [`null_mut`]. Get
    ///                 the handle by calling [`LoadLibrary`], [`LoadLibraryEx`], or
    ///                 [`GetModuleHandle`]. The value should be non-[`null_mut`] when
    ///                 [`D3D_DRIVER_TYPE`] is [`D3D_DRIVER_TYPE::Software`] and [`null_mut`]
    ///                 otherwise.
    ///  * `flags` - The runtime layers to enable (see [`D3D11_CREATE_DEVICE_FLAG`]); values can be
    ///              bitwise OR'd together.
    ///  * `feature_levels` - A pointer to an array of [`D3D_FEATURE_LEVEL`]s, which determine the
    ///                       order of feature levels to attempt to create.
    ///  * `num_feature_levels` - The number of elements in `feature_levels`.
    ///  * `sdk_version` - The SDK version; use [`D3D11_SDK_VERSION`].
    ///  * `swap_chain_desc` - A pointer to a swap chain description (see [`DXGI_SWAP_CHAIN_DESC`])
    ///                        that contains initialization parameters for the swap chain.
    ///  * `swap_chain` - Returns the address of a pointer to the [`IDXGISwapChain`] object that
    ///                   represents the swap chain used for rendering.
    ///  * `device` - Returns the address of a pointer to an [`ID3D11Device`] object that
    ///               represents the device created. If this parameter is [`null_mut`], no
    ///               [`ID3D11Device`] will be returned'.
    ///  * `feature_level` - Returns a pointer to a [`D3D_FEATURE_LEVEL`], which represents the
    ///                      first element in an array of feature levels supported by the device.
    ///                      Supply [`null_mut`] as an input if you don't need to determine which
    ///                      feature level is supported.
    ///  * `immediate_context` - Returns the address of a pointer to an [`ID3D11DeviceContext`]
    ///                          object that represents the device context. If this parameter is
    ///                          [`null_mut`], no [`ID3D11DeviceContext`] will be returned.
    ///
    /// # Return Value
    /// This method can return one of the Direct3D 11 Return Codes.
    ///
    /// This method returns [`DXGI_ERROR_NOT_CURRENTLY_AVAILABLE`] if you call it in a Session 0
    /// process.
    ///
    /// This method returns [`E_INVALIDARG`] if you set the `adapter` parameter to a
    /// non-[`null_mut`] value and the `driver_type` parameter to the [`D3D_DRIVER_TYPE::Hardware`]
    /// value.
    ///
    /// This method returns [`DXGI_ERROR_SDK_COMPONENT_MISSING`] if you specify
    /// [`D3D11_CREATE_DEVICE_FLAG::Debug`] in `flags` and the incorrect version of the debug layer
    /// is installed on your computer. Install the latest Windows SDK to get the correct version.
    ///
    /// # Remarks
    /// This entry-point is supported by the Direct3D 11 runtime, which is available on Windows 7,
    /// Windows Server 2008 R2, and as an update to Windows Vista (KB971644).
    ///
    /// To create a Direct3D 11.1 device ([`ID3D11Device1`]), which is available on Windows 8,
    /// Windows Server 2012, and Windows 7 and Windows Server 2008 R2 with the Platform Update for
    /// Windows 7 installed, you first create a [`ID3D11Device`] with this function, and then call
    /// the [`IUnknown::query_interface`] method on the [`ID3D11Device`] object to obtain the
    /// [`ID3D11Device1`] interface.
    ///
    /// To create a Direct3D 11.2 device ([`ID3D11Device2`]), which is available on Windows 8.1 and
    /// Windows Server 2012 R2, you first create a [`ID3D11Device`] with this function, and then
    /// call the [`IUnknown::query_interface`] method on the [`ID3D11Device`] object to obtain the
    /// [`ID3D11Device2`] interface.
    ///
    /// Also, see the remarks section in [`D3D11CreateDevice`] for details about input parameter
    /// dependencies. To create a device without creating a swap chain, use the
    /// [`D3D11CreateDevice`] function.
    ///
    /// If you set the `adapter` parameter to a non-[`null_mut`] value, you must also set the
    /// `driver_type` parameter to the [`D3D_DRIVER_TYPE::Unknown`] value. If you set the `adapter`
    /// parameter to a non-[`null_mut`] value and the `driver_type` parameter to the
    /// [`D3D_DRIVER_TYPE::Hardware`] value, [`D3D11CreateDeviceAndSwapChain`] returns an
    /// [`HRESULT`] of [`E_INVALIDARG`].
    pub fn D3D11CreateDeviceAndSwapChain(
        adapter: *mut IDXGIAdapter,
        driver_type: D3D_DRIVER_TYPE,
        software: HMODULE,
        flags: UINT,
        feature_levels: *const D3D_FEATURE_LEVEL,
        num_feature_levels: UINT,
        sdk_version: UINT,
        swap_chain_desc: *const DXGI_SWAP_CHAIN_DESC,
        swap_chain: *mut *mut IDXGISwapChain,
        device: *mut *mut ID3D11Device,
        feature_level: *mut D3D_FEATURE_LEVEL,
        immediate_context: *mut *mut ID3D11DeviceContext,
    ) -> HRESULT;
}
