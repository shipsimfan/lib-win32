//! This header is used by System Services

mod constants;
mod functions;

pub use constants::*;
pub use functions::{
    VirtualAlloc, VirtualAlloc2, VirtualAlloc2FromApp, VirtualAllocEx, VirtualAllocExNuma,
    VirtualFree, VirtualFreeEx, VirtualProtect, VirtualProtectEx,
};
