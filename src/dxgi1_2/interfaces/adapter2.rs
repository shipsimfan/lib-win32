use crate::{
    com_interface,
    dxgi::{
        IDXGIAdapter, IDXGIAdapter1, IDXGIAdapter1Trait, IDXGIAdapterTrait, IDXGIObject,
        IDXGIObjectTrait,
    },
    dxgi1_2::DXGI_ADAPTER_DESC2,
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGIDevice, IDXGIFactory, IDXGIFactory1},
    E_INVALIDARG, S_OK,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// The [`IDXGIAdapter2`] interface represents a display subsystem, which includes one or more
    /// GPUs, DACs, and video memory.
    ///
    /// # Remarks
    /// A display subsystem is often referred to as a video card; however, on some computers, the
    /// display subsystem is part of the motherboard.
    ///
    /// To enumerate the display subsystems, use [`IDXGIFactory1::enum_adapters1`].
    ///
    /// To get an interface to the adapter for a particular device, use
    /// [`IDXGIDevice::get_adapter`].
    ///
    /// To create a software adapter, use [`IDXGIFactory::create_software_adapter`].
    pub abstract IDXGIAdapter2(IDXGIAdapter2VTable/IDXGIAdapter2Trait):
        IDXGIAdapter1/IDXGIAdapter1Trait(adapter1) +
        IDXGIAdapter/IDXGIAdapterTrait(adapter1.adapter) +
        IDXGIObject/IDXGIObjectTrait(adapter1.adapter.object) +
        IUnknown/IUnknownTrait(adapter1.adapter.object.unknown) {
        const IID = 0x0AA1AE0A-0xFA0E-0x4B84-0x8644-0xE05FF8E5ACB5;

        /// Gets a Microsoft DirectX Graphics Infrastructure (DXGI) 1.2 description of an adapter
        /// or video card. This description includes information about the granularity at which the
        /// graphics processing unit (GPU) can be preempted from performing its current task.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`DXGI_ADAPTER_DESC2`] structure that describes the adapter.
        ///             This parameter must not be [`null_mut`]. On feature level 9 graphics
        ///             hardware, earlier versions of [`IDXGIAdapter2::get_desc2`]
        ///             ([`IDXGIAdapter::get_desc`] and [`IDXGIAdapter1::get_desc1`]) return zeros
        ///             for `vendor_id`, `device_id`, `sub_sys_id`, and `revision` members of the
        ///             adapter description structure and “Software Adapter” for the description
        ///             string in the `description` member. [`IDXGIAdapter2::get_desc2`] returns
        ///             the actual feature level 9 hardware values in these members.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns [`E_INVALIDARG`] if the `desc`
        /// parameter is [`null_mut`].
        ///
        /// # Remarks
        /// Use the [`IDXGIAdapter2::get_desc2`] method to get a DXGI 1.2 description of an
        /// adapter. To get a DXGI 1.1 description, use the [`IDXGIAdapter1::get_desc1`] method. To
        /// get a DXGI 1.0 description, use the [`IDXGIAdapter::get_desc`] method.
        ///
        /// The Windows Display Driver Model (WDDM) scheduler can preempt the GPU's execution of
        /// application tasks. The granularity at which the GPU can be preempted from performing
        /// its current task in the WDDM 1.1 or earlier driver model is a direct memory access
        /// (DMA) buffer for graphics tasks or a compute packet for compute tasks. The GPU can
        /// switch between tasks only after it completes the currently executing unit of work, a
        /// DMA buffer or a compute packet.
        ///
        /// A DMA buffer is the largest independent unit of graphics work that the WDDM scheduler
        /// can submit to the GPU. This buffer contains a set of GPU instructions that the WDDM
        /// driver and GPU use. A compute packet is the largest independent unit of compute work
        /// that the WDDM scheduler can submit to the GPU. A compute packet contains dispatches
        /// (for example, calls to the [`ID3D11DeviceContext::dispatch`] method), which contain
        /// thread groups. The WDDM 1.2 or later driver model allows the GPU to be preempted at
        /// finer granularity levels than a DMA buffer or compute packet. You can use the
        /// [`IDXGIAdapter2::get_desc2`] method to retrieve the granularity levels for graphics and
        /// compute tasks.
        fn get_desc2(&mut self, desc: *mut DXGI_ADAPTER_DESC2) -> HRESULT;
    }
);
