use crate::{
    com_interface,
    dxgi::{
        IDXGIDevice, IDXGIDevice1, IDXGIDevice1Trait, IDXGIDeviceTrait, IDXGIObject,
        IDXGIObjectTrait, IDXGIResource,
    },
    dxgi1_2::{IDXGIDevice2, IDXGIDevice2Trait, DXGI_OFFER_RESOURCE_PRIORITY},
    dxgi1_3::{IDXGIDevice3, IDXGIDevice3Trait},
    dxgi1_5::DXGI_RECLAIM_RESOURCE_RESULTS,
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::ID3D11DeviceContext,
    dxgi::{IDXGIKeyedMutex, IDXGISwapChain},
    dxgi1_2::IDXGISwapChain1,
    dxgi1_5::DXGI_OFFER_RESOURCE_FLAGS,
    E_INVALIDARG,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// This interface provides updated methods to offer and reclaim resources.
    ///
    /// # Remarks
    /// The Direct3D create device functions return a Direct3D device object. This Direct3D device
    /// object implements the [`IUnknown`] interface. You can query this Direct3D device object for
    /// the device's corresponding [`IDXGIDevice4`] interface.
    pub abstract IDXGIDevice4(IDXGIDevice4VTable/IDXGIDevice4Trait):
        IDXGIDevice3/IDXGIDevice3Trait(device3) +
        IDXGIDevice2/IDXGIDevice2Trait(device3.device2) +
        IDXGIDevice1/IDXGIDevice1Trait(device3.device2.device1) +
        IDXGIDevice/IDXGIDeviceTrait(device3.device2.device1.device) +
        IDXGIObject/IDXGIObjectTrait(device3.device2.device1.device.object) +
        IUnknown/IUnknownTrait(device3.device2.device1.device.object.unknown) {
        const IID = 0x95B4F95F-0xD8DA-0x4CA4-0x9EE6-0x3B76D5968A10;

        /// Allows the operating system to free the video memory of resources, including both
        /// discarding the content and de-committing the memory.
        ///
        /// # Parameters
        ///  * `num_resources` - The number of resources in the ppResources argument array.
        ///  * `resources` - An array of pointers to [`IDXGIResource`] interfaces for the resources
        ///                  to offer.
        ///  * `priority` - A [`DXGI_OFFER_RESOURCE_PRIORITY`]-typed value that indicates how
        ///                 valuable data is.
        ///  * `flags` - Specifies the [`DXGI_OFFER_RESOURCE_FLAGS`].
        ///
        /// # Return Value
        /// This method returns an [`HRESULT`] success or error code, which can include
        /// [`E_INVALIDARG`] if a resource in the array, or the `priority`, is invalid.
        ///
        /// # Remarks
        /// [`IDXGIDevice4::offset_resources1`] (an extension of the original
        /// [`IDXGIDevice2::offer_resources`] API) enables D3D based applications to allow
        /// de-committing of an allocation’s backing store to reduce system commit under low memory
        /// conditions. A de-committed allocation cannot be reused, so opting in to the new
        /// [`DXGI_OFFER_RESOURCE_FLAGS::AllowDecommit`] flag means the new reclaim results must be
        /// properly handled.
        ///
        /// [`IDXGIDevice4::offset_resources1`] and [`IDXGIDevice4::reclaim_resources1`] may not be
        /// used interchangeably with [`IDXGIDevice2::offer_resources`] and
        /// [`IDXGIDevice2::reclaim_resources`].
        ///
        /// The priority value that the `priority` parameter specifies describes how valuable the
        /// caller considers the content to be. The operating system uses the priority value to
        /// discard resources in order of priority. The operating system discards a resource that
        /// is offered with low priority before it discards a resource that is offered with a
        /// higher priority.
        ///
        /// If you call [`IDXGIDevice4::offset_resources1`] to offer a resource while the resource
        /// is bound to the pipeline, the resource is unbound. You cannot call
        /// [`IDXGIDevice4::offset_resources1`] on a resource that is mapped. After you offer a
        /// resource, the resource cannot be mapped or bound to the pipeline until you call the
        /// [`IDXGIDevice4::reclaim_resources1`] method to reclaim the resource. You cannot call
        /// [`IDXGIDevice4::offset_resources1`] to offer immutable resources.
        ///
        /// To offer shared resources, call [`IDXGIDevice4::offset_resources1`] on only one of the
        /// sharing devices. To ensure exclusive access to the resources, you must use an
        /// [`IDXGIKeyedMutex`] object and then call [`IDXGIDevice4::offset_resources1`] only while
        /// you hold the mutex. In fact, you can't offer shared resources unless you use
        /// [`IDXGIKeyedMutex`] because offering shared resources without using [`IDXGIKeyedMutex`]
        /// isn't supported.
        ///
        /// The user mode display driver might not immediately offer the resources that you
        /// specified in a call to [`IDXGIDevice4::offset_resources1`]. The driver can postpone
        /// offering them until the next call to [`IDXGISwapChain::present`],
        /// [`IDXGISwapChain1::present1`], or [`ID3D11DeviceContext::flush`].
        fn offer_resources1(
            &mut self,
            num_resources: UINT,
            resources: *const *mut IDXGIResource,
            priority: DXGI_OFFER_RESOURCE_PRIORITY,
            flags: UINT
        ) -> HRESULT;

        /// Restores access to resources that were previously offered by calling
        /// [`IDXGIDevice4::offset_resources1`].
        ///
        /// # Parameters
        ///  * `num_resources` - The number of resources in the `resources` argument and `results`
        ///                      argument arrays.
        ///  * `resources` - An array of pointers to [`IDXGIResource`] interfaces for the resources
        ///                  to reclaim.
        ///  * `results` - A pointer to an array that receives [`DXGI_RECLAIM_RESOURCE_RESULTS`]
        ///                values. Each value in the array corresponds to a resource at the same
        ///                index that the `resources` parameter specifies. The caller can pass in
        ///                [`null_mut`], if the caller intends to fill the resources with new
        ///                content regardless of whether the old content was discarded.
        ///
        /// # Return Value
        /// This method returns an [`HRESULT`] success or error code, including [`E_INVALIDARG`] if
        /// the resources are invalid.
        ///
        /// # Remarks
        /// After you call [`IDXGIDevice4::offset_resources1`] to offer one or more resources, you
        /// must call [`IDXGIDevice4::reclaim_resources1`] before you can use those resources
        /// again.
        ///
        /// To reclaim shared resources, call [`IDXGIDevice4::reclaim_resources1`] only on one of
        /// the sharing devices. To ensure exclusive access to the resources, you must use an
        /// [`IDXGIKeyedMutex`] object and then call [`IDXGIDevice4::reclaim_resources1`] only
        /// while you hold the mutex.
        fn reclaim_resources1(
            &mut self,
            num_resources: UINT,
            resources: *const *mut IDXGIResource,
            results: *mut DXGI_RECLAIM_RESOURCE_RESULTS
        ) -> HRESULT;
    }
);
