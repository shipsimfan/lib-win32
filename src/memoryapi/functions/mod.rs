mod virtual_alloc;
mod virtual_alloc_ex;
mod virtual_alloc_ex_numa;
mod virtual_free;
mod virtual_free_ex;
mod virtual_protect;
mod virtual_protect_ex;

pub use virtual_alloc::VirtualAlloc;
pub use virtual_alloc_ex::VirtualAllocEx;
pub use virtual_alloc_ex_numa::VirtualAllocExNuma;
pub use virtual_free::VirtualFree;
pub use virtual_free_ex::VirtualFreeEx;
pub use virtual_protect::VirtualProtect;
pub use virtual_protect_ex::VirtualProtectEx;
