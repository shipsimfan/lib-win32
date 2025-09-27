use crate::{
    com_interface,
    d3d11::{
        ID3D11DeviceChild, ID3D11DeviceChildTrait, ID3D11Resource, ID3D11ResourceTrait,
        D3D11_BUFFER_DESC,
    },
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Device;

com_interface!(
    /// A buffer interface accesses a buffer resource, which is unstructured memory. Buffers
    /// typically store vertex or index data.
    ///
    /// # Remarks
    /// There are three types of buffers: vertex, index, or a shader-constant buffer. Create a
    /// buffer resource by calling [`ID3D11Device::create_buffer`].
    ///
    /// A buffer must be bound to the pipeline before it can be accessed. Buffers can be bound to
    /// the input-assembler stage by calls to [`ID3D11DeviceContext::ia_set_vertex_buffers`] and
    /// [`ID3D11DeviceContext::ia_set_index_buffer`], to the stream-output stage by a call to
    /// [`ID3D11DeviceContext::so_set_targets`], and to a shader stage by calling the appropriate
    /// shader method (such as [`ID3D11DeviceContext::vs_set_constant_buffers`] for example).
    ///
    /// Buffers can be bound to multiple pipeline stages simultaneously for reading. A buffer can
    /// also be bound to a single pipeline stage for writing; however, the same buffer cannot be
    /// bound for reading and writing simultaneously.
    pub abstract ID3D11Buffer(ID3D11BufferVTable/ID3D11BufferTrait):
        ID3D11Resource/ID3D11ResourceTrait(resource) +
        ID3D11DeviceChild/ID3D11DeviceChildTrait(resource.device_child) +
        IUnknown/IUnknownTrait(resource.device_child.unknown) {
        const IID = 0x48570B85-0xD1EE-0x4FCD-0xA250-0xEB350722B037;

        /// Get the properties of a buffer resource.
        ///
        /// # Parameters
        ///  * `desc` - Pointer to a resource description (see D3D11_BUFFER_DESC) filled in by the
        ///             method.
        fn get_desc(&mut self, desc: *const D3D11_BUFFER_DESC);
    }
);
