use crate::shtypes::KNOWNFOLDERID;

/// legacy CSIDL value: CSIDL_NETWORK
///
/// display name: "Network"
///
/// legacy display name: "My Network Places"
///
/// default path:
///
/// {D20BEEC4-5CA8-4905-AE3B-BF251EA09B53}
#[allow(non_upper_case_globals)]
pub const FOLDERID_NetworkFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xD20BEEC4,
    data2: 0x5CA8,
    data3: 0x4905,
    data4: [0xAE, 0x3B, 0xBF, 0x25, 0x1E, 0xA0, 0x9B, 0x53],
};

/// {0AC0837C-BBF8-452A-850D-79D08E667CA7}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ComputerFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x0AC0837C,
    data2: 0xBBF8,
    data3: 0x452A,
    data4: [0x85, 0x0D, 0x79, 0xD0, 0x8E, 0x66, 0x7C, 0xA7],
};

/// {4D9F7874-4E0C-4904-967B-40B0D20C3E4B}
#[allow(non_upper_case_globals)]
pub const FOLDERID_InternetFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x4D9F7874,
    data2: 0x4E0C,
    data3: 0x4904,
    data4: [0x96, 0x7B, 0x40, 0xB0, 0xD2, 0x0C, 0x3E, 0x4B],
};

/// {82A74AEB-AEB4-465C-A014-D097EE346D63}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ControlPanelFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x82A74AEB,
    data2: 0xAEB4,
    data3: 0x465C,
    data4: [0xA0, 0x14, 0xD0, 0x97, 0xEE, 0x34, 0x6D, 0x63],
};

/// {76FC4E2D-D6AD-4519-A663-37BD56068185}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PrintersFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x76FC4E2D,
    data2: 0xD6AD,
    data3: 0x4519,
    data4: [0xA6, 0x63, 0x37, 0xBD, 0x56, 0x06, 0x81, 0x85],
};

/// {43668BF8-C14E-49B2-97C9-747784D784B7}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SyncManagerFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x43668BF8,
    data2: 0xC14E,
    data3: 0x49B2,
    data4: [0x97, 0xC9, 0x74, 0x77, 0x84, 0xD7, 0x84, 0xB7],
};

/// {0F214138-B1D3-4a90-BBA9-27CBC0C5389A}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SyncSetupFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xF214138,
    data2: 0xB1D3,
    data3: 0x4A90,
    data4: [0xBB, 0xA9, 0x27, 0xCB, 0xC0, 0xC5, 0x38, 0x9A],
};

/// {4bfefb45-347d-4006-a5be-ac0cb0567192}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ConflictFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x4BFEFB45,
    data2: 0x347D,
    data3: 0x4006,
    data4: [0xA5, 0xBE, 0xAC, 0x0C, 0xB0, 0x56, 0x71, 0x92],
};

/// {289a9a43-be44-4057-a41b-587a76d7e7f9}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SyncResultsFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x289A9A43,
    data2: 0xBE44,
    data3: 0x4057,
    data4: [0xA4, 0x1B, 0x58, 0x7A, 0x76, 0xD7, 0xE7, 0xF9],
};

/// {B7534046-3ECB-4C18-BE4E-64CD4CB7D6AC}
#[allow(non_upper_case_globals)]
pub const FOLDERID_RecycleBinFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xB7534046,
    data2: 0x3ECB,
    data3: 0x4C18,
    data4: [0xBE, 0x4E, 0x64, 0xCD, 0x4C, 0xB7, 0xD6, 0xAC],
};

/// {6F0CD92B-2E97-45D1-88FF-B0D186B8DEDD}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ConnectionsFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x6F0CD92B,
    data2: 0x2E97,
    data3: 0x45D1,
    data4: [0x88, 0xFF, 0xB0, 0xD1, 0x86, 0xB8, 0xDE, 0xDD],
};

/// {FD228CB7-AE11-4AE3-864C-16F3910AB8FE}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Fonts: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xFD228CB7,
    data2: 0xAE11,
    data3: 0x4AE3,
    data4: [0x86, 0x4C, 0x16, 0xF3, 0x91, 0x0A, 0xB8, 0xFE],
};

/// display name:        "Desktop"
///
/// default path:        "C:\Users\<UserName>\Desktop"
///
/// legacy default path: "C:\Documents and Settings\<userName>\Desktop"
///
/// legacy CSIDL value:  CSIDL_DESKTOP
///
/// {B4BFCC3A-DB2C-424C-B029-7FE99A87C641}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Desktop: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xB4BFCC3A,
    data2: 0xDB2C,
    data3: 0x424C,
    data4: [0xB0, 0x29, 0x7F, 0xE9, 0x9A, 0x87, 0xC6, 0x41],
};

/// {B97D20BB-F46A-4C97-BA10-5E3608430854}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Startup: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xB97D20BB,
    data2: 0xF46A,
    data3: 0x4C97,
    data4: [0xBA, 0x10, 0x5E, 0x36, 0x08, 0x43, 0x08, 0x54],
};

/// {A77F5D77-2E2B-44C3-A6A2-ABA601054A51}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Programs: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA77F5D77,
    data2: 0x2E2B,
    data3: 0x44C3,
    data4: [0xA6, 0xA2, 0xAB, 0xA6, 0x01, 0x05, 0x4A, 0x51],
};

/// {625B53C3-AB48-4EC1-BA1F-A1EF4146FC19}
#[allow(non_upper_case_globals)]
pub const FOLDERID_StartMenu: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x625B53C3,
    data2: 0xAB48,
    data3: 0x4EC1,
    data4: [0xBA, 0x1F, 0xA1, 0xEF, 0x41, 0x46, 0xFC, 0x19],
};

/// {AE50C081-EBD2-438A-8655-8A092E34987A}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Recent: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xAE50C081,
    data2: 0xEBD2,
    data3: 0x438A,
    data4: [0x86, 0x55, 0x8A, 0x09, 0x2E, 0x34, 0x98, 0x7A],
};

/// {8983036C-27C0-404B-8F08-102D10DCFD74}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SendTo: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x8983036C,
    data2: 0x27C0,
    data3: 0x404B,
    data4: [0x8F, 0x08, 0x10, 0x2D, 0x10, 0xDC, 0xFD, 0x74],
};

