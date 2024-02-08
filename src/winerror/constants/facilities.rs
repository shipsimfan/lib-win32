use crate::HRESULT;

/// Null
pub const FACILITY_NULL: HRESULT = 0;

/// RPC
pub const FACILITY_RPC: HRESULT = 1;

/// Dispatch
pub const FACILITY_DISPATCH: HRESULT = 2;

/// Storage
pub const FACILITY_STORAGE: HRESULT = 3;

/// ITF
pub const FACILITY_ITF: HRESULT = 4;

/// Win32
pub const FACILITY_WIN32: HRESULT = 7;

/// Windows
pub const FACILITY_WINDOWS: HRESULT = 8;

/// SSPI
pub const FACILITY_SSPI: HRESULT = 9;

/// Security
pub const FACILITY_SECURITY: HRESULT = 9;

/// Control
pub const FACILITY_CONTROL: HRESULT = 10;

/// Certificate
pub const FACILITY_CERT: HRESULT = 11;

/// Internet
pub const FACILITY_INTERNET: HRESULT = 12;

/// Media Server
pub const FACILITY_MEDIASERVER: HRESULT = 13;

/// MSMQ
pub const FACILITY_MSMQ: HRESULT = 14;

/// Setup API
pub const FACILITY_SETUPAPI: HRESULT = 15;

/// SCard
pub const FACILITY_SCARD: HRESULT = 16;

/// COM Plus
pub const FACILITY_COMPLUS: HRESULT = 17;

/// AAF
pub const FACILITY_AAF: HRESULT = 18;

/// URT
pub const FACILITY_URT: HRESULT = 19;

/// ACS
pub const FACILITY_ACS: HRESULT = 20;

/// DPlay
pub const FACILITY_DPLAY: HRESULT = 21;

/// UMI
pub const FACILITY_UMI: HRESULT = 22;

/// SXS
pub const FACILITY_SXS: HRESULT = 23;

/// Windows CE
pub const FACILITY_WINDOWS_CE: HRESULT = 24;

/// HTTP
pub const FACILITY_HTTP: HRESULT = 25;

/// Usermode Common Log
pub const FACILITY_USERMODE_COMMONLOG: HRESULT = 26;

/// WER
pub const FACILITY_WER: HRESULT = 27;

/// Usermode Filter Manager
pub const FACILITY_USERMODE_FILTER_MANAGER: HRESULT = 31;

/// Background Copy
pub const FACILITY_BACKGROUNDCOPY: HRESULT = 32;

/// Configuration
pub const FACILITY_CONFIGURATION: HRESULT = 33;

/// WIA
pub const FACILITY_WIA: HRESULT = 33;

/// State Management
pub const FACILITY_STATE_MANAGEMENT: HRESULT = 34;

/// Meta Directory
pub const FACILITY_METADIRECTORY: HRESULT = 35;

/// Windows Update
pub const FACILITY_WINDOWSUPDATE: HRESULT = 36;

/// Directory Service
pub const FACILITY_DIRECTORYSERVICE: HRESULT = 37;

/// Graphics
pub const FACILITY_GRAPHICS: HRESULT = 38;

/// Shell
pub const FACILITY_SHELL: HRESULT = 39;

/// NAP
pub const FACILITY_NAP: HRESULT = 39;

/// TPM Services
pub const FACILITY_TPM_SERVICES: HRESULT = 40;

/// TPM Software
pub const FACILITY_TPM_SOFTWARE: HRESULT = 41;

/// UI
pub const FACILITY_UI: HRESULT = 42;

/// XAML
pub const FACILITY_XAML: HRESULT = 43;

/// Action Queue
pub const FACILITY_ACTION_QUEUE: HRESULT = 44;

/// PLA
pub const FACILITY_PLA: HRESULT = 48;

/// Windows Setup
pub const FACILITY_WINDOWS_SETUP: HRESULT = 48;

/// FVE
pub const FACILITY_FVE: HRESULT = 49;

/// FWP
pub const FACILITY_FWP: HRESULT = 50;

/// WinRM
pub const FACILITY_WINRM: HRESULT = 51;

/// NDIS
pub const FACILITY_NDIS: HRESULT = 52;

/// Usermode Hypervisor
pub const FACILITY_USERMODE_HYPERVISOR: HRESULT = 53;

/// CMI
pub const FACILITY_CMI: HRESULT = 54;

/// Usermode Virtualization
pub const FACILITY_USERMODE_VIRTUALIZATION: HRESULT = 55;

/// Usermode Volume Manager
pub const FACILITY_USERMODE_VOLMGR: HRESULT = 56;

/// BCD
pub const FACILITY_BCD: HRESULT = 57;

