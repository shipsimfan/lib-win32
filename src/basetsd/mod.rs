//! The data types supported by Windows are used to define function return values, function and
//! message parameters, and structure members

mod types;

pub use types::{
    DWORD64, DWORD_PTR, INT_PTR, LONG64, LONG_PTR, PULONG_PTR, SIZE_T, UINT64, UINT_PTR, ULONG64,
    ULONG_PTR,
};