/// {FDD39AD0-238F-46AF-ADB4-6C85480369C7}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Documents: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xFDD39AD0,
    data2: 0x238F,
    data3: 0x46AF,
    data4: [0xAD, 0xB4, 0x6C, 0x85, 0x48, 0x03, 0x69, 0xC7],
};

/// {1777F761-68AD-4D8A-87BD-30B759FA33DD}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Favorites: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x1777F761,
    data2: 0x68AD,
    data3: 0x4D8A,
    data4: [0x87, 0xBD, 0x30, 0xB7, 0x59, 0xFA, 0x33, 0xDD],
};

/// {C5ABBF53-E17F-4121-8900-86626FC2C973}
#[allow(non_upper_case_globals)]
pub const FOLDERID_NetHood: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xC5ABBF53,
    data2: 0xE17F,
    data3: 0x4121,
    data4: [0x89, 0x00, 0x86, 0x62, 0x6F, 0xC2, 0xC9, 0x73],
};

/// {9274BD8D-CFD1-41C3-B35E-B13F55A758F4}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PrintHood: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x9274BD8D,
    data2: 0xCFD1,
    data3: 0x41C3,
    data4: [0xB3, 0x5E, 0xB1, 0x3F, 0x55, 0xA7, 0x58, 0xF4],
};

/// {A63293E8-664E-48DB-A079-DF759E0509F7}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Templates: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA63293E8,
    data2: 0x664E,
    data3: 0x48DB,
    data4: [0xA0, 0x79, 0xDF, 0x75, 0x9E, 0x05, 0x09, 0xF7],
};

/// {82A5EA35-D9CD-47C5-9629-E15D2F714E6E}
#[allow(non_upper_case_globals)]
pub const FOLDERID_CommonStartup: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x82A5EA35,
    data2: 0xD9CD,
    data3: 0x47C5,
    data4: [0x96, 0x29, 0xE1, 0x5D, 0x2F, 0x71, 0x4E, 0x6E],
};

/// {0139D44E-6AFE-49F2-8690-3DAFCAE6FFB8}
#[allow(non_upper_case_globals)]
pub const FOLDERID_CommonPrograms: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x0139D44E,
    data2: 0x6AFE,
    data3: 0x49F2,
    data4: [0x86, 0x90, 0x3D, 0xAF, 0xCA, 0xE6, 0xFF, 0xB8],
};

/// {A4115719-D62E-491D-AA7C-E74B8BE3B067}
#[allow(non_upper_case_globals)]
pub const FOLDERID_CommonStartMenu: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA4115719,
    data2: 0xD62E,
    data3: 0x491D,
    data4: [0xAA, 0x7C, 0xE7, 0x4B, 0x8B, 0xE3, 0xB0, 0x67],
};

/// {C4AA340D-F20F-4863-AFEF-F87EF2E6BA25}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PublicDesktop: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xC4AA340D,
    data2: 0xF20F,
    data3: 0x4863,
    data4: [0xAF, 0xEF, 0xF8, 0x7E, 0xF2, 0xE6, 0xBA, 0x25],
};

/// {62AB5D82-FDC1-4DC3-A9DD-070D1D495D97}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ProgramData: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x62AB5D82,
    data2: 0xFDC1,
    data3: 0x4DC3,
    data4: [0xA9, 0xDD, 0x07, 0x0D, 0x1D, 0x49, 0x5D, 0x97],
};

/// {B94237E7-57AC-4347-9151-B08C6C32D1F7}
#[allow(non_upper_case_globals)]
pub const FOLDERID_CommonTemplates: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xB94237E7,
    data2: 0x57AC,
    data3: 0x4347,
    data4: [0x91, 0x51, 0xB0, 0x8C, 0x6C, 0x32, 0xD1, 0xF7],
};

/// {ED4824AF-DCE4-45A8-81E2-FC7965083634}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PublicDocuments: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xED4824AF,
    data2: 0xDCE4,
    data3: 0x45A8,
    data4: [0x81, 0xE2, 0xFC, 0x79, 0x65, 0x08, 0x36, 0x34],
};

/// {3EB685DB-65F9-4CF6-A03A-E3EF65729F3D}
#[allow(non_upper_case_globals)]
pub const FOLDERID_RoamingAppData: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x3EB685DB,
    data2: 0x65F9,
    data3: 0x4CF6,
    data4: [0xA0, 0x3A, 0xE3, 0xEF, 0x65, 0x72, 0x9F, 0x3D],
};

/// {F1B32785-6FBA-4FCF-9D55-7B8E7F157091}
#[allow(non_upper_case_globals)]
pub const FOLDERID_LocalAppData: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xF1B32785,
    data2: 0x6FBA,
    data3: 0x4FCF,
    data4: [0x9D, 0x55, 0x7B, 0x8E, 0x7F, 0x15, 0x70, 0x91],
};

/// {A520A1A4-1780-4FF6-BD18-167343C5AF16}
#[allow(non_upper_case_globals)]
pub const FOLDERID_LocalAppDataLow: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA520A1A4,
    data2: 0x1780,
    data3: 0x4FF6,
    data4: [0xBD, 0x18, 0x16, 0x73, 0x43, 0xC5, 0xAF, 0x16],
};

/// {352481E8-33BE-4251-BA85-6007CAEDCF9D}
#[allow(non_upper_case_globals)]
pub const FOLDERID_InternetCache: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x352481E8,
    data2: 0x33BE,
    data3: 0x4251,
    data4: [0xBA, 0x85, 0x60, 0x07, 0xCA, 0xED, 0xCF, 0x9D],
};

/// {2B0F765D-C0E9-4171-908E-08A611B84FF6}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Cookies: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x2B0F765D,
    data2: 0xC0E9,
    data3: 0x4171,
    data4: [0x90, 0x8E, 0x08, 0xA6, 0x11, 0xB8, 0x4F, 0xF6],
};

/// {D9DC8A3B-B784-432E-A781-5A1130A75963}
#[allow(non_upper_case_globals)]
pub const FOLDERID_History: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xD9DC8A3B,
    data2: 0xB784,
    data3: 0x432E,
    data4: [0xA7, 0x81, 0x5A, 0x11, 0x30, 0xA7, 0x59, 0x63],
};

