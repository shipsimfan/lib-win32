/// Flags indicating the memory location of a resource.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum DXGI_RESIDENCY {
    /// The resource is located in video memory.
    FullyResident = 1,

    /// At least some of the resource is located in CPU memory.
    ResidentInSharedMemory = 2,

    /// At least some of the resource has been paged out to the hard drive.
    EvictedToDisk = 3,
}
