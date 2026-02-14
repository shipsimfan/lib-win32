use crate::DPI_AWARENESS_CONTEXT;

/// DPI unaware. This window does not scale for DPI changes and is always assumed to have a scale
/// factor of 100% (96 DPI). It will be automatically scaled by the system on any other DPI
/// setting.
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = -1isize as DPI_AWARENESS_CONTEXT;

/// System DPI aware. This window does not scale for DPI changes. It will query for the DPI once
/// and use that value for the lifetime of the process. If the DPI changes, the process will not
/// adjust to the new DPI value. It will be automatically scaled up or down by the system when the
/// DPI changes from the system value.
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT =
    -2isize as DPI_AWARENESS_CONTEXT;

/// Per monitor DPI aware. This window checks for the DPI when it is created and adjusts the scale
/// factor whenever the DPI changes. These processes are not automatically scaled by the system.
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT =
    -3isize as DPI_AWARENESS_CONTEXT;

/// Also known as Per Monitor v2. An advancement over the original per-monitor DPI awareness mode,
/// which enables applications to access new DPI-related scaling behaviors on a per top-level
/// window basis.
///
/// Per Monitor v2 was made available in the Creators Update of Windows 10 (also known as version
/// 1703), and is not available on earlier versions of the operating system.
///
/// The additional behaviors introduced are as follows:
///  - Child window DPI change notifications - In Per Monitor v2 contexts, the entire window tree
///    is notified of any DPI changes that occur.
///  - Scaling of non-client area - All windows will automatically have their non-client area drawn
///    in a DPI sensitive fashion. Calls to EnableNonClientDpiScaling are unnecessary.
///  - Scaling of Win32 menus - All NTUSER menus created in Per Monitor v2 contexts will be scaling
///    in a per-monitor fashion.
///  - Dialog Scaling - Win32 dialogs created in Per Monitor v2 contexts will automatically respond
///    to DPI changes.
///  - Improved scaling of comctl32 controls - Various comctl32 controls have improved DPI scaling
///    behavior in Per Monitor v2 contexts.
///  - Improved theming behavior - UxTheme handles opened in the context of a Per Monitor v2 window
///    will operate in terms of the DPI associated with that window.
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT =
    -4isize as DPI_AWARENESS_CONTEXT;

/// DPI unaware with improved quality of GDI-based content. This mode behaves similarly to
/// [`DPI_AWARENESS_CONTEXT_UNAWARE`], but also enables the system to automatically improve the
/// rendering quality of text and other GDI-based primitives when the window is displayed on a
/// high-DPI monitor.
///
/// [`DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED`] was introduced in the October 2018 update of
/// Windows 10 (also known as version 1809).
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT =
    -5isize as DPI_AWARENESS_CONTEXT;
