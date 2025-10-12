use crate::{
    com_interface,
    d3d11::{
        ID3D11Asynchronous, ID3D11AsynchronousTrait, ID3D11DeviceChild, ID3D11DeviceChildTrait,
        D3D11_QUERY_DESC,
    },
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext, D3D11_QUERY};

com_interface!(
    /// A query interface queries information from the GPU.
    ///
    /// # Remarks
    /// A query can be created with [`ID3D11Device::create_query`].
    ///
    /// Query data is typically gathered by issuing an [`ID3D11DeviceContext::begin`] command,
    /// issuing some graphics commands, issuing an [`ID3D11DeviceContext::end`] command, and then
    /// calling [`ID3D11DeviceContext::get_data`] to get data about what happened in between the
    /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`] calls. The data returned by
    /// [`ID3D11DeviceContext::get_data`] will be different depending on the type of query.
    ///
    /// There are, however, some queries that do not require calls to Begin. For a list of possible
    /// queries see [`D3D11_QUERY`].
    pub abstract ID3D11Query(ID3D11QueryVTable/ID3D11QueryTrait):
        ID3D11Asynchronous/ID3D11AsynchronousTrait(asynchronous) +
        ID3D11DeviceChild/ID3D11DeviceChildTrait(asynchronous.device_child) +
        IUnknown/IUnknownTrait(asynchronous.device_child.unknown) {
        const IID = 0xD6C00747-0x87B7-0x425E-0xB84D-0x44D108560AFD;

        /// Get a query description.
        ///
        /// # Parameters
        ///  * `desc` - Pointer to a query description (see [`D3D11_QUERY_DESC`]).
        fn get_desc(&mut self, desc: *mut D3D11_QUERY_DESC);
    }
);
