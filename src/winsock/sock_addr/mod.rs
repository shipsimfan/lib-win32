mod ipv4;

pub use ipv4::*;

pub trait SockAddr {
    const FAMILY: u16;
    const LENGTH: usize;
}
