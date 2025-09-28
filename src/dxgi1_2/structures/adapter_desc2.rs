use crate::{
    dxgi1_2::{DXGI_COMPUTE_PREEMPTION_GRANULARITY, DXGI_GRAPHICS_PREEMPTION_GRANULARITY},
    LUID, SIZE_T, UINT, WCHAR,
};

#[allow(unused_imports)]
use crate::dxgi::DXGI_ADAPTER_FLAG;

/// Describes an adapter (or video card) that uses Microsoft DirectX Graphics Infrastructure (DXGI)
/// 1.2.
///
/// # Remarks
/// The [`DXGI_ADAPTER_DESC2`] structure provides a DXGI 1.2 description of an adapter. This
/// structure is initialized by using the [`IDXGIAdapter2::get_desc2`] method.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_ADAPTER_DESC2 {
    /// A string that contains the adapter description.
    pub description: [WCHAR; 128],

    /// The PCI ID or ACPI ID of the adapter's hardware vendor. If this value is less than or equal
    /// to 0xFFFF, it is a PCI ID; otherwise, it is an ACPI ID.
    pub vendor_id: UINT,

    /// The PCI ID or ACPI ID of the adapter's hardware device. If VendorId is a PCI ID, it is also
    /// a PCI ID; otherwise, it is an ACPI ID.
    pub device_id: UINT,

    /// The PCI ID or ACPI ID of the adapter's hardware subsystem. If VendorId is a PCI ID, it is
    /// also a PCI ID; otherwise, it is an ACPI ID.
    pub sub_sys_id: UINT,

    /// The adapter's PCI or ACPI revision number. If VendorId is a PCI ID, it is a PCI device
    /// revision number; otherwise, it is an ACPI device revision number.
    pub revision: UINT,

    /// The number of bytes of dedicated video memory that are not shared with the CPU.
    pub dedicated_video_memory: SIZE_T,

    /// The number of bytes of dedicated system memory that are not shared with the CPU. This
    /// memory is allocated from available system memory at boot time.
    pub dedicated_system_memory: SIZE_T,

    /// The number of bytes of shared system memory. This is the maximum value of system memory
    /// that may be consumed by the adapter during operation. Any incidental memory consumed by the
    /// driver as it manages and uses video memory is additional.
    pub shared_system_memory: SIZE_T,

    /// A unique value that identifies the adapter. See [`LUID`] for a definition of the structure.
    pub adapter_luid: LUID,

    /// A value of the [`DXGI_ADAPTER_FLAG`] enumerated type that describes the adapter type. The
    /// [`DXGI_ADAPTER_FLAG::Remote`] flag is reserved.
    pub flags: UINT,

    /// A value of the [`DXGI_GRAPHICS_PREEMPTION_GRANULARITY`] enumerated type that describes the
    /// granularity level at which the GPU can be preempted from performing its current graphics
    /// rendering task.
    pub graphics_preemption_granularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,

    /// A value of the [`DXGI_COMPUTE_PREEMPTION_GRANULARITY`] enumerated type that describes the
    /// granularity level at which the GPU can be preempted from performing its current compute
    /// task.
    pub compute_preemption_granularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}

impl Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        DXGI_ADAPTER_DESC2 {
            description: [0; 128],
            vendor_id: 0,
            device_id: 0,
            sub_sys_id: 0,
            revision: 0,
            dedicated_video_memory: 0,
            dedicated_system_memory: 0,
            shared_system_memory: 0,
            adapter_luid: LUID::default(),
            flags: 0,
            graphics_preemption_granularity:
                DXGI_GRAPHICS_PREEMPTION_GRANULARITY::DmaBufferBoundary,
            compute_preemption_granularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY::DmaBufferBoundary,
        }
    }
}
