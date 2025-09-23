// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};

/// Options for handling pixels in a display surface after calling [`IDXGISwapChain1::present1`].
///
/// # Remarks
/// This enumeration is used by the [`DXGI_SWAP_CHAIN_DESC`] and [`DXGI_SWAP_CHAIN_DESC1`]
/// structures.
///
/// In D3D12, only [`DXGI_SWAP_EFFECT::FlipSequential`] and [`DXGI_SWAP_EFFECT::FlipDiscard`] are
/// supported, and the bitblt models are not. Because of this, multisampling a back buffer is not
/// supported in D3D12, and you must manually perform multisampling in the app using
/// [`ID3D12GraphicsCommandList::resolve_subresource`] or
/// [`ID3D12GraphicsCommandList1::resolve_subresource_region`].
///
/// To use multisampling with [`DXGI_SWAP_EFFECT::Sequential`] or
/// [`DXGI_SWAP_EFFECT::FlipSequential`], you must perform the multisampling in a separate render
/// target. For example, create a multisampled texture by calling
/// [`ID3D11Device::create_texture_2d`] with a filled [`D3D11_TEXTURE2D_DESC`] structure
/// (`bind_flags` member set to [`D3D11_BIND::RenderTarget`] and `sample_desc` member with
/// multisampling parameters). Next call [`ID3D11Device::create_render_target_view`] to create a
/// render-target view for the texture, and render your scene into the texture. Finally call
/// [`ID3D11DeviceContext::resolve_subresource`] to resolve the multisampled texture into your
/// non-multisampled swap chain.
///
/// The primary difference between presentation models is how back-buffer contents get to the
/// Desktop Window Manager (DWM) for composition. In the bitblt model, which is used with the
/// [`DXGI_SWAP_EFFECT::Discard`] and [`DXGI_SWAP_EFFECT::Sequential`] values, contents of the back
/// buffer get copied into the redirection surface on each call to [`IDXGISwapChain1::present1`].
/// In the flip model, which is used with the [`DXGI_SWAP_EFFECT::FlipSequential`] value, all back
/// buffers are shared with the DWM. Therefore, the DWM can compose straight from those back
/// buffers without any additional copy operations. In general, the flip model is the more
/// efficient model. The flip model also provides more features, such as enhanced present
/// statistics.
///
/// The difference between [`DXGI_SWAP_EFFECT::FlipSequential`] and
/// [`DXGI_SWAP_EFFECT::FlipDiscard`] is that [`DXGI_SWAP_EFFECT::FlipSequential`] forces DXGI to
/// guarantee that the contents of each back buffer is preserved across [`IDXGISwapChain::present`]
/// calls, whereas [`DXGI_SWAP_EFFECT::FlipDiscard`] doesn't provide this guarantee. The
/// compositor, under certain scenarios, can use [`DirectFlip`], where it uses the application's
/// back buffer as the entire display back buffer, which elides the cost of copying the
/// application's back buffer into the final desktop back buffer. With
/// [`DXGI_SWAP_EFFECT::FlipSequential`] and [`DXGI_SWAP_EFFECT::FlipDiscard`], this optimization
/// can occur when the application is the only item visible on the screen. However, even when the
/// application is not the only visible item on the screen, if the flip model is
/// [`DXGI_SWAP_EFFECT::FlipDiscard`], the compositor can in some scenarios still perform this
/// optimization, by drawing other content onto the application's back buffer.
///
/// When you call [`IDXGISwapChain1::present1`] on a flip model swap chain
/// ([`DXGI_SWAP_EFFECT::FlipSequential`]) with 0 specified in the `sync_interval` parameter,
/// [`IDXGISwapChain1::present1`]'s behavior is the same as the behavior of Direct3D 9Ex's
/// [`IDirect3DDevice9Ex::present_ex`] with [`D3DSWAPEFFECT::FlipEx`] and
/// [`D3DPRESENT::ForceImmediate`]. That is, the runtime not only presents the next frame instead
/// of any previously queued frames, it also terminates any remaining time left on the previously
/// queued frames.
///
/// Regardless of whether the flip model is more efficient, an application still might choose the
/// bitblt model because the bitblt model is the only way to mix GDI and DirectX presentation. In
/// the flip model, the application must create the swap chain with
/// [`DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE`], and then must use [`GetDC`] on the back buffer
/// explicitly. After the first successful call to [`IDXGISwapChain1::present1`] on a flip-model
/// swap chain, GDI no longer works with the HWND that is associated with that swap chain, even
/// after the destruction of the swap chain. This restriction even extends to methods like
/// [`ScrollWindowEx`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_SWAP_EFFECT {
    /// Use this flag to specify the bit-block transfer (bitblt) model and to specify that DXGI
    /// discard the contents of the back buffer after you call [`IDXGISwapChain1::present1`].
    ///
    /// This flag is valid for a swap chain with more than one back buffer, although, applications
    /// only have read and write access to buffer 0.
    ///
    /// Use this flag to enable the display driver to select the most efficient presentation
    /// technique for the swap chain.
    Discard = 0,

    /// Use this flag to specify the bitblt model and to specify that DXGI persist the contents of
    /// the back buffer after you call [`IDXGISwapChain1::present1`].
    ///
    /// Use this option to present the contents of the swap chain in order, from the first buffer
    /// (buffer 0) to the last buffer.
    ///
    /// This flag cannot be used with multisampling.
    Sequential = 1,

    /// Use this flag to specify the flip presentation model and to specify that DXGI persist the
    /// contents of the back buffer after you call [`IDXGISwapChain1::present1`].
    ///
    /// This flag cannot be used with multisampling.
    FlipSequential = 3,

    /// Use this flag to specify the flip presentation model and to specify that DXGI discard the
    /// contents of the back buffer after you call [`IDXGISwapChain1::Present1`].
    ///
    /// This flag cannot be used with multisampling and partial presentation.
    ///
    /// See DXGI 1.4 Improvements.
    FlipDiscard = 4,
}
