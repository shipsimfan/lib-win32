// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Device;

/// Unordered resource support options for a compute shader resource (see
/// [`ID3D11Device::check_feature_support`]).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_FORMAT_SUPPORT2 {
    /// Format supports atomic add.
    UavAtomicAdd = 0x1,

    /// Format supports atomic bitwise operations.
    UavAtomicBitwiseOps = 0x2,

    /// Format supports atomic compare with store or exchange.
    UavAtomicCompareStoreOrCompareExchange = 0x4,

    /// Format supports atomic exchange.
    UavAtomicExchange = 0x8,

    /// Format supports atomic min and max.
    UavAtomicSignedMinOrMax = 0x10,

    /// Format supports atomic unsigned min and max.
    UavAtomicUnsignedMinOrMax = 0x20,

    /// Format supports a typed load.
    UavTypedLoad = 0x40,

    /// Format supports a typed store.
    UavTypedStore = 0x80,

    /// Format supports logic operations in blend state.
    OutputMergerLogicOp = 0x100,

    /// Format supports tiled resources.
    Tiled = 0x200,

    /// Format supports shareable resources.
    Shareable = 0x400,

    /// Format supports multi-plane overlays.
    MultiplaneOverlay = 0x4000,
}