/// Usermode VHD
pub const FACILITY_USERMODE_VHD: HRESULT = 58;

/// Usermode HNS
pub const FACILITY_USERMODE_HNS: HRESULT = 59;

/// SDiag
pub const FACILITY_SDIAG: HRESULT = 60;

/// Web Services
pub const FACILITY_WEBSERVICES: HRESULT = 61;

/// WinPE
pub const FACILITY_WINPE: HRESULT = 61;

/// WPN
pub const FACILITY_WPN: HRESULT = 62;

/// Windows Store
pub const FACILITY_WINDOWS_STORE: HRESULT = 63;

/// Input
pub const FACILITY_INPUT: HRESULT = 64;

/// QUIC
pub const FACILITY_QUIC: HRESULT = 65;

/// EAP
pub const FACILITY_EAP: HRESULT = 66;

/// I/O Ring
pub const FACILITY_IORING: HRESULT = 70;

/// Windows Defender
pub const FACILITY_WINDOWS_DEFENDER: HRESULT = 80;

/// OPC
pub const FACILITY_OPC: HRESULT = 81;

/// XPS
pub const FACILITY_XPS: HRESULT = 82;

/// MBN
pub const FACILITY_MBN: HRESULT = 84;

/// Powershell
pub const FACILITY_POWERSHELL: HRESULT = 84;

/// RAS
pub const FACILITY_RAS: HRESULT = 83;

/// P2P INT
pub const FACILITY_P2P_INT: HRESULT = 98;

/// P2P
pub const FACILITY_P2P: HRESULT = 99;

/// DAF
pub const FACILITY_DAF: HRESULT = 100;

/// Bluetooth ATT
pub const FACILITY_BLUETOOTH_ATT: HRESULT = 101;

/// Audio
pub const FACILITY_AUDIO: HRESULT = 102;

/// State Repository
pub const FACILITY_STATEREPOSITORY: HRESULT = 103;

/// VisualC++
pub const FACILITY_VISUALCPP: HRESULT = 109;

/// Script
pub const FACILITY_SCRIPT: HRESULT = 112;

/// Parse
pub const FACILITY_PARSE: HRESULT = 113;

/// BLB
pub const FACILITY_BLB: HRESULT = 120;

/// BLB CLI
pub const FACILITY_BLB_CLI: HRESULT = 121;

/// WSB App
pub const FACILITY_WSBAPP: HRESULT = 122;

/// BLB UI
pub const FACILITY_BLBUI: HRESULT = 128;

/// USN
pub const FACILITY_USN: HRESULT = 129;

/// Usermode VolSnap
pub const FACILITY_USERMODE_VOLSNAP: HRESULT = 130;

/// Tiering
pub const FACILITY_TIERING: HRESULT = 131;

/// WSB Online
pub const FACILITY_WSB_ONLINE: HRESULT = 133;

/// Online ID
pub const FACILITY_ONLINE_ID: HRESULT = 134;

/// Device Update Agent
pub const FACILITY_DEVICE_UPDATE_AGENT: HRESULT = 135;

/// DRV Servicing
pub const FACILITY_DRVSERVICING: HRESULT = 136;

/// DLS
pub const FACILITY_DLS: HRESULT = 153;

/// Delivery Optimization
pub const FACILITY_DELIVERY_OPTIMIZATION: HRESULT = 208;

/// Usermode Spaces
pub const FACILITY_USERMODE_SPACES: HRESULT = 231;

/// Usermode Security Core
pub const FACILITY_USER_MODE_SECURITY_CORE: HRESULT = 232;

/// Usermode Licensing
pub const FACILITY_USERMODE_LICENSING: HRESULT = 234;

/// SOS
pub const FACILITY_SOS: HRESULT = 160;

/// OCP Update Agent
pub const FACILITY_OCP_UPDATE_AGENT: HRESULT = 173;

/// Debuggers
pub const FACILITY_DEBUGGERS: HRESULT = 176;

/// SPP
pub const FACILITY_SPP: HRESULT = 256;

/// Restore
pub const FACILITY_RESTORE: HRESULT = 256;

/// DM Server
pub const FACILITY_DMSERVER: HRESULT = 256;

/// Deployment Services Server
pub const FACILITY_DEPLOYMENT_SERVICES_SERVER: HRESULT = 257;

/// Deployment Services Imaging
pub const FACILITY_DEPLOYMENT_SERVICES_IMAGING: HRESULT = 258;

/// Deployment Services Management
pub const FACILITY_DEPLOYMENT_SERVICES_MANAGEMENT: HRESULT = 259;

/// Deployment Services Util
pub const FACILITY_DEPLOYMENT_SERVICES_UTIL: HRESULT = 260;

