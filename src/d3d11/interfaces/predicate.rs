use crate::{
    com_interface,
    d3d11::{
        ID3D11Asynchronous, ID3D11AsynchronousTrait, ID3D11DeviceChild, ID3D11DeviceChildTrait,
        ID3D11Query, ID3D11QueryTrait,
    },
    unknwn::{IUnknown, IUnknownTrait},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

com_interface!(
    /// A predicate interface determines whether geometry should be processed depending on the
    /// results of a previous draw call.
    ///
    /// # Remarks
    /// To create a predicate object, call [`ID3D11Device::create_predicate`]. To set the predicate
    /// object, call [`ID3D11DeviceContext::set_predication`].
    ///
    /// There are two types of predicates: stream-output-overflow predicates and occlusion
    /// predicates. Stream-output-overflow predicates cause any geometry residing in stream-output
    /// buffers that were overflowed to not be processed. Occlusion predicates cause any geometry
    /// that did not have a single sample pass the depth/stencil tests to not be processed.
    pub abstract ID3D11Predicate(ID3D11PredicateVTable/ID3D11PredicateTrait):
        ID3D11Query/ID3D11QueryTrait(query) +
        ID3D11Asynchronous/ID3D11AsynchronousTrait(query.asynchronous) +
        ID3D11DeviceChild/ID3D11DeviceChildTrait(query.asynchronous.device_child) +
        IUnknown/IUnknownTrait(query.asynchronous.device_child.unknown) {
        const IID = 0x9EB576DD-0x9F77-0x4D86-0x81AA-0x8BAB5FE490E2;
    }
);
