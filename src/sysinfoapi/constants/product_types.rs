use crate::DWORD;

/// Business
pub const PRODUCT_BUSINESS: DWORD = 0x00000006;

/// Business N
pub const PRODUCT_BUSINESS_N: DWORD = 0x00000010;

/// HPC Edition
pub const PRODUCT_CLUSTER_SERVER: DWORD = 0x00000012;

/// Server Hyper Core V
pub const PRODUCT_CLUSTER_SERVER_V: DWORD = 0x00000040;

/// Windows 10 Home
pub const PRODUCT_CORE: DWORD = 0x00000065;

/// Windows 10 Home China
pub const PRODUCT_CORE_COUNTRYSPECIFIC: DWORD = 0x00000063;

/// Windows 10 Home N
pub const PRODUCT_CORE_N: DWORD = 0x00000062;

/// Windows 10 Home Single Language
pub const PRODUCT_CORE_SINGLELANGUAGE: DWORD = 0x00000064;

/// Server Datacenter (evaluation installation)
pub const PRODUCT_DATACENTER_EVALUATION_SERVER: DWORD = 0x00000050;

/// Server Datacenter, Semi-Annual Channel (core installation)
pub const PRODUCT_DATACENTER_A_SERVER_CORE: DWORD = 0x00000091;

/// Server Standard, Semi-Annual Channel (core installation)
pub const PRODUCT_STANDARD_A_SERVER_CORE: DWORD = 0x00000092;

/// Server Datacenter (full installation. For Server Core installations of Windows Server 2012 and
/// later, use the method, Determining whether Server Core is running.)
pub const PRODUCT_DATACENTER_SERVER: DWORD = 0x00000008;

/// Server Datacenter (core installation, Windows Server 2008 R2 and earlier)
pub const PRODUCT_DATACENTER_SERVER_CORE: DWORD = 0x0000000C;

/// Server Datacenter without Hyper-V (core installation)
pub const PRODUCT_DATACENTER_SERVER_CORE_V: DWORD = 0x00000027;

/// Server Datacenter without Hyper-V (full installation)
pub const PRODUCT_DATACENTER_SERVER_V: DWORD = 0x00000025;

/// Windows 10 Education
pub const PRODUCT_EDUCATION: DWORD = 0x00000079;

/// Windows 10 Education N
pub const PRODUCT_EDUCATION_N: DWORD = 0x0000007A;

/// Windows 10 Enterprise
pub const PRODUCT_ENTERPRISE: DWORD = 0x00000004;

/// Windows 10 Enterprise E
pub const PRODUCT_ENTERPRISE_E: DWORD = 0x00000046;

/// Windows 10 Enterprise Evaluation
pub const PRODUCT_ENTERPRISE_EVALUATION: DWORD = 0x00000048;

/// Windows 10 Enterprise N
pub const PRODUCT_ENTERPRISE_N: DWORD = 0x0000001B;

/// Windows 10 Enterprise N Evaluation
pub const PRODUCT_ENTERPRISE_N_EVALUATION: DWORD = 0x00000054;

/// Windows 10 Enterprise 2015 LTSB
pub const PRODUCT_ENTERPRISE_S: DWORD = 0x0000007D;

/// Windows 10 Enterprise 2015 LTSB Evaluation
pub const PRODUCT_ENTERPRISE_S_EVALUATION: DWORD = 0x00000081;

/// Windows 10 Enterprise 2015 LTSB N
pub const PRODUCT_ENTERPRISE_S_N: DWORD = 0x0000007E;

/// Windows 10 Enterprise 2015 LTSB N Evaluation
pub const PRODUCT_ENTERPRISE_S_N_EVALUATION: DWORD = 0x00000082;

/// Server Enterprise (full installation)
pub const PRODUCT_ENTERPRISE_SERVER: DWORD = 0x0000000A;

/// Server Enterprise (core installation)
pub const PRODUCT_ENTERPRISE_SERVER_CORE: DWORD = 0x0000000E;

/// Server Enterprise without Hyper-V (core installation)
pub const PRODUCT_ENTERPRISE_SERVER_CORE_V: DWORD = 0x00000029;

