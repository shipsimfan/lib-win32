//! Microsoft DirectX Graphics Infrastructure (DXGI) handles enumerating graphics adapters,
//! enumerating display modes, selecting buffer formats, sharing resources between processes (such
//! as, between applications and the Desktop Window Manager (DWM)), and presenting rendered frames
//! to a window or monitor for display.
//!
//! DXGI is used by Direct3D 10, Direct3D 11 and Direct3D 12.
//!
//! Though most graphics programming is done using Direct3D, you can use DXGI to present frames to
//! a window, monitor, or other graphics component for eventual composition and display. You can
//! also use DXGI to read the contents on a monitor.

mod constants;
mod enumerations;
mod functions;
mod interfaces;
mod structures;
mod types;

pub use constants::*;
pub use enumerations::*;
pub use functions::*;
pub use interfaces::*;
pub use structures::*;
pub use types::*;
