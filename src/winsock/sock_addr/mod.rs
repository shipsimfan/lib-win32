use std::ops::{Deref, DerefMut};

mod ipv4;

pub use ipv4::*;

pub trait SockAddr: Deref<Target = Self::Inner> + DerefMut {
    const FAMILY: u16;

    type Inner;
}
