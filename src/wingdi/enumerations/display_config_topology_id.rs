/// The [`DISPLAYCONFIG_TOPOLOGY_ID`] enumeration specifies the type of display topology.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum DISPLAYCONFIG_TOPOLOGY_ID {
    /// Indicates that the display topology is an internal configuration.
    Internal = 0x00000001,

    /// Indicates that the display topology is clone-view configuration.
    Clone = 0x00000002,

    /// Indicates that the display topology is an extended configuration.
    Extend = 0x00000004,

    /// Indicates that the display topology is an external configuration.
    External = 0x00000008,

    /// Forces this enumeration to compile to 32 bits in size. Without this value, some compilers
    /// would allow this enumeration to compile to a size other than 32 bits. You should not use
    /// this value.
    ForceUint32 = 0xFFFFFFFF,
}
