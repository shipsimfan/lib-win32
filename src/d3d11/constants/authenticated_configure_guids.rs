use crate::GUID;

/// Initializes the authenticated channel.
pub const D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE: GUID = GUID {
    data1: 0x6114BDB,
    data2: 0x3523,
    data3: 0x470A,
    data4: [0x8D, 0xCA, 0xFB, 0xC2, 0x84, 0x51, 0x54, 0xF0],
};

/// Enables or disables protection for the device.
pub const D3D11_AUTHENTICATED_CONFIGURE_PROTECTION: GUID = GUID {
    data1: 0x50455658,
    data2: 0x3F47,
    data3: 0x4362,
    data4: [0xBF, 0x99, 0xBF, 0xDF, 0xCD, 0xE9, 0xED, 0x29],
};

/// Associates a cryptographic session with a decoder device and a Direct3D device.
pub const D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION: GUID = GUID {
    data1: 0x6346CC54,
    data2: 0x2CFC,
    data3: 0x4AD4,
    data4: [0x82, 0x24, 0xD1, 0x58, 0x37, 0xDE, 0x77, 0x0],
};

/// Sets the level of encryption that is performed before protected content becomes accessible to
/// the CPU or bus.
pub const D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE: GUID = GUID {
    data1: 0x772D047,
    data2: 0x1B40,
    data3: 0x48E8,
    data4: [0x9C, 0xA6, 0xB5, 0xF5, 0x10, 0xDE, 0x9F, 0x1],
};

/// Sets the level of encryption that is performed before protected content becomes accessible to
/// the CPU or bus.
pub const D3D11_AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE: GUID = GUID {
    data1: 0x41FFF286,
    data2: 0x6AE0,
    data3: 0x4D43,
    data4: [0x9D, 0x55, 0xA4, 0x6E, 0x9E, 0xFD, 0x15, 0x8A],
};
