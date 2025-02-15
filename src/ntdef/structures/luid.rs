use crate::{LONG, ULONG};

/// The [`LUID`] structure is an opaque structure that specifies an identifier that is guaranteed
/// to be unique on the local machine.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LUID {
    #[allow(missing_docs)]
    pub low_part: ULONG,

    #[allow(missing_docs)]
    pub high_part: LONG,
}
