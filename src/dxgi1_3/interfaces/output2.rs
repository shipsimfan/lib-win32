use crate::{
    com_interface,
    dxgi::{IDXGIObject, IDXGIOutput},
    dxgi1_2::{IDXGIOutput1},
    unknwn::{IUnknown},
    BOOL,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGIAdapter, IDXGISwapChain, DXGI_SWAP_CHAIN_FLAG},
    FALSE, TRUE,
};

com_interface!(
    /// Represents an adapter output (such as a monitor). The [`IDXGIOutput2`] interface exposes a
    /// method to check for multiplane overlay support on the primary output adapter.
    ///
    /// # Remarks
    /// To determine the outputs that are available from the adapter, use
    /// [`IDXGIAdapter::enum_outputs`]. To determine the specific output that the swap chain will
    /// update, use [`IDXGISwapChain::get_containing_output`]. You can then call
    /// [`IUnknown::query_interface`] from any [`IDXGIOutput`] or [`IDXGIOutput1`] object to obtain
    /// an [`IDXGIOutput2`] object.
    pub abstract IDXGIOutput2(IDXGIOutput2VTable):
        IDXGIOutput1(output1) +
        IDXGIOutput +
        IDXGIObject +
        IUnknown {
        const IID = 0x595E39D1-0x2724-0x4663-0x99B1-0xDA969DE28364;

        /// Queries an adapter output for multiplane overlay support. If this API returns [`TRUE`],
        /// multiple swap chain composition takes place in a performant manner using overlay
        /// hardware. If this API returns [`FALSE`], apps should avoid using foreground swap chains
        /// (that is, avoid using swap chains created with the
        /// [`DXGI_SWAP_CHAIN_FLAG::ForegroundLayer`] flag).
        ///
        /// # Return Value
        /// [`TRUE`] if the output adapter is the primary adapter and it supports multiplane
        /// overlays, otherwise returns [`FALSE`].
        ///
        /// # Remarks
        /// See [`IDXGIFactory2::create_swap_chain_for_core_window`] for info on creating a
        /// foreground swap chain.
        fn supports_overlays(&mut self) -> BOOL;
    }
);
