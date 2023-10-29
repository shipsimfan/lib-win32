use crate::raw::DWord;
use std::ffi::c_int;

pub type LPWSAProtocolInfoW = *mut WSAProtocolChain;

/// # WSAProtocolChain structure (winsock2.h)
///
/// The [`WSAProtocolChain`] structure contains a counted list of Catalog Entry
/// identifiers that comprise a protocol chain.
///
/// ## Members
/// `chain_len`\
/// Length of the chain, in bytes. The following settings apply:
///  - Setting `chain_len` to zero indicates a layered protocol
///  - Setting `chain_len` to one indicates a base protocol
///  - Setting `chain_len` to greater than one indicates a protocol chain
///
/// `chain_entries`\
/// Array of protocol chain entries.
///
/// ## Remarks
/// If the length of the chain is larger than 1, this structure represents a
/// protocol chain which consists of one or more layered protocols on top of a
/// base protocol. The corresponding Catalog Entry IDs are in the
/// ProtocolChain.ChainEntries array starting with the layered protocol at the
/// top (the zeroth element in the ProtocolChain.ChainEntries array) and ending
/// with the base protocol. Refer to Windows Sockets 2 Service Provider
/// Interface for more information on protocol chains.
#[repr(C)]
#[derive(Clone)]
pub struct WSAProtocolChain {
    pub chain_len: c_int,
    pub chain_entries: [DWord; MAX_PROTOCOL_CHAIN],
}

pub const MAX_PROTOCOL_CHAIN: usize = 7;
