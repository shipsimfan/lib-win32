use crate::raw::PAddrInfoW;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{AddrInfoW, GetAddrInfoW};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Ws2_32")]
extern "C" {
    /// # FreeAddrInfoW function (ws2tcpip.h)
    ///
    /// The [`FreeAddrInfoW`] function frees address information that the
    /// [`GetAddrInfoW`] function dynamically allocates in [`AddrInfoW`]
    /// structures.
    ///
    /// ## Parameters
    /// `addr_info`\
    /// A pointer to the [`AddrInfoW`] structure or linked list of
    /// [`AddrInfoW`] structures to be freed. All dynamic storage pointed to
    /// within the [`AddrInfoW`] structure or structures is also freed.
    ///
    /// ## Return Value
    /// This function does not return a value.
    ///
    /// ## Remarks
    /// The [`FreeAddrInfoW`] function frees [`AddrInfoW`] structures
    /// dynamically allocated by the Unicode [`GetAddrInfoW`] function. The
    /// [`FreeAddrInfoW`] function frees the initial [`AddrInfoW`] structure
    /// pointed to in the `addr_info` parameter, including any buffers to which
    /// structure members point, then continues freeing any [`AddrInfoW`]
    /// structures linked by the `next` member of the [`AddrInfoW`] structure.
    /// The [`FreeAddrInfoW`] function continues freeing linked structures
    /// until a [`null`] `next` member is encountered.
    pub fn FreeAddrInfoW(addr_info: PAddrInfoW);
}
