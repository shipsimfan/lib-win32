mod ipv4;

use std::ops::{Deref, DerefMut};

pub use ipv4::*;

pub trait SockAddr: Deref<Target = Self::Inner> + DerefMut {
    const FAMILY: u16;

    type Inner;
}