/// {1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}
#[allow(non_upper_case_globals)]
pub const FOLDERID_System: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x1AC14E77,
    data2: 0x02E7,
    data3: 0x4E5D,
    data4: [0xB7, 0x44, 0x2E, 0xB1, 0xAE, 0x51, 0x98, 0xB7],
};

/// {D65231B0-B2F1-4857-A4CE-A8E7C6EA7D27}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SystemX86: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xD65231B0,
    data2: 0xB2F1,
    data3: 0x4857,
    data4: [0xA4, 0xCE, 0xA8, 0xE7, 0xC6, 0xEA, 0x7D, 0x27],
};

/// {F38BF404-1D43-42F2-9305-67DE0B28FC23}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Windows: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xF38BF404,
    data2: 0x1D43,
    data3: 0x42F2,
    data4: [0x93, 0x05, 0x67, 0xDE, 0x0B, 0x28, 0xFC, 0x23],
};

/// {5E6C858F-0E22-4760-9AFE-EA3317B67173}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Profile: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x5E6C858F,
    data2: 0x0E22,
    data3: 0x4760,
    data4: [0x9A, 0xFE, 0xEA, 0x33, 0x17, 0xB6, 0x71, 0x73],
};

/// {33E28130-4E1E-4676-835A-98395C3BC3BB}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Pictures: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x33E28130,
    data2: 0x4E1E,
    data3: 0x4676,
    data4: [0x83, 0x5A, 0x98, 0x39, 0x5C, 0x3B, 0xC3, 0xBB],
};

/// {7C5A40EF-A0FB-4BFC-874A-C0F2E0B9FA8E}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ProgramFilesX86: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x7C5A40EF,
    data2: 0xA0FB,
    data3: 0x4BFC,
    data4: [0x87, 0x4A, 0xC0, 0xF2, 0xE0, 0xB9, 0xFA, 0x8E],
};

/// {DE974D24-D9C6-4D3E-BF91-F4455120B917}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ProgramFilesCommonX86: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xDE974D24,
    data2: 0xD9C6,
    data3: 0x4D3E,
    data4: [0xBF, 0x91, 0xF4, 0x45, 0x51, 0x20, 0xB9, 0x17],
};

/// {6D809377-6AF0-444b-8957-A3773F02200E}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ProgramFilesX64: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x6D809377,
    data2: 0x6AF0,
    data3: 0x444B,
    data4: [0x89, 0x57, 0xA3, 0x77, 0x3F, 0x02, 0x20, 0x0E],
};

/// {6365D5A7-0F0D-45e5-87F6-0DA56B6A4F7D}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ProgramFilesCommonX64: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x6365D5A7,
    data2: 0xF0D,
    data3: 0x45E5,
    data4: [0x87, 0xF6, 0xD, 0xA5, 0x6B, 0x6A, 0x4F, 0x7D],
};

/// {905e63b6-c1bf-494e-b29c-65b732d3d21a}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ProgramFiles: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x905E63B6,
    data2: 0xC1BF,
    data3: 0x494E,
    data4: [0xB2, 0x9C, 0x65, 0xB7, 0x32, 0xD3, 0xD2, 0x1A],
};

/// {F7F1ED05-9F6D-47A2-AAAE-29D317C6F066}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ProgramFilesCommon: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xF7F1ED05,
    data2: 0x9F6D,
    data3: 0x47A2,
    data4: [0xAA, 0xAE, 0x29, 0xD3, 0x17, 0xC6, 0xF0, 0x66],
};

/// {5cd7aee2-2219-4a67-b85d-6c9ce15660cb}
#[allow(non_upper_case_globals)]
pub const FOLDERID_UserProgramFiles: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x5CD7AEE2,
    data2: 0x2219,
    data3: 0x4A67,
    data4: [0xB8, 0x5D, 0x6C, 0x9C, 0xE1, 0x56, 0x60, 0xCB],
};

/// {bcbd3057-ca5c-4622-b42d-bc56db0ae516}
#[allow(non_upper_case_globals)]
pub const FOLDERID_UserProgramFilesCommon: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xBCBD3057,
    data2: 0xCA5C,
    data3: 0x4622,
    data4: [0xB4, 0x2D, 0xBC, 0x56, 0xDB, 0x0A, 0xE5, 0x16],
};

/// {724EF170-A42D-4FEF-9F26-B60E846FBA4F}
#[allow(non_upper_case_globals)]
pub const FOLDERID_AdminTools: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x724EF170,
    data2: 0xA42D,
    data3: 0x4FEF,
    data4: [0x9F, 0x26, 0xB6, 0x0E, 0x84, 0x6F, 0xBA, 0x4F],
};

/// {D0384E7D-BAC3-4797-8F14-CBA229B392B5}
#[allow(non_upper_case_globals)]
pub const FOLDERID_CommonAdminTools: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xD0384E7D,
    data2: 0xBAC3,
    data3: 0x4797,
    data4: [0x8F, 0x14, 0xCB, 0xA2, 0x29, 0xB3, 0x92, 0xB5],
};

/// {4BD8D571-6D19-48D3-BE97-422220080E43}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Music: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x4BD8D571,
    data2: 0x6D19,
    data3: 0x48D3,
    data4: [0xBE, 0x97, 0x42, 0x22, 0x20, 0x08, 0x0E, 0x43],
};

/// {18989B1D-99B5-455B-841C-AB7C74E4DDFC}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Videos: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x18989B1D,
    data2: 0x99B5,
    data3: 0x455B,
    data4: [0x84, 0x1C, 0xAB, 0x7C, 0x74, 0xE4, 0xDD, 0xFC],
};

/// {C870044B-F49E-4126-A9C3-B52A1FF411E8}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Ringtones: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xC870044B,
    data2: 0xF49E,
    data3: 0x4126,
    data4: [0xA9, 0xC3, 0xB5, 0x2A, 0x1F, 0xF4, 0x11, 0xE8],
};

/// {B6EBFB86-6907-413C-9AF7-4FC2ABF07CC5}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PublicPictures: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xB6EBFB86,
    data2: 0x6907,
    data3: 0x413C,
    data4: [0x9A, 0xF7, 0x4F, 0xC2, 0xAB, 0xF0, 0x7C, 0xC5],
};

