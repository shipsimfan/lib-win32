// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{
        ID3D11Device, ID3D11DeviceContext, D3D11_BIND_FLAG, D3D11_BUFFER_DESC,
        D3D11_CPU_ACCESS_FLAG, D3D11_USAGE,
    },
    dxgi::{IDXGIKeyedMutex, IDXGISurface1, DXGI_FORMAT, DXGI_SAMPLE_DESC},
    unknwn::IUnknown,
    E_OUTOFMEMORY,
};

/// Identifies options for resources.
///
/// # Remarks
/// This enumeration is used in [`D3D11_BUFFER_DESC`], [`D3D11_TEXTURE1D_DESC`],
/// [`D3D11_TEXTURE2D_DESC`], [`D3D11_TEXTURE3D_DESC`].
///
/// These flags can be combined by bitwise OR.
///
/// The [`D3D11_RESOURCE_MISC_FLAG`] cannot be used when creating resources with
/// [`D3D11_CPU_ACCESS_FLAG`]s
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_RESOURCE_MISC_FLAG {
    /// Enables MIP map generation by using [`ID3D11DeviceContext::generate_mips`] on a texture
    /// resource. The resource must be created with the bind flags that specify that the resource
    /// is a render target and a shader resource.
    GenerateMips = 0x1,

    /// Enables resource data sharing between two or more Direct3D devices. The only resources that
    /// can be shared are 2D non-mipmapped textures.
    ///
    /// [`D3D11_RESOURCE_MISC_FLAG::Shared`] and [`D3D11_RESOURCE_MISC_FLAG::SharedKeyedMutex`] are
    /// mutually exclusive.
    ///
    /// `WARP` and `REF` devices do not support shared resources. If you try to create a resource
    /// with this flag on either a `WARP` or `REF` device, the create method will return an
    /// [`E_OUTOFMEMORY`] error code.
    Shared = 0x2,

    /// Sets a resource to be a cube texture created from a `Texture2DArray` that contains 6
    /// textures.
    TextureCube = 0x4,

    /// Enables instancing of GPU-generated content.
    DrawIndirectArgs = 0x10,

    /// Enables a resource as a byte address buffer.
    BufferAllowRawViews = 0x20,

    /// Enables a resource as a structured buffer.
    BufferStructured = 0x40,

    /// Enables a resource with MIP map clamping for use with
    /// [`ID3D11DeviceContext::set_resource_min_lod`].
    ResourceClamp = 0x80,

    /// Enables the resource to be synchronized by using the [`IDXGIKeyedMutex::acquire_sync`] and
    /// [`IDXGIKeyedMutex::release_sync`] APIs. The following Direct3D 11 resource creation APIs,
    /// that take [`D3D11_RESOURCE_MISC_FLAG`] parameters, have been extended to support the new
    /// flag.
    ///  - [`ID3D11Device::create_texture_1d`]
    ///  - [`ID3D11Device::create_texture_2d`]
    ///  - [`ID3D11Device::create_texture_3d`]
    ///  - [`ID3D11Device::create_buffer`]
    ///
    /// If you call any of these methods with the [`D3D11_RESOURCE_MISC_FLAG::SharedKeyedMutex`]
    /// flag set, the interface returned will support the [`IDXGIKeyedMutex`] interface. You can
    /// retrieve a pointer to the [`IDXGIKeyedMutex`] interface from the resource by using
    /// [`IUnknown::query_interface`]. The [`IDXGIKeyedMutex`] interface implements the
    /// [`IDXGIKeyedMutex::acquire_sync`] and [`IDXGIKeyedMutex::release_sync`] APIs to synchronize
    /// access to the surface. The device that creates the surface, and any other device that opens
    /// the surface by using `open_shared_resource`, must call [`IDXGIKeyedMutex::acquire_sync`]
    /// before they issue any rendering commands to the surface. When those devices finish
    /// rendering, they must call [`IDXGIKeyedMutex::release_sync`].
    ///
    /// [`D3D11_RESOURCE_MISC_FLAG::Shared`] and [`D3D11_RESOURCE_MISC_FLAG::SharedKeyedMutex`] are
    /// mutually exclusive.
    ///
    /// `WARP` and `REF` devices do not support shared resources. If you try to create a resource
    /// with this flag on either a `WARP` or `REF` device, the create method will return an
    /// [`E_OUTOFMEMORY`] error code.
    SharedKeyedMutex = 0x100,

    /// Enables a resource compatible with GDI. You must set the
    /// [`D3D11_RESOURCE_MISC_FLAG::GdiCompatible`] flag on surfaces that you use with GDI. Setting
    /// the [`D3D11_RESOURCE_MISC_FLAG::GdiCompatible`] flag allows GDI rendering on the surface
    /// via [`IDXGISurface1::get_dc`].
    ///
    /// Consider the following programming tips for using
    /// [`D3D11_RESOURCE_MISC_FLAG::GdiCompatible`] when you create a texture or use that texture
    /// in a swap chain:
    ///  - [`D3D11_RESOURCE_MISC_FLAG::SharedKeyedMutex`] and
    ///    [`D3D11_RESOURCE_MISC_FLAG::GdiCompatible`] are mutually exclusive. Therefore, do not
    ///    use them together.
    ///  - [`D3D11_RESOURCE_MISC_FLAG::ResourceClamp`] and
    ///    [`D3D11_RESOURCE_MISC_FLAG::GdiCompatible`] are mutually exclusive. Therefore, do not
    ///    use them together.
    ///  - You must bind the texture as a render target for the output-merger stage. For example,
    ///    set the [`D3D11_BIND_FLAG::RenderTarget`] flag in the `bind_flags` member of the
    ///    [`D3D11_TEXTURE2D_DESC`] structure.
    ///  - You must set the maximum number of MIP map levels to 1. For example, set the
    ///    `mip_levels` member of the [`D3D11_TEXTURE2D_DESC`] structure to 1.
    ///  - You must specify that the texture requires read and write access by the GPU. For
    ///    example, set the `usage` member of the [`D3D11_TEXTURE2D_DESC`] structure to
    ///    [`D3D11_USAGE::Default`].
    ///  - You must set the texture format to one of the following types.
    ///    - [`DXGI_FORMAT::B8G8R8A8UNorm`]
    ///    - [`DXGI_FORMAT::B8G8R8A8Typeless`]
    ///    - [`DXGI_FORMAT::B8G8R8A8UNormSRGB`]
    /// - You cannot use [`D3D11_RESOURCE_MISC_FLAG::GdiCompatible`] with multisampling. Therefore,
    ///   set the `count` member of the [`DXGI_SAMPLE_DESC`] structure to 1. Then, set the
    ///   `sample_desc` member of the [`D3D11_TEXTURE2D_DESC`] structure to this
    ///   [`DXGI_SAMPLE_DESC`] structure.
    GdiCompatible = 0x200,

    /// Set this flag to enable the use of NT HANDLE values when you create a shared resource. By
    /// enabling this flag, you deprecate the use of existing HANDLE values.
    ///
    /// The value specifies a new shared resource type that directs the runtime to use NT HANDLE
    /// values for the shared resource. The runtime then must confirm that the shared resource
    /// works on all hardware at the specified feature level.
    ///
    /// Without this flag set, the runtime does not strictly validate shared resource parameters
    /// (that is, formats, flags, usage, and so on). When the runtime does not validate shared
    /// resource parameters, behavior of much of the Direct3D API might be undefined and might vary
    /// from driver to driver.
    SharedNtHandle = 0x800,

    /// Set this flag to indicate that the resource might contain protected content; therefore, the
    /// operating system should use the resource only when the driver and hardware support content
    /// protection. If the driver and hardware do not support content protection and you try to
    /// create a resource with this flag, the resource creation fails.
    RestrictedContent = 0x1000,

    /// Set this flag to indicate that the operating system restricts access to the shared surface.
    /// You can use this flag together with the
    /// [`D3D11_RESOURCE_MISC_FLAG::RestrictSharedResourceDriver`] flag and only when you create a
    /// shared surface. The process that creates the shared resource can always open the shared
    /// resource.
    RestrictSharedResource = 0x2000,

    /// Set this flag to indicate that the driver restricts access to the shared surface. You can
    /// use this flag in conjunction with the [`D3D11_RESOURCE_MISC_FLAG::RestrictSharedResource`]
    /// flag and only when you create a shared surface. The process that creates the shared
    /// resource can always open the shared resource.
    RestrictSharedResourceDriver = 0x4000,

    /// Set this flag to indicate that the resource is guarded. Such a resource is returned by the
    /// [`IDCompositionSurface::begin_draw`] (DirectComposition) and
    /// [`ISurfaceImageSourceNative::begin_draw`] (Windows Runtime) APIs. For these APIs, you
    /// provide a region of interest (ROI) on a surface to update. This surface isn't compatible
    /// with multiple render targets (MRT).
    ///
    /// A guarded resource automatically restricts all writes to the region that is related to one
    /// of the preceding APIs. Additionally, the resource enforces access to the ROI with these
    /// restrictions:
    ///  - Copy operations from the resource by using [`ID3D11DeviceContext::copy_resource`] or
    ///    [`ID3D11DeviceContext::copy_subresource_region`] are restricted to only copy from the
    ///    ROI.
    ///  - When a guarded resource is set as a render target, it must be the only target.
    Guarded = 0x8000,

    /// Set this flag to indicate that the resource is a tile pool.
    TilePool = 0x20000,

    /// Set this flag to indicate that the resource is a tiled resource.
    Tiled = 0x40000,

    /// Set this flag to indicate that the resource should be created such that it will be
    /// protected by the hardware. Resource creation will fail if hardware content protection is
    /// not supported.
    ///
    /// This flag has the following restrictions:
    ///  - This flag cannot be used with the following [`D3D11_USAGE`] values:
    ///    - [`D3D11_USAGE::Dynamic`]
    ///    - [`D3D11_USAGE::Staging`]
    ///  - This flag cannot be used with the following [`D3D11_BIND_FLAG`] values.
    ///    - [`D3D11_BIND_FLAG::VertexBuffer`]
    ///    - [`D3D11_BIND_FLAG::IndexBuffer`]
    ///  - No CPU access flags can be specified.
    HwProtected = 0x80000,

    /// Enables the resource to work with the displayable surfaces feature. You must use
    /// [`D3D11_RESOURCE_MISC_FLAG::SharedDisplayable`] in combination with both
    /// [`D3D11_RESOURCE_MISC_FLAG::Shared`] and [`D3D11_RESOURCE_MISC_FLAG::SharedNtHandle`].
    SharedDisplayable,

    /// TBD
    SharedExclusiveWriter,

    #[allow(missing_docs)]
    NoShaderAccess,
}
