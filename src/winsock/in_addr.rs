use crate::raw::in_addr;
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut, Index, IndexMut},
};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InAddr(in_addr);

impl InAddr {
    pub fn byte(&self, index: usize) -> u8 {
        *self.byte_ref(index)
    }

    pub fn word(&self, index: usize) -> u16 {
        *self.word_ref(index)
    }

    pub fn long(&self) -> u32 {
        unsafe { self.0.un.addr }
    }

    pub fn byte_ref(&self, index: usize) -> &u8 {
        unsafe {
            match index {
                0 => &self.0.un.un_b.b1,
                1 => &self.0.un.un_b.b2,
                2 => &self.0.un.un_b.b3,
                3 => &self.0.un.un_b.b4,
                _ => panic!("index {} is too large for InAddr", index),
            }
        }
    }

    pub fn word_ref(&self, index: usize) -> &u16 {
        unsafe {
            match index {
                0 => &self.0.un.un_w.w1,
                1 => &self.0.un.un_w.w2,
                _ => panic!("index {} is too large for InAddr", index),
            }
        }
    }

    pub fn long_ref(&self) -> &u32 {
        unsafe { &self.0.un.addr }
    }

    pub fn byte_mut(&mut self, index: usize) -> &mut u8 {
        unsafe {
            match index {
                0 => &mut self.0.un.un_b.b1,
                1 => &mut self.0.un.un_b.b2,
                2 => &mut self.0.un.un_b.b3,
                3 => &mut self.0.un.un_b.b4,
                _ => panic!("index {} is too large for InAddr", index),
            }
        }
    }

    pub fn word_mut(&mut self, index: usize) -> &mut u16 {
        unsafe {
            match index {
                0 => &mut self.0.un.un_w.w1,
                1 => &mut self.0.un.un_w.w2,
                _ => panic!("index {} is too large for InAddr", index),
            }
        }
    }

    pub fn long_mut(&mut self) -> &mut u32 {
        unsafe { &mut self.0.un.addr }
    }

    pub fn set_byte(&mut self, index: usize, byte: u8) {
        *self.byte_mut(index) = byte;
    }

    pub fn set_word(&mut self, index: usize, word: u16) {
        *self.word_mut(index) = word;
    }

    pub fn set_long(&mut self, long: u32) {
        self.0.un.addr = long;
    }
}

impl Index<usize> for InAddr {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.byte_ref(index)
    }
}

impl IndexMut<usize> for InAddr {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.byte_mut(index)
    }
}

impl From<[u8; 4]> for InAddr {
    fn from(value: [u8; 4]) -> Self {
        let mut in_addr = in_addr::new();
        in_addr.un.un_b.b1 = value[0];
        in_addr.un.un_b.b2 = value[1];
        in_addr.un.un_b.b3 = value[2];
        in_addr.un.un_b.b4 = value[3];

        InAddr(in_addr)
    }
}

impl From<[u16; 2]> for InAddr {
    fn from(value: [u16; 2]) -> Self {
        let mut in_addr = in_addr::new();
        in_addr.un.un_w.w1 = value[0];
        in_addr.un.un_w.w2 = value[1];

        InAddr(in_addr)
    }
}

impl From<&[u8]> for InAddr {
    fn from(value: &[u8]) -> Self {
        assert!(value.len() >= 4);
        let mut in_addr = in_addr::new();
        in_addr.un.un_b.b1 = value[0];
        in_addr.un.un_b.b2 = value[1];
        in_addr.un.un_b.b3 = value[2];
        in_addr.un.un_b.b4 = value[3];

        InAddr(in_addr)
    }
}

impl From<&[u16]> for InAddr {
    fn from(value: &[u16]) -> Self {
        assert!(value.len() >= 2);
        let mut in_addr = in_addr::new();
        in_addr.un.un_w.w1 = value[0];
        in_addr.un.un_w.w2 = value[1];

        InAddr(in_addr)
    }
}

impl From<u32> for InAddr {
    fn from(value: u32) -> Self {
        let mut in_addr = in_addr::new();
        in_addr.un.addr = value;

        InAddr(in_addr)
    }
}

impl From<in_addr> for InAddr {
    fn from(value: in_addr) -> Self {
        InAddr(value)
    }
}

impl<'a> From<&'a in_addr> for &'a InAddr {
    fn from(value: &'a in_addr) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl<'a> From<&'a mut in_addr> for &'a mut InAddr {
    fn from(value: &'a mut in_addr) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<[u8; 4]> for InAddr {
    fn into(self) -> [u8; 4] {
        unsafe {
            [
                self.un.un_b.b1,
                self.un.un_b.b2,
                self.un.un_b.b3,
                self.un.un_b.b4,
            ]
        }
    }
}

impl Into<[u16; 2]> for InAddr {
    fn into(self) -> [u16; 2] {
        unsafe { [self.un.un_w.w1, self.un.un_w.w2] }
    }
}

impl Into<u32> for InAddr {
    fn into(self) -> u32 {
        unsafe { self.0.un.addr }
    }
}

impl Into<in_addr> for InAddr {
    fn into(self) -> in_addr {
        self.0
    }
}

impl<'a> Into<&'a in_addr> for &'a InAddr {
    fn into(self) -> &'a in_addr {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> Into<&'a mut in_addr> for &'a mut InAddr {
    fn into(self) -> &'a mut in_addr {
        unsafe { std::mem::transmute(self) }
    }
}

impl Deref for InAddr {
    type Target = in_addr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InAddr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for InAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self[0], self[1], self[2], self[3])
    }
}

impl Debug for InAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