/// {3214FAB5-9757-4298-BB61-92A9DEAA44FF}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PublicMusic: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x3214FAB5,
    data2: 0x9757,
    data3: 0x4298,
    data4: [0xBB, 0x61, 0x92, 0xA9, 0xDE, 0xAA, 0x44, 0xFF],
};

/// {2400183A-6185-49FB-A2D8-4A392A602BA3}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PublicVideos: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x2400183A,
    data2: 0x6185,
    data3: 0x49FB,
    data4: [0xA2, 0xD8, 0x4A, 0x39, 0x2A, 0x60, 0x2B, 0xA3],
};

/// {E555AB60-153B-4D17-9F04-A5FE99FC15EC}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PublicRingtones: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xE555AB60,
    data2: 0x153B,
    data3: 0x4D17,
    data4: [0x9F, 0x04, 0xA5, 0xFE, 0x99, 0xFC, 0x15, 0xEC],
};

/// {8AD10C31-2ADB-4296-A8F7-E4701232C972}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ResourceDir: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x8AD10C31,
    data2: 0x2ADB,
    data3: 0x4296,
    data4: [0xA8, 0xF7, 0xE4, 0x70, 0x12, 0x32, 0xC9, 0x72],
};

/// {2A00375E-224C-49DE-B8D1-440DF7EF3DDC}
#[allow(non_upper_case_globals)]
pub const FOLDERID_LocalizedResourcesDir: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x2A00375E,
    data2: 0x224C,
    data3: 0x49DE,
    data4: [0xB8, 0xD1, 0x44, 0x0D, 0xF7, 0xEF, 0x3D, 0xDC],
};

/// {C1BAE2D0-10DF-4334-BEDD-7AA20B227A9D}
#[allow(non_upper_case_globals)]
pub const FOLDERID_CommonOEMLinks: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xC1BAE2D0,
    data2: 0x10DF,
    data3: 0x4334,
    data4: [0xBE, 0xDD, 0x7A, 0xA2, 0x0B, 0x22, 0x7A, 0x9D],
};

/// {9E52AB10-F80D-49DF-ACB8-4330F5687855}
#[allow(non_upper_case_globals)]
pub const FOLDERID_CDBurning: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x9E52AB10,
    data2: 0xF80D,
    data3: 0x49DF,
    data4: [0xAC, 0xB8, 0x43, 0x30, 0xF5, 0x68, 0x78, 0x55],
};

/// {0762D272-C50A-4BB0-A382-697DCD729B80}
#[allow(non_upper_case_globals)]
pub const FOLDERID_UserProfiles: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x0762D272,
    data2: 0xC50A,
    data3: 0x4BB0,
    data4: [0xA3, 0x82, 0x69, 0x7D, 0xCD, 0x72, 0x9B, 0x80],
};

/// {DE92C1C7-837F-4F69-A3BB-86E631204A23}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Playlists: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xDE92C1C7,
    data2: 0x837F,
    data3: 0x4F69,
    data4: [0xA3, 0xBB, 0x86, 0xE6, 0x31, 0x20, 0x4A, 0x23],
};

/// {15CA69B3-30EE-49C1-ACE1-6B5EC372AFB5}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SamplePlaylists: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x15CA69B3,
    data2: 0x30EE,
    data3: 0x49C1,
    data4: [0xAC, 0xE1, 0x6B, 0x5E, 0xC3, 0x72, 0xAF, 0xB5],
};

/// {B250C668-F57D-4EE1-A63C-290EE7D1AA1F}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SampleMusic: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xB250C668,
    data2: 0xF57D,
    data3: 0x4EE1,
    data4: [0xA6, 0x3C, 0x29, 0x0E, 0xE7, 0xD1, 0xAA, 0x1F],
};

/// {C4900540-2379-4C75-844B-64E6FAF8716B}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SamplePictures: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xC4900540,
    data2: 0x2379,
    data3: 0x4C75,
    data4: [0x84, 0x4B, 0x64, 0xE6, 0xFA, 0xF8, 0x71, 0x6B],
};

/// {859EAD94-2E85-48AD-A71A-0969CB56A6CD}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SampleVideos: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x859EAD94,
    data2: 0x2E85,
    data3: 0x48AD,
    data4: [0xA7, 0x1A, 0x09, 0x69, 0xCB, 0x56, 0xA6, 0xCD],
};

/// {69D2CF90-FC33-4FB7-9A0C-EBB0F0FCB43C}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PhotoAlbums: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x69D2CF90,
    data2: 0xFC33,
    data3: 0x4FB7,
    data4: [0x9A, 0x0C, 0xEB, 0xB0, 0xF0, 0xFC, 0xB4, 0x3C],
};

/// {DFDF76A2-C82A-4D63-906A-5644AC457385}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Public: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xDFDF76A2,
    data2: 0xC82A,
    data3: 0x4D63,
    data4: [0x90, 0x6A, 0x56, 0x44, 0xAC, 0x45, 0x73, 0x85],
};

/// {df7266ac-9274-4867-8d55-3bd661de872d}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ChangeRemovePrograms: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xDF7266AC,
    data2: 0x9274,
    data3: 0x4867,
    data4: [0x8D, 0x55, 0x3B, 0xD6, 0x61, 0xDE, 0x87, 0x2D],
};

/// {a305ce99-f527-492b-8b1a-7e76fa98d6e4}
#[allow(non_upper_case_globals)]
pub const FOLDERID_AppUpdates: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA305CE99,
    data2: 0xF527,
    data3: 0x492B,
    data4: [0x8B, 0x1A, 0x7E, 0x76, 0xFA, 0x98, 0xD6, 0xE4],
};

/// {de61d971-5ebc-4f02-a3a9-6c82895e5c04}
#[allow(non_upper_case_globals)]
pub const FOLDERID_AddNewPrograms: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xDE61D971,
    data2: 0x5EBC,
    data3: 0x4F02,
    data4: [0xA3, 0xA9, 0x6C, 0x82, 0x89, 0x5E, 0x5C, 0x04],
};

/// {374DE290-123F-4565-9164-39C4925E467B}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Downloads: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x374DE290,
    data2: 0x123F,
    data3: 0x4565,
    data4: [0x91, 0x64, 0x39, 0xC4, 0x92, 0x5E, 0x46, 0x7B],
};