/// Server Enterprise for Itanium-based Systems
pub const PRODUCT_ENTERPRISE_SERVER_IA64: DWORD = 0x0000000F;

/// Server Enterprise without Hyper-V (full installation)
pub const PRODUCT_ENTERPRISE_SERVER_V: DWORD = 0x00000026;

/// Windows Essential Server Solution Additional
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL: DWORD = 0x0000003C;

/// Windows Essential Server Solution Additional SVC
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC: DWORD = 0x0000003E;

/// Windows Essential Server Solution Management
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT: DWORD = 0x0000003B;

/// Windows Essential Server Solution Management SVC
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC: DWORD = 0x0000003D;

/// Home Basic
pub const PRODUCT_HOME_BASIC: DWORD = 0x00000002;

/// Not supported
pub const PRODUCT_HOME_BASIC_E: DWORD = 0x00000043;

/// Home Basic N
pub const PRODUCT_HOME_BASIC_N: DWORD = 0x00000005;

/// Home Premium
pub const PRODUCT_HOME_PREMIUM: DWORD = 0x00000003;

/// Not supported
pub const PRODUCT_HOME_PREMIUM_E: DWORD = 0x00000044;

/// Home Premium N
pub const PRODUCT_HOME_PREMIUM_N: DWORD = 0x0000001A;

/// Windows Home Server 2011
pub const PRODUCT_HOME_PREMIUM_SERVER: DWORD = 0x00000022;

/// Windows Storage Server 2008 R2 Essentials
pub const PRODUCT_HOME_SERVER: DWORD = 0x00000013;

/// Microsoft Hyper-V Server
pub const PRODUCT_HYPERV: DWORD = 0x0000002A;

/// Windows IoT Enterprise
pub const PRODUCT_IOTENTERPRISE: DWORD = 0x000000BC;

/// Windows IoT Enterprise LTSC
pub const PRODUCT_IOTENTERPRISE_S: DWORD = 0x000000BF;

/// Windows 10 IoT Core
pub const PRODUCT_IOTUAP: DWORD = 0x0000007B;

/// Windows 10 IoT Core Commercial
pub const PRODUCT_IOTUAPCOMMERCIAL: DWORD = 0x00000083;

/// Windows Essential Business Server Management Server
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT: DWORD = 0x0000001E;

/// Windows Essential Business Server Messaging Server
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING: DWORD = 0x00000020;

/// Windows Essential Business Server Security Server
pub const PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY: DWORD = 0x0000001F;

/// Windows 10 Mobile
pub const PRODUCT_MOBILE_CORE: DWORD = 0x00000068;

/// Windows 10 Mobile Enterprise
pub const PRODUCT_MOBILE_ENTERPRISE: DWORD = 0x00000085;

/// Windows MultiPoint Server Premium (full installation)
pub const PRODUCT_MULTIPOINT_PREMIUM_SERVER: DWORD = 0x0000004D;

/// Windows MultiPoint Server Standard (full installation)
pub const PRODUCT_MULTIPOINT_STANDARD_SERVER: DWORD = 0x0000004C;

/// Windows 10 Team
pub const PRODUCT_PPI_PRO: DWORD = 0x00000077;

/// Windows 10 Pro Education
pub const PRODUCT_PRO_FOR_EDUCATION: DWORD = 0x000000A4;

/// Windows 10 Pro for Workstations
pub const PRODUCT_PRO_WORKSTATION: DWORD = 0x000000A1;

/// Windows 10 Pro for Workstations N
pub const PRODUCT_PRO_WORKSTATION_N: DWORD = 0x000000A2;

/// Windows 10 Pro
pub const PRODUCT_PROFESSIONAL: DWORD = 0x00000030;

/// Not supported
pub const PRODUCT_PROFESSIONAL_E: DWORD = 0x00000045;

/// Windows 10 Pro N
pub const PRODUCT_PROFESSIONAL_N: DWORD = 0x00000031;

/// Professional with Media Center
pub const PRODUCT_PROFESSIONAL_WMC: DWORD = 0x00000067;

/// Windows Small Business Server 2011 Essentials
pub const PRODUCT_SB_SOLUTION_SERVER: DWORD = 0x00000032;

