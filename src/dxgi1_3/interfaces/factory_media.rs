use crate::{
    com_interface,
    dxgi::{IDXGIOutput, IDXGIResource},
    dxgi1_2::{IDXGISwapChain1, DXGI_SWAP_CHAIN_DESC1},
    dxgi1_3::{IDXGIDecodeSwapChain, DXGI_DECODE_SWAP_CHAIN_DESC},
    unknwn::{IUnknown, IUnknownTrait},
    HANDLE, HRESULT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::D3D11_BIND_FLAG,
    dxgi::{
        CreateDXGIFactory, CreateDXGIFactory1, IDXGIDevice, IDXGIDevice1, IDXGIObject,
        DXGI_PRESENT_RESTRICT_TO_OUTPUT,
    },
    dxgi1_2::IDXGIDevice2,
    dxgi1_3::IDXGIDevice3,
    DXGI_ERROR_INVALID_CALL, E_OUTOFMEMORY, S_OK,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

com_interface!(
    /// Creates swap chains for desktop media apps that use DirectComposition surfaces to decode
    /// and display video.
    ///
    /// # Remarks
    /// To create a Microsoft DirectX Graphics Infrastructure (DXGI) media factory interface, pass
    /// [`IDXGIFactoryMedia`] into either the [`CreateDXGIFactory`] or [`CreateDXGIFactory1`]
    /// function or call [`IUnknown::query_interface`] from a factory object returned by
    /// [`CreateDXGIFactory`], [`CreateDXGIFactory1`], or [`CreateDXGIFactory2`].
    ///
    /// Because you can create a Direct3D device without creating a swap chain, you might need to
    /// retrieve the factory that is used to create the device in order to create a swap chain. You
    /// can request the [`IDXGIDevice`], [`IDXGIDevice1`], [`IDXGIDevice2`], or [`IDXGIDevice3`]
    /// interface from the Direct3D device and then use the [`IDXGIObject::get_parent`] method to
    /// locate the factory.
    pub abstract IDXGIFactoryMedia(IDXGIFactoryMediaVTable/IDXGIFactoryMediaTrait):
        IUnknown/IUnknownTrait(unknown) {
        const IID = 0x41E7D1F2-0xA591-0x4F7B-0xA2E5-0xFA9C843E1C12;

        /// Creates a YUV swap chain for an existing DirectComposition surface handle.
        ///
        /// # Parameters
        ///  * `device` - A pointer to the Direct3D device for the swap chain. This parameter
        ///               cannot be [`null_mut`]. Software drivers, like
        ///               [`D3D_DRIVER_TYPE_REFERENCE`], are not supported for composition swap
        ///               chains.
        ///  * `surface` - A handle to an existing DirectComposition surface. This parameter cannot
        ///                be [`null_mut`].
        ///  * `desc` - A pointer to a [`DXGI_SWAP_CHAIN_DESC1`] structure for the swap-chain
        ///             description. This parameter cannot be [`null`].
        ///  * `restrict_to_output` - A pointer to the [`IDXGIOutput`] interface for the swap chain
        ///                           to restrict content to. If the swap chain is moved to a
        ///                           different output, the content is black. You can optionally
        ///                           set this parameter to an output target that uses
        ///                           [`DXGI_PRESENT_RESTRICT_TO_OUTPUT`] to restrict the content
        ///                           on this output. If the swap chain is moved to a different0
        ///                           output, the content is black. You must also pass the
        ///                           [`DXGI_PRESENT_RESTRICT_TO_OUTPUT`] flag in a present call to
        ///                           force the content to appear blacked out on any other output.
        ///                           If you want to restrict the content to a different output,
        ///                           you must create a new swap chain. However, you can
        ///                           conditionally restrict content based on the
        ///                           [`DXGI_PRESENT_RESTRICT_TO_OUTPUT`] flag. Set this parameter
        ///                           to [`null_mut`] if you don't want to restrict content to an
        ///                           output target.
        ///  * `swap_chain` - A pointer to a variable that receives a pointer to the
        ///                   [`IDXGISwapChain1`] interface for the swap chain that this method
        ///                   creates.
        ///
        /// # Return Value
        /// [`IDXGIFactoryMedia::create_swap_chain_for_composition_surface_handle`] returns:
        ///  - [`S_OK`] if it successfully created a swap chain.
        ///  - [`E_OUTOFMEMORY`] if memory is unavailable to complete the operation.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the calling application provided invalid data, for
        ///    example, if `desc`, or `swap_chain` is [`null`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic that are
        ///    defined by the type of device that you pass to `device`.
        fn create_swap_chain_for_composition_surface_handle(
            &mut self,
            device: *mut IUnknown,
            surface: HANDLE,
            desc: *const DXGI_SWAP_CHAIN_DESC1,
            restrict_to_output: *mut IDXGIOutput,
            swap_chain: *mut *mut IDXGISwapChain1
        ) -> HRESULT;

        /// Creates a YUV swap chain for an existing DirectComposition surface handle. The swap
        /// chain is created with pre-existing buffers and very few descriptive elements are
        /// required. Instead, this method requires a DirectComposition surface handle and an
        /// [`IDXGIResource`] buffer to hold decoded frame data. The swap chain format is
        /// determined by the format of the subresources of the [`IDXGIResource`].
        ///
        /// # Parameters
        ///  * `device` - A pointer to the Direct3D device for the swap chain. This parameter
        ///               cannot be [`null_mut`]. Software drivers, like
        ///               [`D3D_DRIVER_TYPE_REFERENCE`], are not supported for composition swap
        ///               chains.
        ///  * `surface` - A handle to an existing DirectComposition surface. This parameter cannot
        ///                be [`null_mut`].
        ///  * `desc` - A pointer to a [`DXGI_DECODE_SWAP_CHAIN_DESC`] structure for the swap-chain
        ///             description. This parameter cannot be [`null_mut`].
        ///  * `yuv_decode_buffers` - A pointer to a [`IDXGIResource`] interface that represents
        ///                           the resource that contains the info that
        ///          [`IDXGIFactoryMedia::create_decode_swap_chain_for_composition_surface_handle`]
        ///                           decodes.
        ///  * `restrict_to_output` - A pointer to the [`IDXGIOutput`] interface for the swap chain
        ///                           to restrict content to. If the swap chain is moved to a
        ///                           different output, the content is black. You can optionally
        ///                           set this parameter to an output target that uses
        ///                           [`DXGI_PRESENT_RESTRICT_TO_OUTPUT`] to restrict the content
        ///                           on this output. If the swap chain is moved to a different
        ///                           output, the content is black. You must also pass the
        ///                           [`DXGI_PRESENT_RESTRICT_TO_OUTPUT`] flag in a present call to
        ///                           force the content to appear blacked out on any other output.
        ///                           If you want to restrict the content to a different output,
        ///                           you must create a new swap chain. However, you can
        ///                           conditionally restrict content based on the
        ///                           [`DXGI_PRESENT_RESTRICT_TO_OUTPUT`] flag. Set this parameter
        ///                           to [`null_mut`] if you don't want to restrict content to an
        ///                           output target.
        ///  * `swap_chain` - A pointer to a variable that receives a pointer to the
        ///                   [`IDXGIDecodeSwapChain`] interface for the swap chain that this
        ///                   method creates.
        ///
        /// # Return Value
        /// [`IDXGIFactoryMedia::create_decode_swap_chain_for_composition_surface_handle`] returns:
        ///  - [`S_OK`] if it successfully created a swap chain.
        ///  - [`E_OUTOFMEMORY`] if memory is unavailable to complete the operation.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the calling application provided invalid data, for
        ///    example, if `desc`, `yuv_decode_buffers`, or `swap_chain` is [`null_mut`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic that are
        ///    defined by the type of device that you pass to `device`.
        ///
        /// # Remarks
        /// The [`IDXGIResource`] provided via the `yuv_decode_buffers` parameter must point to at
        /// least one subresource, and all subresources must be created with the
        /// [`D3D11_BIND_FLAG::Decoder`] flag.
        fn create_decode_swap_chain_for_composition_surface_handle(
            &mut self,
            device: *mut IUnknown,
            surface: HANDLE,
            desc: *mut DXGI_DECODE_SWAP_CHAIN_DESC,
            yuv_decode_buffers: *mut IDXGIResource,
            restrict_to_output: *mut IDXGIOutput,
            swap_chain: *mut *mut IDXGIDecodeSwapChain
        ) -> HRESULT;
    }
);