/// {3D644C9B-1FB8-4f30-9B45-F670235F79C0}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PublicDownloads: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x3D644C9B,
    data2: 0x1FB8,
    data3: 0x4F30,
    data4: [0x9B, 0x45, 0xF6, 0x70, 0x23, 0x5F, 0x79, 0xC0],
};

/// {7d1d3a04-debb-4115-95cf-2f29da2920da}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SavedSearches: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x7D1D3A04,
    data2: 0xDEBB,
    data3: 0x4115,
    data4: [0x95, 0xCF, 0x2F, 0x29, 0xDA, 0x29, 0x20, 0xDA],
};

/// {52a4f021-7b75-48a9-9f6b-4b87a210bc8f}
#[allow(non_upper_case_globals)]
pub const FOLDERID_QuickLaunch: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x52A4F021,
    data2: 0x7B75,
    data3: 0x48A9,
    data4: [0x9F, 0x6B, 0x4B, 0x87, 0xA2, 0x10, 0xBC, 0x8F],
};

/// {56784854-C6CB-462b-8169-88E350ACB882}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Contacts: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x56784854,
    data2: 0xC6CB,
    data3: 0x462B,
    data4: [0x81, 0x69, 0x88, 0xE3, 0x50, 0xAC, 0xB8, 0x82],
};

/// {A75D362E-50FC-4fb7-AC2C-A8BEAA314493}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SidebarParts: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA75D362E,
    data2: 0x50FC,
    data3: 0x4FB7,
    data4: [0xAC, 0x2C, 0xA8, 0xBE, 0xAA, 0x31, 0x44, 0x93],
};

/// {7B396E54-9EC5-4300-BE0A-2482EBAE1A26}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SidebarDefaultParts: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x7B396E54,
    data2: 0x9EC5,
    data3: 0x4300,
    data4: [0xBE, 0xA, 0x24, 0x82, 0xEB, 0xAE, 0x1A, 0x26],
};

/// {DEBF2536-E1A8-4c59-B6A2-414586476AEA}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PublicGameTasks: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xDEBF2536,
    data2: 0xE1A8,
    data3: 0x4C59,
    data4: [0xB6, 0xA2, 0x41, 0x45, 0x86, 0x47, 0x6A, 0xEA],
};

/// {054FAE61-4DD8-4787-80B6-090220C4B700}
#[allow(non_upper_case_globals)]
pub const FOLDERID_GameTasks: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x54FAE61,
    data2: 0x4DD8,
    data3: 0x4787,
    data4: [0x80, 0xB6, 0x9, 0x2, 0x20, 0xC4, 0xB7, 0x0],
};

/// {4C5C32FF-BB9D-43b0-B5B4-2D72E54EAAA4}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SavedGames: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x4C5C32FF,
    data2: 0xBB9D,
    data3: 0x43B0,
    data4: [0xB5, 0xB4, 0x2D, 0x72, 0xE5, 0x4E, 0xAA, 0xA4],
};

/// {CAC52C1A-B53D-4edc-92D7-6B2E8AC19434} - deprecated
#[allow(non_upper_case_globals)]
pub const FOLDERID_Games: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xCAC52C1A,
    data2: 0xB53D,
    data3: 0x4EDC,
    data4: [0x92, 0xD7, 0x6B, 0x2E, 0x8A, 0xC1, 0x94, 0x34],
};

/// {98ec0e18-2098-4d44-8644-66979315a281}
pub const FOLDERID_SEARCH_MAPI: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x98EC0E18,
    data2: 0x2098,
    data3: 0x4D44,
    data4: [0x86, 0x44, 0x66, 0x97, 0x93, 0x15, 0xA2, 0x81],
};

/// {ee32e446-31ca-4aba-814f-a5ebd2fd6d5e}
pub const FOLDERID_SEARCH_CSC: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xEE32E446,
    data2: 0x31CA,
    data3: 0x4ABA,
    data4: [0x81, 0x4F, 0xA5, 0xEB, 0xD2, 0xFD, 0x6D, 0x5E],
};

/// {bfb9d5e0-c6a9-404c-b2b2-ae6db6af4968}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Links: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xBFB9D5E0,
    data2: 0xC6A9,
    data3: 0x404C,
    data4: [0xB2, 0xB2, 0xAE, 0x6D, 0xB6, 0xAF, 0x49, 0x68],
};

/// {f3ce0f7c-4901-4acc-8648-d5d44b04ef8f}
#[allow(non_upper_case_globals)]
pub const FOLDERID_UsersFiles: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xF3CE0F7C,
    data2: 0x4901,
    data3: 0x4ACC,
    data4: [0x86, 0x48, 0xD5, 0xD4, 0x4B, 0x04, 0xEF, 0x8F],
};

/// {A302545D-DEFF-464b-ABE8-61C8648D939B}
#[allow(non_upper_case_globals)]
pub const FOLDERID_UsersLibraries: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA302545D,
    data2: 0xDEFF,
    data3: 0x464B,
    data4: [0xAB, 0xE8, 0x61, 0xC8, 0x64, 0x8D, 0x93, 0x9B],
};

/// {190337d1-b8ca-4121-a639-6d472d16972a}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SearchHome: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x190337D1,
    data2: 0xB8CA,
    data3: 0x4121,
    data4: [0xA6, 0x39, 0x6D, 0x47, 0x2D, 0x16, 0x97, 0x2A],
};

/// {2C36C0AA-5812-4b87-BFD0-4CD0DFB19B39}
#[allow(non_upper_case_globals)]
pub const FOLDERID_OriginalImages: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x2C36C0AA,
    data2: 0x5812,
    data3: 0x4B87,
    data4: [0xBF, 0xD0, 0x4C, 0xD0, 0xDF, 0xB1, 0x9B, 0x39],
};

/// {7b0db17d-9cd2-4a93-9733-46cc89022e7c}
#[allow(non_upper_case_globals)]
pub const FOLDERID_DocumentsLibrary: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x7B0DB17D,
    data2: 0x9CD2,
    data3: 0x4A93,
    data4: [0x97, 0x33, 0x46, 0xCC, 0x89, 0x02, 0x2E, 0x7C],
};

/// {2112AB0A-C86A-4ffe-A368-0DE96E47012E}
#[allow(non_upper_case_globals)]
pub const FOLDERID_MusicLibrary: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x2112AB0A,
    data2: 0xC86A,
    data3: 0x4FFE,
    data4: [0xA3, 0x68, 0xD, 0xE9, 0x6E, 0x47, 0x1, 0x2E],
};

