// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGIKeyedMutex, IDXGISurface1},
    E_OUTOFMEMORY,
};

/// Identifies other, less common options for resources.
///
/// # Remarks
/// This enumeration is used in [`D3D10_BUFFER_DESC`], [`D3D10_TEXTURE1D_DESC`],
/// [`D3D10_TEXTURE2D_DESC`], [`D3D10_TEXTURE3D_DESC`], [`D3DX10_IMAGE_INFO`], and
/// [`D3DX10_IMAGE_LOAD_INFO`].
///
/// These flags can be combined by bitwise OR.
///
/// [`D3D10_RESOURCE_MISC_FLAG::Shared`] and [`D3D10_RESOURCE_MISC_FLAG::SharedKeyedMutex`] are
/// mutually exclusive flags: either one may be set in the resource creation calls but not both
/// simultaneously.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D10_RESOURCE_MISC_FLAG {
    /// Enables an application to call [`ID3D10Device::generate_mips`] on a texture resource. The
    /// resource must be created with the bind flags that specify that the resource is a render
    /// target and a shader resource.
    GenerateMips = 0x1,

    /// Enables the sharing of resource data between two or more Direct3D devices. The only
    /// resources that can be shared are 2D non-mipmapped textures.
    ///
    /// WARP and REF devices do not support shared resources. Attempting to create a resource with
    /// this flag on either a WARP or REF device will cause the create method to return an
    /// [`E_OUTOFMEMORY`] error code.
    Shared = 0x2,

    /// Enables an application to create a cube texture from a Texture2DArray that contains 6
    /// textures.
    TextureCube = 0x4,

    /// Enables the resource created to be synchronized using the [`IDXGIKeyedMutex::acquire_sync`]
    /// and [`IDXGIKeyedMutex::release_sync`] APIs. The following resource creation D3D10 APIs,
    /// that all take a [`D3D10_RESOURCE_MISC_FLAG`] parameter, have been extended to support the
    /// new flag.
    ///  - [`ID3D10Device1::create_texture_1d`]
    ///  - [`ID3D10Device1::create_texture_2d`]
    ///  - [`ID3D10Device1::create_texture_3d`]
    ///  - [`ID3D10Device1::create_buffer`]
    ///
    /// If any of the listed functions are called with the
    /// [`D3D10_RESOURCE_MISC_FLAG::SharedKeyedMutex`] flag set, the interface returned can be
    /// queried for an [`IDXGIKeyedMutex`] interface, which implements
    /// [`IDXGIKeyedMutex::acquire_sync`] and [`IDXGIKeyedMutex::release_sync`] APIs to synchronize
    /// access to the surface. The device creating the surface, and any other device opening the
    /// surface (using [`ID3D10Device::open_shared_resource`]) is required to call
    /// [`IDXGIKeyedMutex::acquire_sync`] before any rendering commands to the surface, and
    /// [`IDXGIKeyedMutex::release_sync`] when it is done rendering.
    ///
    /// WARP and REF devices do not support shared resources. Attempting to create a resource with
    /// this flag on either a WARP or REF device will cause the create method to return an
    /// [`E_OUTOFMEMORY`] error code.
    SharedKeyedMutex = 0x10,

    /// Enables a surface to be used for GDI interoperability. Setting this flag enables rendering
    /// on the surface via [`IDXGISurface1::get_dc`].
    GdiCompatible = 0x20,
}
