use crate::{
    com_interface,
    unknwn::{IUnknown},
    LPVOID, SIZE_T,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3dcommon::{self, ID3DBlob};

com_interface!(
    /// This interface is used to return arbitrary-length data.
    ///
    /// # Remarks
    /// The [`ID3DBlob`] interface is type-defined in the [`d3dcommon`] module as a [`ID3D10Blob`]
    /// interface, which is fully defined in the [`d3dCommon`] module. [`ID3DBlob`] is
    /// version-neutral and can be used in code for any Direct3D version.
    ///
    /// Blobs can be used as a data buffer, storing vertex, adjacency, and material information
    /// during mesh optimization and loading operations. Also, these objects are used to return
    /// object code and error messages in APIs that compile vertex, geometry and pixel shaders.
    pub abstract ID3D10Blob(ID3D10BlobVTable):
        IUnknown(unknown) {
        const IID = 0x8BA5FB08-0x5195-0x40E2-0xAC58-0x0D989C3A0102;

        /// Gets a pointer to the data.
        ///
        /// # Return Value
        /// Returns a pointer.
        fn get_buffer_pointer(&mut self) -> LPVOID;

        /// Gets the size.
        ///
        /// # Return Value
        /// The size of the data, in bytes.
        fn get_buffer_size(&mut self) -> SIZE_T;
    }
);
