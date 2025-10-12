use crate::{
    com_interface,
    d3d11::{ID3D11DeviceChild, ID3D11DeviceChildTrait},
    unknwn::{IUnknown, IUnknownTrait},
    UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Counter, ID3D11DeviceContext, ID3D11Predicate, ID3D11Query};

com_interface!(
    /// This interface encapsulates methods for retrieving data from the GPU asynchronously.
    ///
    /// # Remarks
    /// There are three types of asynchronous interfaces, all of which inherit this interface:
    ///  * [`ID3D11Query`] - Queries information from the GPU.
    ///  * [`ID3D11Predicate`] - Determines whether a piece of geometry should be processed or not
    ///                          depending on the results of a previous draw call.
    ///  * [`ID3D11Counter`] - Measures GPU performance.
    pub abstract ID3D11Asynchronous(ID3D11AsynchronousVTable/ID3D11AsynchronousTrait):
        ID3D11DeviceChild/ID3D11DeviceChildTrait(device_child) +
        IUnknown/IUnknownTrait(device_child.unknown) {
        const IID = 0x4B35D0CD-0x1E15-0x4258-0x9C98-0x1B1333F6DD3B;

        /// Get the size of the data (in bytes) that is output when calling
        /// [`ID3D11DeviceContext::get_data`].
        ///
        /// # Return Value
        /// Size of the data (in bytes) that is output when calling
        /// [`ID3D11DeviceContext::get_data`].
        fn get_data_size(&mut self) -> UINT;
    }
);
