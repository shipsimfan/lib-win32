mod virtual_alloc;
mod virtual_alloc_ex;
mod virtual_alloc_ex_numa;
mod virtual_protect;

pub use virtual_alloc::VirtualAlloc;
pub use virtual_alloc_ex::VirtualAllocEx;
pub use virtual_alloc_ex_numa::VirtualAllocExNuma;
pub use virtual_protect::VirtualProtect;
