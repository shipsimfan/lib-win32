use crate::{DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_DEVICE_INFO_TYPE, UINT32};
use std::ops::{Deref, DerefMut};

/// The [`DISPLAYCONFIG_SET_TARGET_PERSISTENCE`] structure contains information about setting the
/// display.
#[repr(C)]
#[derive(Clone)]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    /// A [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure that contains information for setting the
    /// target persistence. The `r#type` member of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] is set to
    /// [`DISPLAYCONFIG_DEVICE_INFO_TYPE::SetTargetPersistence`].
    /// [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] also contains the adapter and target identifiers of
    /// the target to set the persistence for. The `size` member of
    /// [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] is set to at least the size of the
    /// [`DISPLAYCONFIG_SET_TARGET_PERSISTENCE`] structure.
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,

    #[allow(missing_docs)]
    pub dummy: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_UNION,
}

impl Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    fn default() -> Self {
        DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE::SetTargetPersistence,
                size: std::mem::size_of::<DISPLAYCONFIG_SET_TARGET_PERSISTENCE>() as _,
                ..Default::default()
            },
            dummy: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_UNION::default(),
        }
    }
}

impl Deref for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    type Target = DISPLAYCONFIG_SET_TARGET_PERSISTENCE_UNION;

    fn deref(&self) -> &Self::Target {
        &self.dummy
    }
}

impl DerefMut for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dummy
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub union DISPLAYCONFIG_SET_TARGET_PERSISTENCE_UNION {
    #[allow(missing_docs)]
    pub dummy: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_STRUCT,

    /// A member in the union that [`DISPLAYCONFIG_SET_TARGET_PERSISTENCE`] contains that can hold
    /// a 32-bit value that identifies information about setting the display.
    pub value: UINT32,
}

impl Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_UNION {
    fn default() -> Self {
        DISPLAYCONFIG_SET_TARGET_PERSISTENCE_UNION { value: 0 }
    }
}

impl Deref for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_UNION {
    type Target = DISPLAYCONFIG_SET_TARGET_PERSISTENCE_STRUCT;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.dummy }
    }
}

impl DerefMut for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_UNION {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.dummy }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE_STRUCT {
    /// A [`UINT32`] value that specifies whether the [`SetDisplayConfig`] function should enable
    /// or disable boot persistence for the specified target.
    ///
    /// Setting this member is equivalent to setting the first bit of the 32-bit `value` member
    /// (0x00000001).
    pub boot_persistence_on: UINT32,
}

impl Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_STRUCT {
    fn default() -> Self {
        DISPLAYCONFIG_SET_TARGET_PERSISTENCE_STRUCT {
            boot_persistence_on: 0,
        }
    }
}