/// Server For SB Solutions EM
pub const PRODUCT_SB_SOLUTION_SERVER_EM: DWORD = 0x00000036;

/// Server For SB Solutions
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS: DWORD = 0x00000033;

/// Server For SB Solutions EM
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM: DWORD = 0x00000037;

/// Windows Server 2008 for Windows Essential Server Solutions
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS: DWORD = 0x00000018;

/// Windows Server 2008 without Hyper-V for Windows Essential Server Solutions
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS_V: DWORD = 0x00000023;

/// Server Foundation
pub const PRODUCT_SERVER_FOUNDATION: DWORD = 0x00000021;

/// Windows 10 Enterprise for Virtual Desktops
pub const PRODUCT_SERVERRDSH: DWORD = 0x000000AF;

/// Windows Small Business Server
pub const PRODUCT_SMALLBUSINESS_SERVER: DWORD = 0x00000009;

/// Small Business Server Premium
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM: DWORD = 0x00000019;

/// Small Business Server Premium (core installation)
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE: DWORD = 0x0000003F;

/// Windows MultiPoint Server
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER: DWORD = 0x00000038;

/// Server Standard (evaluation installation)
pub const PRODUCT_STANDARD_EVALUATION_SERVER: DWORD = 0x0000004F;

/// Server Standard (full installation. For Server Core installations of Windows Server 2012 and
/// later, use the method, Determining whether Server Core is running.)
pub const PRODUCT_STANDARD_SERVER: DWORD = 0x00000007;

/// Server Standard (core installation, Windows Server 2008 R2 and earlier)
pub const PRODUCT_STANDARD_SERVER_CORE: DWORD = 0x0000000D;

/// Server Standard without Hyper-V (core installation)
pub const PRODUCT_STANDARD_SERVER_CORE_V: DWORD = 0x00000028;

/// Server Standard without Hyper-V
pub const PRODUCT_STANDARD_SERVER_V: DWORD = 0x00000024;

/// Server Solutions Premium
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS: DWORD = 0x00000034;

/// Server Solutions Premium (core installation)
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE: DWORD = 0x00000035;

/// Starter
pub const PRODUCT_STARTER: DWORD = 0x0000000B;

/// Not supported
pub const PRODUCT_STARTER_E: DWORD = 0x00000042;

/// Starter N
pub const PRODUCT_STARTER_N: DWORD = 0x0000002F;

/// Storage Server Enterprise
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER: DWORD = 0x00000017;

/// Storage Server Enterprise (core installation)
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE: DWORD = 0x0000002E;

/// Storage Server Express
pub const PRODUCT_STORAGE_EXPRESS_SERVER: DWORD = 0x00000014;

/// Storage Server Express (core installation)
pub const PRODUCT_STORAGE_EXPRESS_SERVER_CORE: DWORD = 0x0000002B;

/// Storage Server Standard (evaluation installation)
pub const PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER: DWORD = 0x00000060;

/// Storage Server Standard
pub const PRODUCT_STORAGE_STANDARD_SERVER: DWORD = 0x00000015;

/// Storage Server Standard (core installation)
pub const PRODUCT_STORAGE_STANDARD_SERVER_CORE: DWORD = 0x0000002C;

/// Storage Server Workgroup (evaluation installation)
pub const PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER: DWORD = 0x0000005F;

/// Storage Server Workgroup
pub const PRODUCT_STORAGE_WORKGROUP_SERVER: DWORD = 0x00000016;

/// Storage Server Workgroup (core installation)
pub const PRODUCT_STORAGE_WORKGROUP_SERVER_CORE: DWORD = 0x0000002D;

/// Ultimate
pub const PRODUCT_ULTIMATE: DWORD = 0x00000001;

/// Not supported
pub const PRODUCT_ULTIMATE_E: DWORD = 0x00000047;

/// Ultimate N
pub const PRODUCT_ULTIMATE_N: DWORD = 0x0000001C;

/// An unknown product
pub const PRODUCT_UNDEFINED: DWORD = 0x00000000;

/// Web Server (full installation)
pub const PRODUCT_WEB_SERVER: DWORD = 0x00000011;

/// Web Server (core installation)
pub const PRODUCT_WEB_SERVER_CORE: DWORD = 0x0000001D;
