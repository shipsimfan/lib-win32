use crate::{
    com_interface,
    dxgi::{
        IDXGIDeviceSubObject, IDXGIObject,
        IDXGISurface,
    },
    unknwn::{IUnknown},
    BOOL, HDC, HRESULT, RECT,
};

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11")]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D, D3D11_RESOURCE_MISC_FLAG};
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGIOutput, DXGI_FORMAT, DXGI_SWAP_CHAIN_FLAG},
    S_OK, TRUE,
};

com_interface!(
    /// The [`IDXGISurface1`] interface extends the [`IDXGISurface`] by adding support for using
    /// Windows Graphics Device Interface (GDI) to render to a Microsoft DirectX Graphics
    /// Infrastructure (DXGI) surface.
    ///
    /// # Remarks
    /// This interface is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
    /// Server 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows Server
    /// 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows Server
    /// 2008.
    ///
    /// An image-data object is a 2D section of memory, commonly called a surface. To get the
    /// surface from an output, call [`IDXGIOutput::get_display_surface_data`]. Then, call
    /// [`IUnknown::query_interface`] on the [`IDXGISurface`] object that
    /// [`IDXGIOutput::get_display_surface_data`] returns to retrieve the [`IDXGISurface1`]
    /// interface.
    ///
    /// Any object that supports [`IDXGISurface`] also supports [`IDXGISurface1`].
    ///
    /// The runtime automatically creates an [`IDXGISurface1`] interface when it creates a Direct3D
    /// resource object that represents a surface. For example, the runtime creates an
    /// [`IDXGISurface1`] interface when you call [`ID3D11Device::create_texture_2d`] or
    /// [`ID3D10Device::create_texture_2d`] to create a 2D texture. To retrieve the
    /// [`IDXGISurface1`] interface that represents the 2D texture surface, call
    /// [`ID3D11Texture2D::query_interface`] or [`ID3D10Texture2D::query_interface`]. In this call,
    /// you must pass the identifier of [`IDXGISurface1`]. If the 2D texture has only a single
    /// MIP-map level and does not consist of an array of textures, [`IUnknown::query_interface`]
    /// succeeds and returns a pointer to the [`IDXGISurface1`] interface pointer. Otherwise,
    /// [`IUnknown::query_interface`] fails and does not return the pointer to [`IDXGISurface1`].
    pub abstract IDXGISurface1(IDXGISurface1VTable):
        IDXGISurface(surface) +
        IDXGIDeviceSubObject +
        IDXGIObject +
        IUnknown {
        const IID = 0x4AE63092-0x6327-0x4C1B-0x80AE-0xBFE12EA32B86;

        /// Returns a device context (DC) that allows you to render to a Microsoft DirectX Graphics
        /// Infrastructure (DXGI) surface using Windows Graphics Device Interface (GDI).
        ///
        /// # Parameters
        ///  * `discard` - A Boolean value that specifies whether to preserve Direct3D contents in
        ///                the GDI DC. [`TRUE`] directs the runtime not to preserve Direct3D
        ///                contents in the GDI DC; that is, the runtime discards the Direct3D
        ///                contents. [`FALSE`] guarantees that Direct3D contents are available in
        ///                the GDI DC.
        ///  * `dc` - A pointer to an HDC handle that represents the current device context for GDI
        ///           rendering.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, an error code.
        ///
        /// # Remarks
        /// This method is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
        /// Server 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows
        /// Server 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows
        /// Server 2008.
        ///
        /// After you use the [`IDXGISurface1::get_dc`] method to retrieve a DC, you can render to
        /// the DXGI surface by using GDI. The [`IDXGISurface1::get_dc`] method readies the surface
        /// for GDI rendering and allows inter-operation between DXGI and GDI technologies.
        ///
        /// Keep the following in mind when using this method:
        ///  - You must create the surface by using the [`D3D11_RESOURCE_MISC_FLAG::GdiCompatible`]
        ///    flag for a surface or by using the [`DXGI_SWAP_CHAIN_FLAG::GdiCompatible`] flag for
        ///    swap chains, otherwise this method fails.
        ///  - You must release the device and call the [`IDXGISurface1::release_dc`] method before
        ///    you issue any new Direct3D commands.
        ///  - This method fails if an outstanding DC has already been created by this method.
        ///  - The format for the surface or swap chain must be [`DXGI_FORMAT::B8G8R8A8UNormSRGB`]
        ///    or [`DXGI_FORMAT::B8G8R8A8UNorm`].
        ///  - On [`IDXGISurface1::get_dc`], the render target in the output merger of the Direct3D
        ///    pipeline is unbound from the surface. You must call the
        ///    [`ID3D11DeviceContext::om_set_render_targets`] method on the device prior to
        ///    Direct3D rendering after GDI rendering.
        ///  - Prior to resizing buffers you must release all outstanding DCs.
        ///
        /// You can also call [`IDXGISurface1::get_dc`] on the back buffer at index 0 of a swap
        /// chain by obtaining an [`IDXGISurface1`] from the swap chain.
        fn get_dc(&mut self, discard: BOOL, dc: *mut HDC) -> HRESULT;

        /// Releases the GDI device context (DC) that is associated with the current surface and
        /// allows you to use Direct3D to render.
        ///
        /// # Parameters
        ///  * `dirty_rect` - A pointer to a [`RECT`] structure that identifies the dirty region of
        ///                   the surface. A dirty region is any part of the surface that you used
        ///                   for GDI rendering and that you want to preserve. This area is used as
        ///                   a performance hint to graphics subsystem in certain scenarios. Do not
        ///                   use this parameter to restrict rendering to the specified rectangular
        ///                   region. If you pass in [`null_mut`], [`IDXGISurface1::release_dc`]
        ///                   considers the whole surface as dirty. Otherwise,
        ///                   [`IDXGISurface1::release_dc`] uses the area specified by the [`RECT`]
        ///                   as a performance hint to indicate what areas have been manipulated by
        ///                   GDI rendering. You can pass a pointer to an empty [`RECT`] structure
        ///                   (a rectangle with no position or area) if you didn't change any
        ///                   content.
        ///
        /// # Return Value
        /// If this method succeeds, it returns [`S_OK`]. Otherwise, it returns an [`HRESULT`]
        /// error code.
        ///
        /// # Remarks
        /// This method is not supported by DXGI 1.0, which shipped in Windows Vista and Windows
        /// Server 2008. DXGI 1.1 support is required, which is available on Windows 7, Windows
        /// Server 2008 R2, and as an update to Windows Vista with Service Pack 2 (SP2) and Windows
        /// Server 2008.
        ///
        /// Use the [`IDXGISurface1::release_dc`] method to release the DC and indicate that your
        /// application finished all GDI rendering to this surface. You must call the
        /// [`IDXGISurface1::release_dc`] method before you can use Direct3D to perform additional
        /// rendering.
        ///
        /// Prior to resizing buffers you must release all outstanding DCs.
        fn release_dc(&mut self, dirty_rect: *mut RECT) -> HRESULT;
    }
);