/// {A990AE9F-A03B-4e80-94BC-9912D7504104}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PicturesLibrary: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA990AE9F,
    data2: 0xA03B,
    data3: 0x4E80,
    data4: [0x94, 0xBC, 0x99, 0x12, 0xD7, 0x50, 0x41, 0x4],
};

/// {491E922F-5643-4af4-A7EB-4E7A138D8174}
#[allow(non_upper_case_globals)]
pub const FOLDERID_VideosLibrary: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x491E922F,
    data2: 0x5643,
    data3: 0x4AF4,
    data4: [0xA7, 0xEB, 0x4E, 0x7A, 0x13, 0x8D, 0x81, 0x74],
};

/// {1A6FDBA2-F42D-4358-A798-B74D745926C5}
#[allow(non_upper_case_globals)]
pub const FOLDERID_RecordedTVLibrary: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x1A6FDBA2,
    data2: 0xF42D,
    data3: 0x4358,
    data4: [0xA7, 0x98, 0xB7, 0x4D, 0x74, 0x59, 0x26, 0xC5],
};

/// {52528A6B-B9E3-4add-B60D-588C2DBA842D}
#[allow(non_upper_case_globals)]
pub const FOLDERID_HomeGroup: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x52528A6B,
    data2: 0xB9E3,
    data3: 0x4ADD,
    data4: [0xB6, 0xD, 0x58, 0x8C, 0x2D, 0xBA, 0x84, 0x2D],
};

/// {9B74B6A3-0DFD-4f11-9E78-5F7800F2E772}
#[allow(non_upper_case_globals)]
pub const FOLDERID_HomeGroupCurrentUser: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x9B74B6A3,
    data2: 0xDFD,
    data3: 0x4F11,
    data4: [0x9E, 0x78, 0x5F, 0x78, 0x0, 0xF2, 0xE7, 0x72],
};

/// {5CE4A5E9-E4EB-479D-B89F-130C02886155}
#[allow(non_upper_case_globals)]
pub const FOLDERID_DeviceMetadataStore: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x5CE4A5E9,
    data2: 0xE4EB,
    data3: 0x479D,
    data4: [0xB8, 0x9F, 0x13, 0x0C, 0x02, 0x88, 0x61, 0x55],
};

/// {1B3EA5DC-B587-4786-B4EF-BD1DC332AEAE}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Libraries: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x1B3EA5DC,
    data2: 0xB587,
    data3: 0x4786,
    data4: [0xB4, 0xEF, 0xBD, 0x1D, 0xC3, 0x32, 0xAE, 0xAE],
};

/// {48daf80b-e6cf-4f4e-b800-0e69d84ee384}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PublicLibraries: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x48DAF80B,
    data2: 0xE6CF,
    data3: 0x4F4E,
    data4: [0xB8, 0x00, 0x0E, 0x69, 0xD8, 0x4E, 0xE3, 0x84],
};

/// {9e3995ab-1f9c-4f13-b827-48b24b6c7174}
#[allow(non_upper_case_globals)]
pub const FOLDERID_UserPinned: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x9E3995AB,
    data2: 0x1F9C,
    data3: 0x4F13,
    data4: [0xB8, 0x27, 0x48, 0xB2, 0x4B, 0x6C, 0x71, 0x74],
};

/// {bcb5256f-79f6-4cee-b725-dc34e402fd46}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ImplicitAppShortcuts: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xBCB5256F,
    data2: 0x79F6,
    data3: 0x4CEE,
    data4: [0xB7, 0x25, 0xDC, 0x34, 0xE4, 0x2, 0xFD, 0x46],
};

/// {008ca0b1-55b4-4c56-b8a8-4de4b299d3be}
#[allow(non_upper_case_globals)]
pub const FOLDERID_AccountPictures: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x008CA0B1,
    data2: 0x55B4,
    data3: 0x4C56,
    data4: [0xB8, 0xA8, 0x4D, 0xE4, 0xB2, 0x99, 0xD3, 0xBE],
};

/// {0482af6c-08f1-4c34-8c90-e17ec98b1e17}
#[allow(non_upper_case_globals)]
pub const FOLDERID_PublicUserTiles: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x0482AF6C,
    data2: 0x08F1,
    data3: 0x4C34,
    data4: [0x8C, 0x90, 0xE1, 0x7E, 0xC9, 0x8B, 0x1E, 0x17],
};

/// {1e87508d-89c2-42f0-8a7e-645a0f50ca58}
#[allow(non_upper_case_globals)]
pub const FOLDERID_AppsFolder: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x1E87508D,
    data2: 0x89C2,
    data3: 0x42F0,
    data4: [0x8A, 0x7E, 0x64, 0x5A, 0x0F, 0x50, 0xCA, 0x58],
};

/// {F26305EF-6948-40B9-B255-81453D09C785}
#[allow(non_upper_case_globals)]
pub const FOLDERID_StartMenuAllPrograms: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xF26305EF,
    data2: 0x6948,
    data3: 0x40B9,
    data4: [0xB2, 0x55, 0x81, 0x45, 0x3D, 0x9, 0xC7, 0x85],
};

/// {A440879F-87A0-4F7D-B700-0207B966194A}
#[allow(non_upper_case_globals)]
pub const FOLDERID_CommonStartMenuPlaces: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA440879F,
    data2: 0x87A0,
    data3: 0x4F7D,
    data4: [0xB7, 0x0, 0x2, 0x7, 0xB9, 0x66, 0x19, 0x4A],
};

/// {A3918781-E5F2-4890-B3D9-A7E54332328C}
#[allow(non_upper_case_globals)]
pub const FOLDERID_ApplicationShortcuts: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA3918781,
    data2: 0xE5F2,
    data3: 0x4890,
    data4: [0xB3, 0xD9, 0xA7, 0xE5, 0x43, 0x32, 0x32, 0x8C],
};

/// {00BCFC5A-ED94-4e48-96A1-3F6217F21990}
#[allow(non_upper_case_globals)]
pub const FOLDERID_RoamingTiles: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xBCFC5A,
    data2: 0xED94,
    data3: 0x4E48,
    data4: [0x96, 0xA1, 0x3F, 0x62, 0x17, 0xF2, 0x19, 0x90],
};

