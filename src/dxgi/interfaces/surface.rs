use crate::{
    com_interface,
    dxgi::{
        IDXGIDeviceSubObject, IDXGIObject,
        DXGI_MAPPED_RECT, DXGI_SURFACE_DESC,
    },
    unknwn::{IUnknown},
    HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{DXGI_MAP_DISCARD, DXGI_MAP_READ, DXGI_MAP_WRITE},
    S_OK,
};

com_interface!(
    /// The [`IDXGISurface`] interface implements methods for image-data objects.
    pub abstract IDXGISurface(IDXGISurfaceVTable):
        IDXGIDeviceSubObject(device_sub_object) +
        IDXGIObject +
        IUnknown {
        const IID = 0xCAFCB56C-0x6AC3-0x4889-0xBF47-0x9E23BBD260EC;

        /// Get a description of the surface.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to the surface description (see [`DXGI_SURFACE_DESC`]).
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the `DXGI_ERROR`.
        fn get_desc(&mut self, desc: *mut DXGI_SURFACE_DESC) -> HRESULT;

        /// Get a pointer to the data contained in the surface, and deny GPU access to the surface.
        ///
        /// # Parameters
        ///  * `locked_rect` - A pointer to the surface data (see [`DXGI_MAPPED_RECT`]).
        ///  * `map_flags` - CPU read-write flags. These flags can be combined with a logical OR.
        ///    * [`DXGI_MAP_READ`] - Allow CPU read access.
        ///    * [`DXGI_MAP_WRITE`] - Allow CPU write access.
        ///    * [`DXGI_MAP_DISCARD`] - Discard the previous contents of a resource when it is
        ///                             mapped.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the `DXGI_ERROR`.
        ///
        /// # Remarks
        /// Use [`IDXGISurface::map`] to access a surface from the CPU. To release a mapped surface
        /// (and allow GPU access) call [`IDXGISurface::unmap`].
        fn map(&mut self, locked_rect: *mut DXGI_MAPPED_RECT, map_flags: UINT) -> HRESULT;

        /// Invalidate the pointer to the surface retrieved by [`IDXGISurface::map`] and re-enable
        /// GPU access to the resource.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the `DXGI_ERROR`.
        fn unmap(&mut self) -> HRESULT;
    }
);
