mod virtual_alloc;
mod virtual_protect;
mod virtual_alloc_ex;

pub use virtual_alloc_ex::VirtualAllocEx;
pub use virtual_alloc::VirtualAlloc;
pub use virtual_protect::VirtualProtect;
