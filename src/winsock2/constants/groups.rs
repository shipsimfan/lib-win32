use crate::winsock2::GROUP;

/// Create an unconstrained socket group and have the new socket be the first member. For an
/// unconstrained group, Winsock does not constrain all sockets in the socket group to have been
/// created with the same value for the `type` and `protocol` parameters.
pub const SG_UNCONSTRAINED_GROUP: GROUP = 0x01;

/// Create a constrained socket group and have the new socket be the first member. For a
/// constrained socket group, Winsock constrains all sockets in the socket group to have been
/// created with the same value for the type and protocol parameters. A constrained socket group
/// may consist only of connection-oriented sockets, and requires that connections on all grouped
/// sockets be to the same address on the same host.
pub const SG_CONSTRAINED_GROUP: GROUP = 0x02;
