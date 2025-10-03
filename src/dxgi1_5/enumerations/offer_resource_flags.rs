/// Specifies flags for the [`IDXGIDevice4::offer_resources1`] method.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_OFFER_RESOURCE_FLAGS {
    /// Indicates the ability to allow memory de-commit by the DirectX Graphics Kernel.
    Decommit = 0x1,
}