/// {AAA8D5A5-F1D6-4259-BAA8-78E7EF60835E}
#[allow(non_upper_case_globals)]
pub const FOLDERID_RoamedTileImages: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xAAA8D5A5,
    data2: 0xF1D6,
    data3: 0x4259,
    data4: [0xBA, 0xA8, 0x78, 0xE7, 0xEF, 0x60, 0x83, 0x5E],
};

/// {b7bede81-df94-4682-a7d8-57a52620b86f}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Screenshots: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xB7BEDE81,
    data2: 0xDF94,
    data3: 0x4682,
    data4: [0xA7, 0xD8, 0x57, 0xA5, 0x26, 0x20, 0xB8, 0x6F],
};

/// {AB5FB87B-7CE2-4F83-915D-550846C9537B}
#[allow(non_upper_case_globals)]
pub const FOLDERID_CameraRoll: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xAB5FB87B,
    data2: 0x7CE2,
    data3: 0x4F83,
    data4: [0x91, 0x5D, 0x55, 0x8, 0x46, 0xC9, 0x53, 0x7B],
};

/// {A52BBA46-E9E1-435f-B3D9-28DAA648C0F6} - deprecated
///
/// Same KNOWNFOLDERID as FOLDERID_OneDrive
#[allow(non_upper_case_globals)]
pub const FOLDERID_SkyDrive: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA52BBA46,
    data2: 0xE9E1,
    data3: 0x435F,
    data4: [0xB3, 0xD9, 0x28, 0xDA, 0xA6, 0x48, 0xC0, 0xF6],
};

/// {A52BBA46-E9E1-435f-B3D9-28DAA648C0F6}
///
/// Same KNOWNFOLDERID as FOLDERID_SkyDrive
#[allow(non_upper_case_globals)]
pub const FOLDERID_OneDrive: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA52BBA46,
    data2: 0xE9E1,
    data3: 0x435F,
    data4: [0xB3, 0xD9, 0x28, 0xDA, 0xA6, 0x48, 0xC0, 0xF6],
};

/// {24D89E24-2F19-4534-9DDE-6A6671FBB8FE}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SkyDriveDocuments: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x24D89E24,
    data2: 0x2F19,
    data3: 0x4534,
    data4: [0x9D, 0xDE, 0x6A, 0x66, 0x71, 0xFB, 0xB8, 0xFE],
};

/// {339719B5-8C47-4894-94C2-D8F77ADD44A6}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SkyDrivePictures: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x339719B5,
    data2: 0x8C47,
    data3: 0x4894,
    data4: [0x94, 0xC2, 0xD8, 0xF7, 0x7A, 0xDD, 0x44, 0xA6],
};

/// {C3F2459E-80D6-45DC-BFEF-1F769F2BE730}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SkyDriveMusic: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xC3F2459E,
    data2: 0x80D6,
    data3: 0x45DC,
    data4: [0xBF, 0xEF, 0x1F, 0x76, 0x9F, 0x2B, 0xE7, 0x30],
};

/// {767E6811-49CB-4273-87C2-20F355E1085B}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SkyDriveCameraRoll: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x767E6811,
    data2: 0x49CB,
    data3: 0x4273,
    data4: [0x87, 0xC2, 0x20, 0xF3, 0x55, 0xE1, 0x08, 0x5B],
};

/// {0D4C3DB6-03A3-462F-A0E6-08924C41B5D4}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SearchHistory: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x0D4C3DB6,
    data2: 0x03A3,
    data3: 0x462F,
    data4: [0xA0, 0xE6, 0x08, 0x92, 0x4C, 0x41, 0xB5, 0xD4],
};

/// {7E636BFE-DFA9-4D5E-B456-D7B39851D8A9}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SearchTemplates: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x7E636BFE,
    data2: 0xDFA9,
    data3: 0x4D5E,
    data4: [0xB4, 0x56, 0xD7, 0xB3, 0x98, 0x51, 0xD8, 0xA9],
};

/// {2B20DF75-1EDA-4039-8097-38798227D5B7}
#[allow(non_upper_case_globals)]
pub const FOLDERID_CameraRollLibrary: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x2B20DF75,
    data2: 0x1EDA,
    data3: 0x4039,
    data4: [0x80, 0x97, 0x38, 0x79, 0x82, 0x27, 0xD5, 0xB7],
};

/// {3B193882-D3AD-4eab-965A-69829D1FB59F}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SavedPictures: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x3B193882,
    data2: 0xD3AD,
    data3: 0x4EAB,
    data4: [0x96, 0x5A, 0x69, 0x82, 0x9D, 0x1F, 0xB5, 0x9F],
};

/// {E25B5812-BE88-4bd9-94B0-29233477B6C3}
#[allow(non_upper_case_globals)]
pub const FOLDERID_SavedPicturesLibrary: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xE25B5812,
    data2: 0xBE88,
    data3: 0x4BD9,
    data4: [0x94, 0xB0, 0x29, 0x23, 0x34, 0x77, 0xB6, 0xC3],
};

/// {12D4C69E-24AD-4923-BE19-31321C43A767}
#[allow(non_upper_case_globals)]
pub const FOLDERID_RetailDemo: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x12D4C69E,
    data2: 0x24AD,
    data3: 0x4923,
    data4: [0xBE, 0x19, 0x31, 0x32, 0x1C, 0x43, 0xA7, 0x67],
};

/// {1C2AC1DC-4358-4B6C-9733-AF21156576F0}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Device: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x1C2AC1DC,
    data2: 0x4358,
    data3: 0x4B6C,
    data4: [0x97, 0x33, 0xAF, 0x21, 0x15, 0x65, 0x76, 0xF0],
};

/// The file system directory that contains development files that have been copied to this device
/// by a development tool. A typical path is C:\Users\username\AppData\Local\DevelopmentFiles. This
/// directory is used by development tools that need to copy files to a device. This may include
/// copying application binaries for temporary registration and execution in order to allow a
/// developer to test their application without having to go through the full app packaging
/// process. It could also include development time only components such as a remote debugger.
/// Recommended practice is to create sub-directories rather than copying files to the
/// DevelopmentFiles directory. Development tools should be careful to use a naming convention that
/// avoids conflicts. For example application binaries should be copied to a directory with a
/// unique name such as the app package full name. This information is per user and will not roam.
///
/// {DBE8E08E-3053-4BBC-B183-2A7B2B191E59}
#[allow(non_upper_case_globals)]
pub const FOLDERID_DevelopmentFiles: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xDBE8E08E,
    data2: 0x3053,
    data3: 0x4BBC,
    data4: [0xB1, 0x83, 0x2A, 0x7B, 0x2B, 0x19, 0x1E, 0x59],
};

