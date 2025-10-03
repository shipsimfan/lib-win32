use crate::{
    com_interface,
    d3d11::{ID3D11Buffer, D3D11_BUFFER_DESC, D3D11_SUBRESOURCE_DATA},
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{
        ID3D11DeviceContext, D3D11_BIND_FLAG, D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT, D3D11_USAGE,
    },
    E_OUTOFMEMORY, S_FALSE,
};
#[allow(unused_imports)]
use std::ptr::null;

com_interface!(
    /// The device interface represents a virtual adapter; it is used to create resources.
    pub abstract ID3D11Device(ID3D11DeviceVTable/ID3D11DeviceTrait):
        IUnknown/IUnknownTrait(unknown) {
        const IID = 0xDB6F6DDB-0xAC77-0x4E88-0x8253-0x819DF9BBF140;

        /// Creates a buffer (vertex buffer, index buffer, or shader-constant buffer).
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_BUFFER_DESC`] structure that describes the buffer.
        ///  * `initial_data` - A pointer to a [`D3D11_SUBRESOURCE_DATA`] structure that describes
        ///                     the initialization data; use [`null`] to allocate space only (with
        ///                     the exception that it cannot be [`null`] if the usage flag is
        ///                     [`D3D11_USAGE::Immutable`]). If you don't pass anything to
        ///                     `inital_data`, the initial content of the memory for the buffer is
        ///                     undefined. In this case, you need to write the buffer content some
        ///                     other way before the resource is read.
        ///  * `buffer` - Address of a pointer to the [`ID3D11Buffer`] interface for the buffer
        ///               object created. Set this parameter to [`null`] to validate the other
        ///               input parameters ([`S_FALSE`] indicates a pass).
        ///
        /// # Return Value
        /// This method returns [`E_OUTOFMEMORY`] if there is insufficient memory to create the
        /// buffer.
        ///
        /// # Remarks
        /// For a constant buffer (`bind_flags` of [`D3D11_BUFFER_DESC`] set to
        /// [`D3D11_BIND_FLAG::ConstantBuffer`]), you must set the `byte_width` value of
        /// [`D3D11_BUFFER_DESC`] in multiples of 16, and less than or equal to
        /// [`D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT`].
        ///
        /// The Direct3D 11.1 runtime, which is available on Windows 8 and later operating
        /// systems, provides the following new functionality for [`ID3D11Device::create_buffer`]:
        ///
        /// You can create a constant buffer that is larger than the maximum constant buffer size
        /// that a shader can access (4096 32-bit*4-component constants – 64KB). When you bind the
        /// constant buffer to the pipeline (for example, via
        /// [`ID3D11DeviceContext::ps_set_constant_buffers`] or
        /// [`ID3D11DeviceContext::ps_set_constant_buffers1`]), you can define a range of the
        /// buffer that the shader can access that fits within the 4096 constant limit.
        ///
        /// The Direct3D 11.1 runtime (available in Windows 8 and later operating systems) emulates
        /// this feature for feature level 9.1, 9.2, and 9.3; therefore, this feature is supported
        /// for feature level 9.1, 9.2, and 9.3.
        ///
        /// This feature is always available on new drivers for feature level 10 and higher.
        ///
        /// On runtimes older than Direct3D 11.1, a call to [`ID3D11Device::create_buffer`] to
        /// request a constant buffer that is larger than 4096 fails.
        fn create_buffer(
            &mut self,
            desc: *const D3D11_BUFFER_DESC,
            initial_data: *const D3D11_SUBRESOURCE_DATA,
            buffer: *mut *mut ID3D11Buffer
        ) -> HRESULT;
    }
);
