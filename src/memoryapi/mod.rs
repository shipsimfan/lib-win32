//! This header is used by System Services

mod constants;
mod functions;

pub use constants::*;
pub use functions::{
    CreateFileMapping, CreateFileMappingW, GetLargePageMinimum, MapViewOfFile3, VirtualAlloc,
    VirtualAlloc2, VirtualAlloc2FromApp, VirtualAllocEx, VirtualAllocExNuma, VirtualFree,
    VirtualFreeEx, VirtualProtect, VirtualProtectEx,
};