/// {31C0DD25-9439-4F12-BF41-7FF4EDA38722}
#[allow(non_upper_case_globals)]
pub const FOLDERID_Objects3D: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x31C0DD25,
    data2: 0x9439,
    data3: 0x4F12,
    data4: [0xBF, 0x41, 0x7F, 0xF4, 0xED, 0xA3, 0x87, 0x22],
};

/// {EDC0FE71-98D8-4F4A-B920-C8DC133CB165}
#[allow(non_upper_case_globals)]
pub const FOLDERID_AppCaptures: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xEDC0FE71,
    data2: 0x98D8,
    data3: 0x4F4A,
    data4: [0xB9, 0x20, 0xC8, 0xDC, 0x13, 0x3C, 0xB1, 0x65],
};

/// {f42ee2d3-909f-4907-8871-4c22fc0bf756}
#[allow(non_upper_case_globals)]
pub const FOLDERID_LocalDocuments: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xF42EE2D3,
    data2: 0x909F,
    data3: 0x4907,
    data4: [0x88, 0x71, 0x4C, 0x22, 0xFC, 0x0B, 0xF7, 0x56],
};

/// {0ddd015d-b06c-45d5-8c4c-f59713854639 }
#[allow(non_upper_case_globals)]
pub const FOLDERID_LocalPictures: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x0DDD015D,
    data2: 0xB06C,
    data3: 0x45D5,
    data4: [0x8C, 0x4C, 0xF5, 0x97, 0x13, 0x85, 0x46, 0x39],
};

/// {35286a68-3c57-41a1-bbb1-0eae73d76c95}
#[allow(non_upper_case_globals)]
pub const FOLDERID_LocalVideos: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x35286A68,
    data2: 0x3C57,
    data3: 0x41A1,
    data4: [0xBB, 0xB1, 0x0E, 0xAE, 0x73, 0xD7, 0x6C, 0x95],
};

/// {a0c69a99-21c8-4671-8703-7934162fcf1d}
#[allow(non_upper_case_globals)]
pub const FOLDERID_LocalMusic: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xA0C69A99,
    data2: 0x21C8,
    data3: 0x4671,
    data4: [0x87, 0x03, 0x79, 0x34, 0x16, 0x2F, 0xCF, 0x1D],
};

/// {7d83ee9b-2244-4e70-b1f5-5393042af1e4}
#[allow(non_upper_case_globals)]
pub const FOLDERID_LocalDownloads: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x7D83EE9B,
    data2: 0x2244,
    data3: 0x4E70,
    data4: [0xB1, 0xF5, 0x53, 0x93, 0x04, 0x2A, 0xF1, 0xE4],
};

/// {2f8b40c2-83ed-48ee-b383-a1f157ec6f9a}
#[allow(non_upper_case_globals)]
pub const FOLDERID_RecordedCalls: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x2F8B40C2,
    data2: 0x83ED,
    data3: 0x48EE,
    data4: [0xB3, 0x83, 0xA1, 0xF1, 0x57, 0xEC, 0x6F, 0x9A],
};

/// {7ad67899-66af-43ba-9156-6aad42e6c596}
#[allow(non_upper_case_globals)]
pub const FOLDERID_AllAppMods: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x7AD67899,
    data2: 0x66AF,
    data3: 0x43BA,
    data4: [0x91, 0x56, 0x6A, 0xAD, 0x42, 0xE6, 0xC5, 0x96],
};

/// {3db40b20-2a30-4dbe-917e-771dd21dd099}
#[allow(non_upper_case_globals)]
pub const FOLDERID_CurrentAppMods: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x3DB40B20,
    data2: 0x2A30,
    data3: 0x4DBE,
    data4: [0x91, 0x7E, 0x77, 0x1D, 0xD2, 0x1D, 0xD0, 0x99],
};

/// {B2C5E279-7ADD-439F-B28C-C41FE1BBF672}
#[allow(non_upper_case_globals)]
pub const FOLDERID_AppDataDesktop: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xB2C5E279,
    data2: 0x7ADD,
    data3: 0x439F,
    data4: [0xB2, 0x8C, 0xC4, 0x1F, 0xE1, 0xBB, 0xF6, 0x72],
};

/// {7BE16610-1F7F-44AC-BFF0-83E15F2FFCA1}
#[allow(non_upper_case_globals)]
pub const FOLDERID_AppDataDocuments: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x7BE16610,
    data2: 0x1F7F,
    data3: 0x44AC,
    data4: [0xBF, 0xF0, 0x83, 0xE1, 0x5F, 0x2F, 0xFC, 0xA1],
};

/// {7CFBEFBC-DE1F-45AA-B843-A542AC536CC9}
#[allow(non_upper_case_globals)]
pub const FOLDERID_AppDataFavorites: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x7CFBEFBC,
    data2: 0xDE1F,
    data3: 0x45AA,
    data4: [0xB8, 0x43, 0xA5, 0x42, 0xAC, 0x53, 0x6C, 0xC9],
};

/// {559D40A3-A036-40FA-AF61-84CB430A4D34}
#[allow(non_upper_case_globals)]
pub const FOLDERID_AppDataProgramData: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0x559D40A3,
    data2: 0xA036,
    data3: 0x40FA,
    data4: [0xAF, 0x61, 0x84, 0xCB, 0x43, 0xA, 0x4D, 0x34],
};

/// {B3EB08D3-A1F3-496B-865A-42B536CDA0EC}
#[allow(non_upper_case_globals)]
pub const FOLDERID_LocalStorage: KNOWNFOLDERID = KNOWNFOLDERID {
    data1: 0xB3EB08D3,
    data2: 0xA1F3,
    data3: 0x496B,
    data4: [0x86, 0x5A, 0x42, 0xB5, 0x36, 0xCD, 0xA0, 0xEC],
};
