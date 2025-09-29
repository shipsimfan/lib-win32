use crate::{
    com_interface,
    dxgi::{
        IDXGIFactory, IDXGIFactory1, IDXGIFactory1Trait, IDXGIFactoryTrait, IDXGIObject,
        IDXGIObjectTrait, IDXGIOutput,
    },
    dxgi1_2::{IDXGISwapChain1, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FULLSCREEN_DESC},
    unknwn::{IUnknown, IUnknownTrait},
    BOOL, DWORD, HANDLE, HRESULT, HWND, LUID, UINT,
};

com_interface!(
    pub abstract IDXGIFactory2(IDXGIFactory2VTable/IDXGIFactory2Trait):
        IDXGIFactory1/IDXGIFactory1Trait(factory1) +
        IDXGIFactory/IDXGIFactoryTrait(factory1.factory) +
        IDXGIObject/IDXGIObjectTrait(factory1.factory.object) +
        IUnknown/IUnknownTrait(factory1.factory.object.unknown) {
        const IID = 0x50C83A1C-0xE072-0x4C48-0x87B0-0x3630FA36A6D0;

        fn is_window_stereo_enabled(&mut self) -> BOOL;

        fn create_swap_chain_for_hwnd(
            &mut self,
            device: *mut IUnknown,
            wnd: HWND,
            desc: *const DXGI_SWAP_CHAIN_DESC1,
            fullscreen_desc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
            restrict_to_output: *mut IDXGIOutput,
            swap_chain: *mut *mut IDXGISwapChain1
        ) -> HRESULT;

        fn create_swap_chain_for_core_window(
            &mut self,
            device: *mut IUnknown,
            window: *mut IUnknown,
            desc: *const DXGI_SWAP_CHAIN_DESC1,
            restrict_to_output: *mut IDXGIOutput,
            swap_chain: *mut *mut IDXGISwapChain1
        ) -> HRESULT;

        fn get_shared_resource_adapter_luid(
            &mut self,
            resource: HANDLE,
            luid: *mut LUID
        ) -> HRESULT;

        fn register_stereo_status_window(
            &mut self,
            window_handle: HWND,
            msg: UINT,
            cookie: *mut DWORD
        ) -> HRESULT;

        fn register_stereo_status_event(&mut self, event: HANDLE, cookie: *mut DWORD) -> HRESULT;

        fn unregister_stereo_status(&mut self, cookie: DWORD);

        fn register_occlusion_status_window(
            &mut self,
            window_handle: HWND,
            msg: UINT,
            cookie: *mut DWORD
        ) -> HRESULT;

        fn register_occlusion_status_event(
            &mut self,
            event: HANDLE,
            cookie: *mut DWORD
        ) -> HRESULT;

        fn unregister_occlusion_status(&mut self, cookie: DWORD);

        fn create_swap_chain_for_composition(
            &mut self,
            device: *mut IUnknown,
            desc: *const DXGI_SWAP_CHAIN_DESC1,
            restrict_to_output: *mut IDXGIOutput,
            swap_chain: *mut *mut IDXGISwapChain1
        ) -> HRESULT;
    }
);
