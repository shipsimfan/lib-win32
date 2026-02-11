use crate::{DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_DEVICE_INFO_TYPE, UINT32};
use std::ops::{Deref, DerefMut};

/// The [`DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION`] structure contains information on the state of
/// virtual resolution support for the monitor.
#[repr(C)]
#[derive(Clone)]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    /// A [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure that holds information on the `r#type`,
    /// `size`, `adapter_id`, and `id` of the target the monitor is connected to.
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,

    #[allow(missing_docs)]
    pub dummy: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_UNION,
}

impl Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    fn default() -> Self {
        DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE::GetSupportVirtualResolution,
                size: std::mem::size_of::<DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION>() as _,
                ..Default::default()
            },
            dummy: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_UNION::default(),
        }
    }
}

impl Deref for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    type Target = DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_UNION;

    fn deref(&self) -> &Self::Target {
        &self.dummy
    }
}

impl DerefMut for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dummy
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub union DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_UNION {
    #[allow(missing_docs)]
    pub dummy: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_STRUCT,

    /// Reflects the value of `disable_monitor_virtual_resolution` in cases where debugging is
    /// utilized.
    pub value: UINT32,
}

impl Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_UNION {
    fn default() -> Self {
        DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_UNION { value: 0 }
    }
}

impl Deref for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_UNION {
    type Target = DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_STRUCT;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.dummy }
    }
}

impl DerefMut for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_UNION {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.dummy }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_STRUCT {
    /// Setting this bit disables virtual mode for the monitor using information found in header.
    pub disable_monitor_virtual_resolution: UINT32,
}

impl Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_STRUCT {
    fn default() -> Self {
        DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_STRUCT {
            disable_monitor_virtual_resolution: 0,
        }
    }
}
