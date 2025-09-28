use crate::UINT;

/// Prevent DXGI from monitoring an applications message queue; this makes DXGI unable to respond
/// to mode changes.
pub const DXGI_MWA_NO_WINDOW_CHANGES: UINT = 1 << 0;

/// Prevent DXGI from responding to an alt-enter sequence.
pub const DXGI_MWA_NO_ALT_ENTER: UINT = 1 << 1;

/// Prevent DXGI from responding to a print-screen key.
pub const DXGI_MWA_NO_PRINT_SCREEN: UINT = 1 << 2;

#[allow(missing_docs)]
pub const DXGI_MWA_VALID: UINT = 0x7;
