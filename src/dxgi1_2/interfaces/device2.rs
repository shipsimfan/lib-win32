use crate::{
    com_interface,
    dxgi::{
        IDXGIDevice, IDXGIDevice1, IDXGIDevice1Trait, IDXGIDeviceTrait, IDXGIObject,
        IDXGIObjectTrait, IDXGIResource,
    },
    dxgi1_2::DXGI_OFFER_RESOURCE_PRIORITY,
    unknwn::{IUnknown, IUnknownTrait},
    BOOL, HANDLE, HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGIKeyedMutex, IDXGISwapChain},
    CreateEvent, SetEvent, WaitForSingleObject, E_INVALIDARG, E_OUTOFMEMORY, FALSE, S_OK, TRUE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// The [`IDXGIDevice2`] interface implements a derived class for DXGI objects that produce
    /// image data. The interface exposes methods to block CPU processing until the GPU completes
    /// processing, and to offer resources to the operating system.
    ///
    /// # Remarks
    /// The [`IDXGIDevice2`] interface is designed for use by DXGI objects that need access to
    /// other DXGI objects. This interface is useful to applications that do not use Direct3D to
    /// communicate with DXGI.
    ///
    /// The Direct3D create device functions return a Direct3D device object. This Direct3D device
    /// object implements the [`IUnknown`] interface. You can query this Direct3D device object for
    /// the device's corresponding [`IDXGIDevice2`] interface.
    pub abstract IDXGIDevice2(IDXGIDevice2VTable/IDXGIDevice2Trait):
        IDXGIDevice1/IDXGIDevice1Trait(device1) +
        IDXGIDevice/IDXGIDeviceTrait(device1.device) +
        IDXGIObject/IDXGIObjectTrait(device1.device.object) +
        IUnknown/IUnknownTrait(device1.device.object.unknown) {
        const IID = 0x05008617-0xFBFD-0x4051-0xA790-0x144884B4F6A9;

        /// Allows the operating system to free the video memory of resources by discarding their
        /// content.
        ///
        /// # Parameters
        ///  * `num_resources` - The number of resources in the ppResources argument array.
        ///  * `resources` - An array of pointers to [`IDXGIResource`] interfaces for the resources
        ///                  to offer.
        ///  * `priority` - A [`DXGI_OFFER_RESOURCE_PRIORITY`]-typed value that indicates how
        ///                 valuable data is.
        ///
        /// # Return Value
        /// [`IDXGIDevice2::offer_resources`] returns:
        ///  - [`S_OK`] if resources were successfully offered
        ///  - [`E_INVALIDARG`] if a resource in the array or the priority is invalid
        ///
        /// # Remarks
        /// The priority value that the `priority` parameter specifies describes how valuable the
        /// caller considers the content to be. The operating system uses the priority value to
        /// discard resources in order of priority. The operating system discards a resource that
        /// is offered with low priority before it discards a resource that is offered with a
        /// higher priority.
        ///
        /// If you call [`IDXGIDevice2::offer_resources`] to offer a resource while the resource is
        /// bound to the pipeline, the resource is unbound. You cannot call
        /// [`IDXGIDevice2::offer_resources`] on a resource that is mapped. After you offer a
        /// resource, the resource cannot be mapped or bound to the pipeline until you call the
        /// [`IDXGIDevice2::reclaim_resource`] method to reclaim the resource. You cannot call
        /// [`IDXGIDevice2::offer_resources`] to offer immutable resources.
        ///
        /// To offer shared resources, call [`IDXGIDevice2::offer_resources`] on only one of the
        /// sharing devices. To ensure exclusive access to the resources, you must use an
        /// [`IDXGIKeyedMutex`] object and then call [`IDXGIDevice2::offer_resources`] only while
        /// you hold the mutex. In fact, you can't offer shared resources unless you use
        /// [`IDXGIKeyedMutex`] because offering shared resources without using [`IDXGIKeyedMutex`]
        /// isn't supported.
        fn offer_resources(
            &mut self,
            num_resources: UINT,
            resources: *const *mut IDXGIResource,
            priority: DXGI_OFFER_RESOURCE_PRIORITY
        ) -> HRESULT;

        /// Restores access to resources that were previously offered by calling
        /// [`IDXGIDevice2::offer_resources`].
        ///
        /// # Parameters
        ///  * `num_resources` - The number of resources in the `resources` argument and
        ///                      `discarded` argument arrays.
        ///  * `resources` - An array of pointers to [`IDXGIResource`] interfaces for the resources
        ///                  to reclaim.
        ///  * `discarded` - A pointer to an array that receives Boolean values. Each value in the
        ///                  array corresponds to a resource at the same index that the `resources`
        ///                  parameter specifies. The runtime sets each Boolean value to [`TRUE`]
        ///                  if the corresponding resource’s content was discarded and is now
        ///                  undefined, or to [`FALSE`] if the corresponding resource’s old content
        ///                  is still intact. The caller can pass in [`null_mut`], if the caller
        ///                  intends to fill the resources with new content regardless of whether
        ///                  the old content was discarded.
        ///
        /// # Return Value
        /// [`IDXGIDevice2::reclaim_resources`] returns:
        ///  - [`S_OK`] if resources were successfully reclaimed
        ///  - [`E_INVALIDARG`] if the resources are invalid
        ///
        /// # Remarks
        /// After you call [`IDXGIDevice2::offer_resources`] to offer one or more resources, you
        /// must call [`IDXGIDevice2::offer_resources`] before you can use those resources again.
        /// You must check the values in the array at `discarded` to determine whether each
        /// resource’s content was discarded. If a resource’s content was discarded while it was
        /// offered, its current content is undefined. Therefore, you must overwrite the resource’s
        /// content before you use the resource.
        ///
        /// To reclaim shared resources, call [`IDXGIDevice2::offer_resources`] only on one of the
        /// sharing devices. To ensure exclusive access to the resources, you must use an
        /// [`IDXGIKeyedMutex`] object and then call [`IDXGIDevice2::offer_resources`] only while
        /// you hold the mutex.
        fn reclaim_resources(
            &mut self,
            num_resources: UINT,
            resources: *const *mut IDXGIResource,
            discarded: *mut BOOL
        ) -> HRESULT;

        /// Flushes any outstanding rendering commands and sets the specified event object to the
        /// signaled state after all previously submitted rendering commands complete.
        ///
        /// # Parameters
        ///  * `event` - A handle to the event object. The [`CreateEvent`] or [`OpenEvent`]
        ///              function returns this handle. All types of event objects (manual-reset,
        ///              auto-reset, and so on) are supported. The handle must have the
        ///              `EVENT_MODIFY_STATE` access right.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the following values:
        ///  - [`E_OUTOFMEMORY`] if insufficient memory is available to complete the operation.
        ///  - [`E_INVALIDARG`] if the parameter was validated and determined to be incorrect.
        ///
        /// # Remarks
        /// [`IDXGIDevice2::enqueue_set_event`] calls the [`SetEvent`] function on the event object
        /// after all previously submitted rendering commands complete or the device is removed.
        ///
        /// After an application calls [`IDXGIDevice2::enqueue_set_event`], it can immediately call
        /// the [`WaitForSingleObject`] function to put itself to sleep until rendering commands
        /// complete.
        ///
        /// You cannot use [`IDXGIDevice2::enqueue_set_event`] to determine work completion that is
        /// associated with presentation ([`IDXGISwapChain::present`]); instead, we recommend that
        /// you use [`IDXGISwapChain::get_frame_statistics`].
        fn enqueue_set_event(&mut self, event: HANDLE) -> HRESULT;
    }
);
