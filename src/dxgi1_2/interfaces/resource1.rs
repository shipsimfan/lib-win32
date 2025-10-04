use crate::{
    com_interface,
    dxgi::{
        IDXGIDeviceSubObject, IDXGIDeviceSubObjectTrait, IDXGIObject, IDXGIObjectTrait,
        IDXGIResource, IDXGIResourceTrait,
    },
    dxgi1_2::IDXGISurface2,
    unknwn::{IUnknown, IUnknownTrait},
    DWORD, HANDLE, HRESULT, LPCWSTR, SECURITY_ATTRIBUTES, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::D3D11_RESOURCE_MISC_FLAG,
    d3d11_1::ID3D11Device1,
    dxgi::{IDXGIDevice, IDXGISurface, IDXGISurface1},
    dxgi1_2::{DXGI_SHARED_RESOURCE_READ, DXGI_SHARED_RESOURCE_WRITE},
    CloseHandle, DXGI_ERROR_INVALID_CALL, DXGI_ERROR_NAME_ALREADY_EXISTS, E_ACCESSDENIED,
    E_OUTOFMEMORY, MAX_PATH, SECURITY_DESCRIPTOR, S_OK,
};
#[allow(unused_imports)]
use std::ptr::null;

com_interface!(
    /// An [`IDXGIResource1`] interface extends the [`IDXGIResource`] interface by adding support
    /// for creating a subresource surface object and for creating a handle to a shared resource.
    ///
    /// # Remarks
    /// To determine the type of memory a resource is currently located in, use
    /// [`IDXGIDevice::query_resource_residency`]. To share resources between processes, use
    /// [`ID3D11Device1::open_shared_resource1`]. For information about how to share resources
    /// between multiple Windows graphics APIs, including Direct3D 11, Direct2D, Direct3D 10, and
    /// Direct3D 9Ex, see Surface Sharing Between Windows Graphics APIs.
    ///
    /// You can retrieve the [`IDXGIResource1`] interface from any video memory resource that you
    /// create from a Direct3D 10 and later function. Any Direct3D object that supports
    /// [`ID3D10Resource`] or [`ID3D11Resource`] also supports [`IDXGIResource1`]. For example, the
    /// Direct3D 2D texture object that you create from [`ID3D11Device::create_texture_2d`]
    /// supports [`IDXGIResource1`]. You can call [`IUnknown::query_interface`] on the 2D texture
    /// object ([`ID3D11Texture2D`]) to retrieve the [`IDXGIResource1`] interface.
    pub abstract IDXGIResource1(IDXGIResource1VTable/IDXGIResource1Trait):
        IDXGIResource/IDXGIResourceTrait(resource) +
        IDXGIDeviceSubObject/IDXGIDeviceSubObjectTrait(resource.device_sub_object) +
        IDXGIObject/IDXGIObjectTrait(resource.device_sub_object.object) +
        IUnknown/IUnknownTrait(resource.device_sub_object.object.unknown) {
        const IID = 0x30961379-0x4609-0x4A41-0x998E-0x54FE567EE0C1;

        /// Creates a subresource surface object.
        ///
        /// # Parameters
        ///  * `index` - The index of the subresource surface object to enumerate.
        ///  * `surface` - The address of a pointer to a [`IDXGISurface2`] interface that
        ///                represents the created subresource surface object at the position
        ///                specified by the index parameter.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the following values:
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the index is out of range or if the subresource is
        ///    not a valid surface.
        ///  - [`E_OUTOFMEMORY`] if insufficient memory is available to create the subresource
        ///    surface object.
        ///
        /// A subresource is a valid surface if the original resource would have been a valid
        /// surface had its array size been equal to 1.
        ///
        /// # Remarks
        /// Subresource surface objects implement the [`IDXGISurface2`] interface, which inherits
        /// from [`IDXGISurface1`] and indirectly [`IDXGISurface`]. Therefore, the
        /// GDI-interoperable methods of [`IDXGISurface1`] work if the original resource interface
        /// object was created with the GDI-interoperable flag
        /// ([`D3D11_RESOURCE_MISC_FLAG::GdiCompatible`]).
        ///
        /// [`IDXGIResource1::create_subresource_surface`] creates a subresource surface that is
        /// based on the resource interface on which [`IDXGIResource1::create_subresource_surface`]
        /// is called. For example, if the original resource interface object is a 2D texture, the
        /// created subresource surface is also a 2D texture.
        ///
        /// You can use [`IDXGIResource1::create_subresource_surface`] to create parts of a stereo
        /// resource so you can use Direct2D on either the left or right part of the stereo
        /// resource.
        fn create_subresource_surface(
            &mut self,
            index: UINT,
            surface: *mut *mut IDXGISurface2
        ) -> HRESULT;

        /// Creates a handle to a shared resource. You can then use the returned handle with
        /// multiple Direct3D devices.
        ///
        /// # Parameters
        ///  * `attributes` - A pointer to a [`SECURITY_ATTRIBUTES`] structure that contains two
        ///                   separate but related data members: an optional security descriptor,
        ///                   and a boolean value that determines whether child processes can
        ///                   inherit the returned handle. Set this parameter to [`null`] if you
        ///                   want child processes that the application might create to not inherit
        ///                   the handle returned by [`IDXGIResource1::create_shared_handle`], and
        ///                   if you want the resource that is associated with the returned handle
        ///                   to get a default security descriptor. The `security_descriptor`
        ///                   member of the structure specifies a [`SECURITY_DESCRIPTOR`] for the
        ///                   resource. Set this member to [`null`] if you want the runtime to
        ///                   assign a default security descriptor to the resource that is
        ///                   associated with the returned handle. The ACLs in the default security
        ///                   descriptor for the resource come from the primary or impersonation
        ///                   token of the creator.
        ///  * `access` - The requested access rights to the resource. In addition to the generic
        ///               access rights, DXGI defines the following values:
        ///    * [`DXGI_SHARED_RESOURCE_READ`] - specifies read access to the resource.
        ///    * [`DXGI_SHARED_RESOURCE_WRITE`] - specifies write access to the resource.
        ///  * `name` - The name of the resource to share. The name is limited to [`MAX_PATH`]
        ///             characters. Name comparison is case sensitive. You will need the resource
        ///             name if you call the [`ID3D11Device1::open_shared_resource_by_name`] method
        ///             to access the shared resource by name. If you instead call the
        ///             [`ID3D11Device1::open_shared_resource1`] method to access the shared
        ///             resource by handle, set this parameter to [`null`]. If `name` matches the
        ///             name of an existing resource, [`IDXGIResource1::create_shared_handle`]
        ///             fails with [`DXGI_ERROR_NAME_ALREADY_EXISTS`]. This occurs because these
        ///             objects share the same namespace. The name can have a "Global" or "Local"
        ///             prefix to explicitly create the object in the global or session namespace.
        ///             The remainder of the name can contain any character except the backslash
        ///             character (\). Fast user switching is implemented using Terminal Services
        ///             sessions. Kernel object names must follow the guidelines outlined for
        ///             Terminal Services so that applications can support multiple users. The
        ///             object can be created in a private namespace.
        ///  * `handle` - A pointer to a variable that receives the NT [`HANDLE`] value to the
        ///               resource to share. You can use this handle in calls to access the
        ///               resource.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the following values:
        ///  - [`DXGI_ERROR_INVALID_CALL`] if one of the parameters is invalid.
        ///  - [`DXGI_ERROR_NAME_ALREADY_EXISTS`] if the supplied name of the resource to share is
        ///    already associated with another resource.
        ///  - [`E_ACCESSDENIED`] if the object is being created in a protected namespace.
        ///  - [`E_OUTOFMEMORY`] if sufficient memory is not available to create the handle.
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// [`IDXGIResource1::create_shared_handle`] only returns the NT handle when you created
        /// the resource as shared and specified that it uses NT handles (that is, you set the
        /// [`D3D11_RESOURCE_MISC_FLAG::SharedNtHandle`] and
        /// [`D3D11_RESOURCE_MISC_FLAG::SharedKeyedMutex`] flags). If you created the resource as
        /// shared and specified that it uses NT handles, you must use
        /// [`IDXGIResource1::create_shared_handle`] to get a handle for sharing. In this
        /// situation, you can't use the [`IDXGIResource::get_shared_handle`] method because it
        /// will fail.
        ///
        /// You can pass the handle that [`IDXGIResource1::create_shared_handle`] returns in a call
        /// to the [`ID3D11Device1::open_shared_resource1`] method to give a device access to a
        /// shared resource that you created on a different device.
        ///
        /// Because the handle that [`IDXGIResource1::create_shared_handle`] returns is an NT
        /// handle, you can use the handle with [`CloseHandle`], [`DuplicateHandle`], and so on.
        /// You can call [`IDXGIResource1::create_shared_handle`] only once for a shared resource;
        /// later calls fail. If you need more handles to the same shared resource, call
        /// [`DuplicateHandle`]. When you no longer need the shared resource handle, call
        /// [`CloseHandle`] to close the handle, in order to avoid memory leaks.
        ///
        /// If you pass a name for the resource to `name` when you call
        /// [`IDXGIResource1::create_shared_handle`] to share the resource, you can subsequently
        /// pass this name in a call to the [`ID3D11Device1::open_shared_resource_by_name`] method
        /// to give another device access to the shared resource. If you use a named resource, a
        /// malicious user can use this named resource before you do and prevent your app from
        /// starting. To prevent this situation, create a randomly named resource and store the
        /// name so that it can only be obtained by an authorized user. Alternatively, you can use
        /// a file for this purpose. To limit your app to one instance per user, create a locked
        /// file in the user's profile directory.
        ///
        /// If you created the resource as shared and did not specify that it uses NT handles, you
        /// cannot use [`IDXGIResource1::create_shared_handle`] to get a handle for sharing because
        /// [`IDXGIResource1::create_shared_handle`] will fail.
        fn create_shared_handle(
            &mut self,
            attributes: *const SECURITY_ATTRIBUTES,
            access: DWORD,
            name: LPCWSTR,
            handle: *mut HANDLE
        ) -> HRESULT;
    }
);
