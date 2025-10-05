use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, ID3D11DeviceChildTrait, ID3D11Resource},
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// A view interface specifies the parts of a resource the pipeline can access during rendering.
    ///
    /// # Remarks
    /// A view interface is the base interface for all views. There are four types of views; a
    /// depth-stencil view, a render-target view, a shader-resource view, and an unordered-access
    /// view.
    ///  - To create a render-target view, call [`ID3D11Device::create_render_target_view`].
    ///  - To create a depth-stencil view, call [`ID3D11Device::create_depth_stencil_view`].
    ///  - To create a shader-resource view, call [`ID3D11Device::create_shader_resource_view`].
    ///  - To create an unordered-access view, call [`ID3D11Device::create_unordered_access_view`].
    ///
    /// All resources must be bound to the pipeline before they can be accessed.
    ///  - To bind a render-target view or a depth-stencil view, call
    ///    [`ID3D11DeviceContext::om_set_render_targets`].
    ///  - To bind a shader resource, call [`ID3D11DeviceContext::vs_set_shader_resources`].
    pub abstract ID3D11View(ID3D11ViewVTable/ID3D11ViewTrait):
        ID3D11DeviceChild/ID3D11DeviceChildTrait(device_child) +
        IUnknown/IUnknownTrait(device_child.unknown) {
        const IID = 0x839D1216-0xBB2E-0x412B-0xB7F4-0xA9DBEBE08ED1;

        /// Get the resource that is accessed through this view.
        ///
        /// # Parameters
        ///  * `resource` - Address of a pointer to the resource that is accessed through this
        ///                 view. (See [`ID3D11Resource`].)
        ///
        /// # Remarks
        /// This function increments the reference count of the resource by one, so it is necessary
        /// to call [`IUnknown::release`] on the returned pointer when the application is done with
        /// it. Destroying (or losing) the returned pointer before [`IUnknown::release`] is called
        /// will result in a memory leak.
        fn get_resource(&mut self, resource: *mut *mut ID3D11Resource);
    }
);
