use crate::DWORD;

#[allow(missing_docs)]
pub const FD_READ_BIT: DWORD = 0;

/// Wants to receive notification of readiness for reading.
pub const FD_READ: DWORD = 1 << FD_READ_BIT;

#[allow(missing_docs)]
pub const FD_WRITE_BIT: DWORD = 1;

/// Wants to receive notification of readiness for writing.
pub const FD_WRITE: DWORD = 1 << FD_WRITE_BIT;

#[allow(missing_docs)]
pub const FD_OOB_BIT: DWORD = 2;

/// Wants to receive notification of the arrival of OOB data.
pub const FD_OOB: DWORD = 1 << FD_OOB_BIT;

#[allow(missing_docs)]
pub const FD_ACCEPT_BIT: DWORD = 3;

/// Wants to receive notification of incoming connections.
pub const FD_ACCEPT: DWORD = 1 << FD_ACCEPT_BIT;

#[allow(missing_docs)]
pub const FD_CONNECT_BIT: DWORD = 4;

/// Wants to receive notification of completed connection or multipoint join operation.
pub const FD_CONNECT: DWORD = 1 << FD_CONNECT_BIT;

#[allow(missing_docs)]
pub const FD_CLOSE_BIT: DWORD = 5;

/// Wants to receive notification of socket closure.
pub const FD_CLOSE: DWORD = 1 << FD_CLOSE_BIT;

#[allow(missing_docs)]
pub const FD_QOS_BIT: DWORD = 6;

/// Wants to receive notification of socket QoS changes.
pub const FD_QOS: DWORD = 1 << FD_QOS_BIT;

#[allow(missing_docs)]
pub const FD_GROUP_QOS_BIT: DWORD = 7;

/// Reserved for future use with socket groups. Want to receive notification of socket group QoS
/// changes.
pub const FD_GROUP_QOS: DWORD = 1 << FD_GROUP_QOS_BIT;

#[allow(missing_docs)]
pub const FD_ROUTING_INTERFACE_CHANGE_BIT: DWORD = 8;

/// Wants to receive notification of routing interface changes for the specified destination.
pub const FD_ROUTING_INTERFACE_CHANGE: DWORD = 1 << FD_ROUTING_INTERFACE_CHANGE_BIT;

#[allow(missing_docs)]
pub const FD_ADDRESS_LIST_CHANGE_BIT: DWORD = 9;

/// Wants to receive notification of local address list changes for the address family of the
/// socket.
pub const FD_ADDRESS_LIST_CHANGE: DWORD = 1 << FD_ADDRESS_LIST_CHANGE_BIT;

#[allow(missing_docs)]
pub const FD_MAX_EVENTS: DWORD = 10;

#[allow(missing_docs)]
pub const FD_ALL_EVENTS: DWORD = (1 << FD_MAX_EVENTS) - 1;