/// Deployment Services BINL Service
pub const FACILITY_DEPLOYMENT_SERVICES_BINLSVC: HRESULT = 261;

/// Deployment Services PXE
pub const FACILITY_DEPLOYMENT_SERVICES_PXE: HRESULT = 263;

/// Deployment Services TFTP
pub const FACILITY_DEPLOYMENT_SERVICES_TFTP: HRESULT = 264;

/// Deployment Services Transport Management
pub const FACILITY_DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT: HRESULT = 272;

/// Deployment Services Driver Provisioning
pub const FACILITY_DEPLOYMENT_SERVICES_DRIVER_PROVISIONING: HRESULT = 278;

/// Deployment Services Multicast Server
pub const FACILITY_DEPLOYMENT_SERVICES_MULTICAST_SERVER: HRESULT = 289;

/// Deployment Services Multicast Client
pub const FACILITY_DEPLOYMENT_SERVICES_MULTICAST_CLIENT: HRESULT = 290;

/// Deployment Services Content Provider
pub const FACILITY_DEPLOYMENT_SERVICES_CONTENT_PROVIDER: HRESULT = 293;

/// HSP Services
pub const FACILITY_HSP_SERVICES: HRESULT = 296;

/// HSP Software
pub const FACILITY_HSP_SOFTWARE: HRESULT = 297;

/// Linguistic Services
pub const FACILITY_LINGUISTIC_SERVICES: HRESULT = 305;

/// Audio Streaming
pub const FACILITY_AUDIOSTREAMING: HRESULT = 1094;

/// TTD
pub const FACILITY_TTD: HRESULT = 1490;

/// Accelerator
pub const FACILITY_ACCELERATOR: HRESULT = 1536;

/// WMAAECMA
pub const FACILITY_WMAAECMA: HRESULT = 1996;

/// Direct Music
pub const FACILITY_DIRECTMUSIC: HRESULT = 2168;

/// Direct 3D10
pub const FACILITY_DIRECT3D10: HRESULT = 2169;

/// DXGI
pub const FACILITY_DXGI: HRESULT = 2170;

/// DXGI DDI
pub const FACILITY_DXGI_DDI: HRESULT = 2171;

/// Direct3D11
pub const FACILITY_DIRECT3D11: HRESULT = 2172;

/// Direct3D11 Debug
pub const FACILITY_DIRECT3D11_DEBUG: HRESULT = 2173;

/// Direct3D12
pub const FACILITY_DIRECT3D12: HRESULT = 2174;

/// Direct3D12 Debug
pub const FACILITY_DIRECT3D12_DEBUG: HRESULT = 2175;

/// DX Core
pub const FACILITY_DXCORE: HRESULT = 2176;

/// Presentation
pub const FACILITY_PRESENTATION: HRESULT = 2177;

/// Leap
pub const FACILITY_LEAP: HRESULT = 2184;

/// AUDCLNT
pub const FACILITY_AUDCLNT: HRESULT = 2185;

/// Wincodec DWrite DWM
pub const FACILITY_WINCODEC_DWRITE_DWM: HRESULT = 2200;

/// WinML
pub const FACILITY_WINML: HRESULT = 2192;

/// Direct2D
pub const FACILITY_DIRECT2D: HRESULT = 2201;

/// Defrag
pub const FACILITY_DEFRAG: HRESULT = 2304;

/// Usermode SDBus
pub const FACILITY_USERMODE_SDBUS: HRESULT = 2305;

/// JScript
pub const FACILITY_JSCRIPT: HRESULT = 2306;

/// PidgenX
pub const FACILITY_PIDGENX: HRESULT = 2561;

/// EAS
pub const FACILITY_EAS: HRESULT = 85;

/// Web
pub const FACILITY_WEB: HRESULT = 885;

/// Web Socket
pub const FACILITY_WEB_SOCKET: HRESULT = 886;

/// Mobile
pub const FACILITY_MOBILE: HRESULT = 1793;

/// SQLite
pub const FACILITY_SQLITE: HRESULT = 1967;

/// Service Fabric
pub const FACILITY_SERVICE_FABRIC: HRESULT = 1968;

/// UTC
pub const FACILITY_UTC: HRESULT = 1989;

/// WEP
pub const FACILITY_WEP: HRESULT = 2049;

/// Sync Engine
pub const FACILITY_SYNCENGINE: HRESULT = 2050;

/// X-Box
pub const FACILITY_XBOX: HRESULT = 2339;

/// Game
pub const FACILITY_GAME: HRESULT = 2340;

/// PIX
pub const FACILITY_PIX: HRESULT = 2748;
