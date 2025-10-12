/// Specifies the type of I/O bus that is used by the graphics adapter.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_BUS_TYPE {
    /// Indicates a type of bus other than the types listed here.
    Other = 0,

    /// PCI bus.
    Pci = 0x1,

    /// PCI-X bus.
    PciX = 0x2,

    /// PCI Express bus.
    PciExpress = 0x3,

    /// Accelerated Graphics Port (AGP) bus.
    Agp = 0x4,

    /// The implementation for the graphics adapter is in a motherboard chipset's north bridge.
    /// This flag implies that data never goes over an expansion bus (such as PCI or AGP) when it
    /// is transferred from main memory to the graphics adapter.
    ModifierInsideOfChipset = 0x10000,

    /// Indicates that the graphics adapter is connected to a motherboard chipset's north bridge by
    /// tracks on the motherboard, and all of the graphics adapter's chips are soldered to the
    /// motherboard. This flag implies that data never goes over an expansion bus (such as PCI or
    /// AGP) when it is transferred from main memory to the graphics adapter.
    ModifierTracksOnMotherBoardToChip = 0x20000,

    /// The graphics adapter is connected to a motherboard chipset's north bridge by tracks on the
    /// motherboard, and all of the graphics adapter's chips are connected through sockets to the
    /// motherboard.
    ModifierTracksOnMotherBoardToSocket = 0x30000,

    /// The graphics adapter is connected to the motherboard through a daughterboard connector.
    ModifierDaughterBoardConnector = 0x40000,

    /// The graphics adapter is connected to the motherboard through a daughterboard connector, and
    /// the graphics adapter is inside an enclosure that is not user accessible.
    ModifierDaughterBoardConnectorInsideOfNuae = 0x50000,

    /// One of the `D3D11_BUS_TYPE::Modifier*` flags is set.
    ModifierNonStandard = 0x80000000,
}
