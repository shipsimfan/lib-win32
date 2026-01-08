//! The data types supported by Windows are used to define function return values, function and
//! message parameters, and structure members

mod types;

pub use types::{
    DWORD64, DWORD_PTR, INT_PTR, LONG64, LONG_PTR, PDWORD_PTR, PULONG_PTR, SIZE_T, UINT16, UINT64,
    UINT8, UINT_PTR, ULONG64, ULONG_PTR,
};
