use crate::{DWORD, WORD, dbt::DBT_DEVTYP_VOLUME};

// rustdoc imports
#[allow(unused_imports)]
use crate::dbt::{DBTF_MEDIA, DBTF_NET};

/// Contains information about a logical volume.
///
/// # Remarks
/// Although the `unit_mask` member may specify more than one volume in any message, this does not
/// guarantee that only one message is generated for a specified event. Multiple system features
/// may independently generate messages for logical volumes at the same time.
///
/// Messages for media arrival and removal are sent only for media in devices that support a
/// soft-eject mechanism. For example, applications will not see media-related volume messages for
/// floppy disks.
///
/// Messages for network drive arrival and removal are not sent whenever network commands are
/// issued, but rather when network connections will disappear as the result of a hardware event.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DEV_BROADCAST_VOLUME {
    /// The size of this structure, in bytes.
    pub size: DWORD,

    /// Set to [`DBT_DEVTYP_VOLUME`].
    pub device_type: DWORD,

    /// Reserved; do not use.
    pub reserved: DWORD,

    /// The logical unit mask identifying one or more logical units. Each bit in the mask
    /// corresponds to one logical drive. Bit 0 represents drive A, bit 1 represents drive B, and
    /// so on.
    pub unit_mask: DWORD,

    /// This parameter can be one of the following values:
    ///  * [`DBTF_MEDIA`] - Change affects media in drive. If not set, change affects physical
    ///                     device or drive.
    ///  * [`DBTF_NET`] - Indicated logical volume is a network volume.
    pub flags: WORD,
}

impl Default for DEV_BROADCAST_VOLUME {
    fn default() -> Self {
        DEV_BROADCAST_VOLUME {
            size: std::mem::size_of::<DEV_BROADCAST_VOLUME>() as _,
            device_type: DBT_DEVTYP_VOLUME,
            reserved: 0,
            unit_mask: 0,
            flags: 0,
        }
    }
}
