use crate::{BOOL, DWORD, PDWORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    PRODUCT_BUSINESS, PRODUCT_BUSINESS_N, PRODUCT_CLUSTER_SERVER, PRODUCT_CLUSTER_SERVER_V,
    PRODUCT_CORE, PRODUCT_CORE_COUNTRYSPECIFIC, PRODUCT_CORE_N, PRODUCT_CORE_SINGLELANGUAGE,
    PRODUCT_DATACENTER_A_SERVER_CORE, PRODUCT_DATACENTER_EVALUATION_SERVER,
    PRODUCT_DATACENTER_SERVER, PRODUCT_DATACENTER_SERVER_CORE, PRODUCT_DATACENTER_SERVER_CORE_V,
    PRODUCT_DATACENTER_SERVER_V, PRODUCT_EDUCATION, PRODUCT_EDUCATION_N, PRODUCT_ENTERPRISE,
    PRODUCT_ENTERPRISE_E, PRODUCT_ENTERPRISE_EVALUATION, PRODUCT_ENTERPRISE_N,
    PRODUCT_ENTERPRISE_N_EVALUATION, PRODUCT_ENTERPRISE_S, PRODUCT_ENTERPRISE_S_EVALUATION,
    PRODUCT_ENTERPRISE_S_N, PRODUCT_ENTERPRISE_S_N_EVALUATION, PRODUCT_ENTERPRISE_SERVER,
    PRODUCT_ENTERPRISE_SERVER_CORE, PRODUCT_ENTERPRISE_SERVER_CORE_V,
    PRODUCT_ENTERPRISE_SERVER_IA64, PRODUCT_ENTERPRISE_SERVER_V,
    PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL, PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC,
    PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT, PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC,
    PRODUCT_HOME_BASIC, PRODUCT_HOME_BASIC_E, PRODUCT_HOME_BASIC_N, PRODUCT_HOME_PREMIUM,
    PRODUCT_HOME_PREMIUM_E, PRODUCT_HOME_PREMIUM_N, PRODUCT_HOME_PREMIUM_SERVER,
    PRODUCT_HOME_SERVER, PRODUCT_HYPERV, PRODUCT_IOTENTERPRISE, PRODUCT_IOTENTERPRISE_S,
    PRODUCT_IOTUAP, PRODUCT_IOTUAPCOMMERCIAL, PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT,
    PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING, PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY,
    PRODUCT_MOBILE_CORE, PRODUCT_MOBILE_ENTERPRISE, PRODUCT_MULTIPOINT_PREMIUM_SERVER,
    PRODUCT_MULTIPOINT_STANDARD_SERVER, PRODUCT_PPI_PRO, PRODUCT_PRO_FOR_EDUCATION,
    PRODUCT_PRO_WORKSTATION, PRODUCT_PRO_WORKSTATION_N, PRODUCT_PROFESSIONAL,
    PRODUCT_PROFESSIONAL_E, PRODUCT_PROFESSIONAL_N, PRODUCT_PROFESSIONAL_WMC,
    PRODUCT_SB_SOLUTION_SERVER, PRODUCT_SB_SOLUTION_SERVER_EM, PRODUCT_SERVER_FOR_SB_SOLUTIONS,
    PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM, PRODUCT_SERVER_FOR_SMALLBUSINESS,
    PRODUCT_SERVER_FOR_SMALLBUSINESS_V, PRODUCT_SERVER_FOUNDATION, PRODUCT_SERVERRDSH,
    PRODUCT_SMALLBUSINESS_SERVER, PRODUCT_SMALLBUSINESS_SERVER_PREMIUM,
    PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE, PRODUCT_SOLUTION_EMBEDDEDSERVER,
    PRODUCT_STANDARD_A_SERVER_CORE, PRODUCT_STANDARD_EVALUATION_SERVER, PRODUCT_STANDARD_SERVER,
    PRODUCT_STANDARD_SERVER_CORE, PRODUCT_STANDARD_SERVER_CORE_V,
    PRODUCT_STANDARD_SERVER_SOLUTIONS, PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE,
    PRODUCT_STANDARD_SERVER_V, PRODUCT_STARTER, PRODUCT_STARTER_E, PRODUCT_STARTER_N,
    PRODUCT_STORAGE_ENTERPRISE_SERVER, PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE,
    PRODUCT_STORAGE_EXPRESS_SERVER, PRODUCT_STORAGE_EXPRESS_SERVER_CORE,
    PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER, PRODUCT_STORAGE_STANDARD_SERVER,
    PRODUCT_STORAGE_STANDARD_SERVER_CORE, PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER,
    PRODUCT_STORAGE_WORKGROUP_SERVER, PRODUCT_STORAGE_WORKGROUP_SERVER_CORE, PRODUCT_ULTIMATE,
    PRODUCT_ULTIMATE_E, PRODUCT_ULTIMATE_N, PRODUCT_UNDEFINED, PRODUCT_WEB_SERVER,
    PRODUCT_WEB_SERVER_CORE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

unsafe extern "C" {
    /// Retrieves the product type for the operating system on the local computer, and maps the
    /// type to the product types supported by the specified operating system.
    ///
    /// To retrieve product type information on versions of Windows prior to the minimum supported
    /// operating systems specified in the Requirements section, use the [`GetVersionEx`] function.
    /// You can also use the `OperatingSystemSKU` property of the `Win32_OperatingSystem` WMI
    /// class.
    ///
    /// # Parameters
    ///  * `os_major_version` - The major version number of the operating system. The minimum value
    ///                         is 6. The combination of the `os_major_version`,
    ///                         `os_minor_version`, `sp_major_version`, and `sp_minor_version`
    ///                         parameters describes the maximum target operating system version
    ///                         for the application. For example, Windows Vista and Windows Server
    ///                         2008 are version 6.0.0.0 and Windows 7 and Windows Server 2008 R2
    ///                         are version 6.1.0.0. All Windows 10 based releases will be listed
    ///                         as version 6.3.
    ///  * `os_minor_version` - The minor version number of the operating system. The minimum value
    ///                         is 0.
    ///  * `sp_major_version` - The major version number of the operating system service pack. The
    ///                         minimum value is 0.
    ///  * `sp_minor_version` - The minor version number of the operating system service pack. The
    ///                         minimum value is 0.
    ///  * `returned_product_type` - The product type. This parameter cannot be [`null_mut`]. If
    ///                              the specified operating system is less than the current
    ///                              operating system, this information is mapped to the types
    ///                              supported by the specified operating system. If the specified
    ///                              operating system is greater than the highest supported
    ///                              operating system, this information is mapped to the types
    ///                              supported by the current operating system. This parameter can
    ///                              be one of the following values (some products below may be out
    ///                              of support):
    ///    * [`PRODUCT_BUSINESS`] - Business
    ///    * [`PRODUCT_BUSINESS_N`] - Business N
    ///    * [`PRODUCT_CLUSTER_SERVER`] - HPC Edition
    ///    * [`PRODUCT_CLUSTER_SERVER_V`] - Server Hyper Core V
    ///    * [`PRODUCT_CORE`] - Windows 10 Home
    ///    * [`PRODUCT_CORE_COUNTRYSPECIFIC`] - Windows 10 Home China
    ///    * [`PRODUCT_CORE_N`] - Windows 10 Home N
    ///    * [`PRODUCT_CORE_SINGLELANGUAGE`] - Windows 10 Home Single Language
    ///    * [`PRODUCT_DATACENTER_EVALUATION_SERVER`] - Server Datacenter (evaluation installation)
    ///    * [`PRODUCT_DATACENTER_A_SERVER_CORE`] - Server Datacenter, Semi-Annual Channel (core
    ///                                             installation)
    ///    * [`PRODUCT_STANDARD_A_SERVER_CORE`] - Server Standard, Semi-Annual Channel (core
    ///                                           installation)
    ///    * [`PRODUCT_DATACENTER_SERVER`] - Server Datacenter (full installation. For Server Core
    ///                                      installations of Windows Server 2012 and later, use
    ///                                      the method, Determining whether Server Core is
    ///                                      running.)
    ///    * [`PRODUCT_DATACENTER_SERVER_CORE`] - Server Datacenter (core installation, Windows
    ///                                           Server 2008 R2 and earlier)
    ///    * [`PRODUCT_DATACENTER_SERVER_CORE_V`] - Server Datacenter without Hyper-V (core
    ///                                             installation)
    ///    * [`PRODUCT_DATACENTER_SERVER_V`] - Server Datacenter without Hyper-V (full
    ///                                        installation)
    ///    * [`PRODUCT_EDUCATION`] - Windows 10 Education
    ///    * [`PRODUCT_EDUCATION_N`] - Windows 10 Education N
    ///    * [`PRODUCT_ENTERPRISE`] - Windows 10 Enterprise
    ///    * [`PRODUCT_ENTERPRISE_E`] - Windows 10 Enterprise E
    ///    * [`PRODUCT_ENTERPRISE_EVALUATION`] - Windows 10 Enterprise Evaluation
    ///    * [`PRODUCT_ENTERPRISE_N`] - Windows 10 Enterprise N
    ///    * [`PRODUCT_ENTERPRISE_N_EVALUATION`] - Windows 10 Enterprise N Evaluation
    ///    * [`PRODUCT_ENTERPRISE_S`] - Windows 10 Enterprise 2015 LTSB
    ///    * [`PRODUCT_ENTERPRISE_S_EVALUATION`] - Windows 10 Enterprise 2015 LTSB Evaluation
    ///    * [`PRODUCT_ENTERPRISE_S_N`] - Windows 10 Enterprise 2015 LTSB N
    ///    * [`PRODUCT_ENTERPRISE_S_N_EVALUATION`] - Windows 10 Enterprise 2015 LTSB N Evaluation
    ///    * [`PRODUCT_ENTERPRISE_SERVER`] - Server Enterprise (full installation)
    ///    * [`PRODUCT_ENTERPRISE_SERVER_CORE`] - Server Enterprise (core installation)
    ///    * [`PRODUCT_ENTERPRISE_SERVER_CORE_V`] - Server Enterprise without Hyper-V (core
    ///                                             installation)
    ///    * [`PRODUCT_ENTERPRISE_SERVER_IA64`] - Server Enterprise for Itanium-based Systems
    ///    * [`PRODUCT_ENTERPRISE_SERVER_V`] - Server Enterprise without Hyper-V (full
    ///                                        installation)
    ///    * [`PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL`] - Windows Essential Server Solution
    ///                                                  Additional
    ///    * [`PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC`] - Windows Essential Server Solution
    ///                                                     Additional SVC
    ///    * [`PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT`] - Windows Essential Server Solution
    ///                                                  Management
    ///    * [`PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC`] - Windows Essential Server Solution
    ///                                                     Management SVC
    ///    * [`PRODUCT_HOME_BASIC`] - Home Basic
    ///    * [`PRODUCT_HOME_BASIC_E`] - Not supported
    ///    * [`PRODUCT_HOME_BASIC_N`] - Home Basic N
    ///    * [`PRODUCT_HOME_PREMIUM`] - Home Premium
    ///    * [`PRODUCT_HOME_PREMIUM_E`] - Not supported
    ///    * [`PRODUCT_HOME_PREMIUM_N`] - Home Premium N
    ///    * [`PRODUCT_HOME_PREMIUM_SERVER`] - Windows Home Server 2011
    ///    * [`PRODUCT_HOME_SERVER`] - Windows Storage Server 2008 R2 Essentials
    ///    * [`PRODUCT_HYPERV`] - Microsoft Hyper-V Server
    ///    * [`PRODUCT_IOTENTERPRISE`] - Windows IoT Enterprise
    ///    * [`PRODUCT_IOTENTERPRISE_S`] - Windows IoT Enterprise LTSC
    ///    * [`PRODUCT_IOTUAP`] - Windows 10 IoT Core
    ///    * [`PRODUCT_IOTUAPCOMMERCIAL`] - Windows 10 IoT Core Commercial
    ///    * [`PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT`] - Windows Essential Business Server
    ///                                                     Management Server
    ///    * [`PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING`] - Windows Essential Business Server
    ///                                                    Messaging Server
    ///    * [`PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY`] - Windows Essential Business Server
    ///                                                   Security Server
    ///    * [`PRODUCT_MOBILE_CORE`] - Windows 10 Mobile
    ///    * [`PRODUCT_MOBILE_ENTERPRISE`] - Windows 10 Mobile Enterprise
    ///    * [`PRODUCT_MULTIPOINT_PREMIUM_SERVER`] - Windows MultiPoint Server Premium (full
    ///                                              installation)
    ///    * [`PRODUCT_MULTIPOINT_STANDARD_SERVER`] - Windows MultiPoint Server Standard (full
    ///                                               installation)
    ///    * [`PRODUCT_PPI_PRO`] - Windows 10 Team
    ///    * [`PRODUCT_PRO_FOR_EDUCATION`] - Windows 10 Pro Education
    ///    * [`PRODUCT_PRO_WORKSTATION`] - Windows 10 Pro for Workstations
    ///    * [`PRODUCT_PRO_WORKSTATION_N`] - Windows 10 Pro for Workstations N
    ///    * [`PRODUCT_PROFESSIONAL`] - Windows 10 Pro
    ///    * [`PRODUCT_PROFESSIONAL_E`] - Not supported
    ///    * [`PRODUCT_PROFESSIONAL_N`] - Windows 10 Pro N
    ///    * [`PRODUCT_PROFESSIONAL_WMC`] - Professional with Media Center
    ///    * [`PRODUCT_SB_SOLUTION_SERVER`] - Windows Small Business Server 2011 Essentials
    ///    * [`PRODUCT_SB_SOLUTION_SERVER_EM`] - Server For SB Solutions EM
    ///    * [`PRODUCT_SERVER_FOR_SB_SOLUTIONS`] - Server For SB Solutions
    ///    * [`PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM`] - Server For SB Solutions EM
    ///    * [`PRODUCT_SERVER_FOR_SMALLBUSINESS`] - Windows Server 2008 for Windows Essential
    ///                                             Server Solutions
    ///    * [`PRODUCT_SERVER_FOR_SMALLBUSINESS_V`] - Windows Server 2008 without Hyper-V for
    ///                                               Windows Essential Server Solutions
    ///    * [`PRODUCT_SERVER_FOUNDATION`] - Server Foundation
    ///    * [`PRODUCT_SERVERRDSH`] - Windows 10 Enterprise for Virtual Desktops
    ///    * [`PRODUCT_SMALLBUSINESS_SERVER`] - Windows Small Business Server
    ///    * [`PRODUCT_SMALLBUSINESS_SERVER_PREMIUM`] - Small Business Server Premium
    ///    * [`PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE`] - Small Business Server Premium (core
    ///                                                      installation)
    ///    * [`PRODUCT_SOLUTION_EMBEDDEDSERVER`] - Windows MultiPoint Server
    ///    * [`PRODUCT_STANDARD_EVALUATION_SERVER`] - Server Standard (evaluation installation)
    ///    * [`PRODUCT_STANDARD_SERVER`] - Server Standard (full installation. For Server Core
    ///                                    installations of Windows Server 2012 and later, use the
    ///                                    method, Determining whether Server Core is running.)
    ///    * [`PRODUCT_STANDARD_SERVER_CORE`] - Server Standard (core installation, Windows Server
    ///                                         2008 R2 and earlier)
    ///    * [`PRODUCT_STANDARD_SERVER_CORE_V`] - Server Standard without Hyper-V (core
    ///                                           installation)
    ///    * [`PRODUCT_STANDARD_SERVER_V`] - Server Standard without Hyper-V
    ///    * [`PRODUCT_STANDARD_SERVER_SOLUTIONS`] - Server Solutions Premium
    ///    * [`PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE`] - Server Solutions Premium (core
    ///                                                   installation)
    ///    * [`PRODUCT_STARTER`] - Starter
    ///    * [`PRODUCT_STARTER_E`] - Not supported
    ///    * [`PRODUCT_STARTER_N`] - Starter N
    ///    * [`PRODUCT_STORAGE_ENTERPRISE_SERVER`] - Storage Server Enterprise
    ///    * [`PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE`] - Storage Server Enterprise (core
    ///                                                   installation)
    ///    * [`PRODUCT_STORAGE_EXPRESS_SERVER`] - Storage Server Express
    ///    * [`PRODUCT_STORAGE_EXPRESS_SERVER_CORE`] - Storage Server Express (core installation)
    ///    * [`PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER`] - Storage Server Standard (evaluation
    ///                                                       installation)
    ///    * [`PRODUCT_STORAGE_STANDARD_SERVER`] - Storage Server Standard
    ///    * [`PRODUCT_STORAGE_STANDARD_SERVER_CORE`] - Storage Server Standard (core installation)
    ///    * [`PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER`] - Storage Server Workgroup (evaluation
    ///                                                        installation)
    ///    * [`PRODUCT_STORAGE_WORKGROUP_SERVER`] - Storage Server Workgroup
    ///    * [`PRODUCT_STORAGE_WORKGROUP_SERVER_CORE`] - Storage Server Workgroup (core
    ///                                                  installation)
    ///    * [`PRODUCT_ULTIMATE`] - Ultimate
    ///    * [`PRODUCT_ULTIMATE_E`] - Not supported
    ///    * [`PRODUCT_ULTIMATE_N`] - Ultimate N
    ///    * [`PRODUCT_UNDEFINED`] - An unknown product
    ///    * [`PRODUCT_WEB_SERVER`] - Web Server (full installation)
    ///    * [`PRODUCT_WEB_SERVER_CORE`] - Web Server (core installation)
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a nonzero value.
    ///
    /// If the function fails, the return value is zero. This function fails if one of the input
    /// parameters is invalid.
    ///
    /// # Remarks
    /// To detect whether a server role or feature is installed, use the Server Feature WMI provider.
    ///
    /// Subsequent releases of Windows will map the product types it supports to the set of product
    /// types supported by each supported previous release of Windows, back to version 6.0.0.0.
    /// Therefore, an application that does an equality test for any of these values will continue
    /// to work on future releases, even when new product types are added.
    ///
    /// `PRODUCT_*_SERVER_CORE` values are not returned in Windows Server 2012, and later. For
    /// example, the base server edition, Server Datacenter, is used to build the two different
    /// installation options: "full server" and "core server". With Windows Server 2012,
    /// [`GetProductInfo`] will return `PRODUCT_DATACENTER` regardless of the option used during
    /// product installation. As noted above, for Server Core installations of Windows Server 2012
    /// and later, use the method Determining whether Server Core is running.
    pub fn GetProductInfo(
        os_major_version: DWORD,
        os_minor_version: DWORD,
        sp_major_version: DWORD,
        sp_minor_version: DWORD,
        returned_product_type: PDWORD,
    ) -> BOOL;
}
