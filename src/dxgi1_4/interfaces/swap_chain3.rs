use crate::{
    com_interface,
    dxgi::{
        IDXGIDeviceSubObject, IDXGIDeviceSubObjectTrait, IDXGIObject, IDXGIObjectTrait,
        IDXGISwapChain, IDXGISwapChainTrait, DXGI_COLOR_SPACE_TYPE, DXGI_FORMAT,
    },
    dxgi1_2::{IDXGISwapChain1, IDXGISwapChain1Trait},
    dxgi1_3::{IDXGISwapChain2, IDXGISwapChain2Trait},
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi::DXGI_SWAP_CHAIN_FLAG, dxgi1_2::IDXGIFactory2, S_OK};

com_interface!(
    /// Extends [`IDXGISwapChain2`] with methods to support getting the index of the swap chain's
    /// current back buffer and support for color space.
    pub abstract IDXGISwapChain3(IDXGISwapChain3VTable/IDXGISwapChain3Trait):
        IDXGISwapChain2/IDXGISwapChain2Trait(swap_chain2) +
        IDXGISwapChain1/IDXGISwapChain1Trait(swap_chain2.swap_chain1) +
        IDXGISwapChain/IDXGISwapChainTrait(swap_chain2.swap_chain1.swap_chain) +
        IDXGIDeviceSubObject/IDXGIDeviceSubObjectTrait(swap_chain2.swap_chain1.swap_chain.device_sub_object) +
        IDXGIObject/IDXGIObjectTrait(swap_chain2.swap_chain1.swap_chain.device_sub_object.object) +
        IUnknown/IUnknownTrait(swap_chain2.swap_chain1.swap_chain.device_sub_object.object.unknown) {
        const IID = 0x94D99BDB-0xF1F8-0x4AB0-0xB236-0x7DA0170EDAB1;

        /// Gets the index of the swap chain's current back buffer.
        ///
        /// # Return Value
        /// Returns the index of the current back buffer.
        fn get_current_back_buffer_index(&mut self) -> UINT;

        /// Checks whether the swap chain currently supports the specified color space, based on
        /// the current adapter output (for example, what monitor the swap chain window is being
        /// displayed on).
        ///
        /// While a color space has been successfully set to the swap chain (whether or not it was
        /// returned as supported before), it will be returned as supported when queried with this
        /// function.
        ///
        /// # Parameters
        ///  * `color_space` - A [`DXGI_COLOR_SPACE_TYPE`]-typed value that specifies color space
        ///                    type to check support for.
        ///  * `color_space_support` - A pointer to a variable that receives a combination of
        ///                            [`DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG`]-typed values
        ///                            that are combined by using a bitwise OR operation. The
        ///                            resulting value specifies options for color space support.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or it returns one of the error codes that are
        /// described in the `DXGI_ERROR` topic.
        fn check_color_space_support(
            &mut self,
            color_space: DXGI_COLOR_SPACE_TYPE,
            color_space_support: *mut UINT
        ) -> HRESULT;

        /// Sets the color space used by the swap chain.
        ///
        /// # Parameters
        ///  * `color_space` - A [`DXGI_COLOR_SPACE_TYPE`]-typed value that specifies the color
        ///                    space to set.
        ///
        /// # Return Value
        /// This method returns [`S_OK`] on success, or it returns one of the error codes that are
        /// described in the `DXGI_ERROR` topic.
        fn set_color_space1(&mut self, color_space: DXGI_COLOR_SPACE_TYPE) -> HRESULT;

        /// Changes the swap chain's back buffer size, format, and number of buffers, where the
        /// swap chain was created using a D3D12 command queue as an input device. This should be
        /// called when the application window is resized.
        ///
        /// # Parameters
        ///  * `buffer_count` - The number of buffers in the swap chain (including all back and
        ///                     front buffers). This number can be different from the number of
        ///                     buffers with which you created the swap chain. This number can't be
        ///                     greater than [`DXGI_MAX_SWAP_CHAIN_BUFFERS`]. Set this number to
        ///                     zero to preserve the existing number of buffers in the swap chain.
        ///                     You can't specify less than two buffers for the flip presentation
        ///                     model.
        ///  * `width` - The new width of the back buffer. If you specify zero, DXGI will use the
        ///              width of the client area of the target window. You can't specify the width
        ///              as zero if you called the
        ///              [`IDXGIFactory2::create_swap_chain_for_composition`] method to create the
        ///              swap chain for a composition surface.
        ///  * `height` - The new height of the back buffer. If you specify zero, DXGI will use the
        ///               height of the client area of the target window. You can't specify the
        ///               height as zero if you called the
        ///               [`IDXGIFactory2::create_swap_chain_for_composition`] method to create the
        ///               swap chain for a composition surface.
        ///  * `format` - A [`DXGI_FORMAT`]-typed value for the new format of the back buffer. Set
        ///               this value to [`DXGI_FORMAT::Unknown`] to preserve the existing format of
        ///               the back buffer. The flip presentation model supports a more restricted
        ///               set of formats than the bit-block transfer (bitblt) model.
        ///  * `swap_chain_flags` - A combination of [`DXGI_SWAP_CHAIN_FLAG`]-typed values that are
        ///                         combined by using a bitwise OR operation. The resulting value
        ///                         specifies options for swap-chain behavior.
        ///  * `creation_node_mask` - An array of [`UINT`]s, of total size `buffer_count`, where
        ///                           the value indicates which node the back buffer should be
        ///                           created on. Buffers created using
        ///                           [`IDXGISwapChain3::resize_buffers1`] with a non-null
        ///                           `creation_node_mask` array are visible to all nodes.
        ///  * `present_queue` - An array of command queues ([`ID3D12CommandQueue`] instances), of
        ///                      total size `buffer_count`. Each queue provided must match the
        ///                      corresponding creation node mask specified in the
        ///                      `creation_node_mask` array. When [`IDXGISwapChain::Present`] is
        ///                      called, in addition to rotating to the next buffer for the next
        ///                      frame, the swapchain will also rotate through these command
        ///                      queues. This allows the app to control which queue requires
        ///                      synchronization for a given present operation.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// # Remarks
        /// This method is only valid to call when the swapchain was created using a D3D12 command
        /// queue ([`ID3D12CommandQueue`]) as an input device.
        ///
        /// When a swapchain is created on a multi-GPU adapter, the backbuffers are all created on
        /// node 1 and only a single command queue is supported.
        /// [`IDXGISwapChain3::resize_buffers1`] enables applications to create backbuffers on
        /// different nodes, allowing a different command queue to be used with each node. These
        /// capabilities enable Alternate Frame Rendering (AFR) techniques to be used with the
        /// swapchain.
        ///
        /// Also see the Remarks section in [`IDXGISwapChain::resize_buffers`], all of which is
        /// relevant to [`IDXGISwapChain3::resize_buffers1`].
        fn resize_buffers1(
            &mut self,
            buffer_count: UINT,
            width: UINT,
            height: UINT,
            format: DXGI_FORMAT,
            swap_chain_flags: UINT,
            creation_node_mask: *const UINT,
            present_queue: *const *mut IUnknown
        ) -> HRESULT;
    }
);
