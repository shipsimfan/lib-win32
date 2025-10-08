use crate::{
    com_interface,
    dxgi::{
        IDXGIDeviceSubObject, IDXGIDeviceSubObjectTrait, IDXGIObject, IDXGIObjectTrait,
        IDXGISurface, IDXGISurface1, IDXGISurface1Trait, IDXGISurfaceTrait,
    },
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, REFIID, UINT,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11")]
use crate::d3d11::{ID3D11Device, ID3D11Texture2D};
#[allow(unused_imports)]
use crate::{dxgi::IDXGIOutput, dxgi1_2::IDXGIResource1, E_NOINTERFACE, S_OK};

com_interface!(
    /// The [`IDXGISurface2`] interface extends the [`IDXGISurface1`] interface by adding support
    /// for subresource surfaces and getting a handle to a shared resource.
    ///
    /// # Remarks
    /// An image-data object is a 2D section of memory, commonly called a surface. To get the
    /// surface from an output, call [`IDXGIOutput::get_display_surface_data`]. Then, call
    /// [`IUnknown::query_interface`] on the [`IDXGISurface`] object that
    /// [`IDXGIOutput::get_display_surface_data`] returns to retrieve the [`IDXGISurface2`]
    /// interface.
    ///
    /// Any object that supports [`IDXGISurface`] also supports [`IDXGISurface2`].
    ///
    /// The runtime automatically creates an [`IDXGISurface2`] interface when it creates a Direct3D
    /// resource object that represents a surface. For example, the runtime creates an
    /// [`IDXGISurface2`] interface when you call [`ID3D11Device::create_texture_2d`] to create a
    /// 2D texture. To retrieve the [`IDXGISurface2`] interface that represents the 2D texture
    /// surface, call [`ID3D11Texture2D::query_interface`]. In this call, you must pass the
    /// identifier of [`IDXGISurface2`]. If the 2D texture has only a single MIP-map level and does
    /// not consist of an array of textures, [`IUnknown::query_interface`] succeeds and returns a
    /// pointer to the [`IDXGISurface2`] interface pointer. Otherwise,
    /// [`IUnknown::query_interface`] fails and does not return the pointer to [`IDXGISurface2`].
    ///
    /// You can call the [`IDXGIResource1::create_subresource_surface`] method to create an
    /// [`IDXGISurface2`] interface that refers to one subresource of a stereo resource.
    pub abstract IDXGISurface2(IDXGISurface2VTable/IDXGISurface2Trait):
        IDXGISurface1/IDXGISurface1Trait(surface1) +
        IDXGISurface/IDXGISurfaceTrait(surface1.surface) +
        IDXGIDeviceSubObject/IDXGIDeviceSubObjectTrait(surface1.surface.device_sub_object) +
        IDXGIObject/IDXGIObjectTrait(surface1.surface.device_sub_object.object) +
        IUnknown/IUnknownTrait(surface1.surface.device_sub_object.object.unknown) {
        const IID = 0xABA496DD-0xB617-0x4CB8-0xA866-0xBC44D7EB1FA2;

        /// Gets the parent resource and subresource index that support a subresource surface.
        ///
        /// # Parameters
        ///  * `iid` - The globally unique identifier (GUID) of the requested interface type.
        ///  * `parent_resource` - A pointer to a buffer that receives a pointer to the parent
        ///                        resource object for the subresource surface.
        ///  * `subresource_index` - A pointer to a variable that receives the index of the
        ///                          subresource surface.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the following values:
        ///  - [`E_NOINTERFACE`] if the object does not implement the GUID that the `iid` parameter
        ///    specifies.
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// For subresource surface objects that the [`IDXGIResource1::create_subresource_surface`]
        /// method creates, [`IDXGISurface2::get_resource`] simply returns the values that were
        /// used to create the subresource surface.
        ///
        /// Current objects that implement [`IDXGISurface`] are either resources or views.
        /// [`IDXGISurface2::get_resource`] for these objects returns “this” or the resource that
        /// supports the view respectively. In this situation, the subresource index is 0.
        fn get_resource(
            &mut self,
            iid: REFIID,
            parent_resource: *mut *mut c_void,
            subresource_index: *mut UINT
        ) -> HRESULT;
    }
);
