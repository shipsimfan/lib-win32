#[macro_export]
#[allow(missing_docs)]
#[cfg(target_pointer_width = "64")]
macro_rules! RAWINPUT_ALIGN {
    ($x: expr) => {
        ($x as usize + std::mem::size_of::<$crate::QWORD>() - 1)
            & !(std::mem::size_of::<$crate::QWORD>() - 1)
    };
}

#[macro_export]
#[allow(missing_docs)]
#[cfg(target_pointer_width = "32")]
macro_rules! RAWINPUT_ALIGN {
    ($x: expr) => {
        ($x as usize + std::mem::size_of::<$crate::DWORD>() - 1)
            & !(std::mem::size_of::<$crate::DWORD>() - 1)
    };
}
