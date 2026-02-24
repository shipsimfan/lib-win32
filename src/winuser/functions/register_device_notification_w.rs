use crate::{DWORD, HANDLE, HDEVNOTIFY, LPVOID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    DEVICE_NOTIFY_ALL_INTERFACE_CLASSES, DEVICE_NOTIFY_SERVICE_HANDLE, DEVICE_NOTIFY_WINDOW_HANDLE,
    GetLastError, RegisterDeviceNotification, WM_DEVICECHANGE,
    dbt::{
        DBT_DEVICEARRIVAL, DBT_DEVICEREMOVECOMPLETE, DBT_DEVTYP_DEVICEINTERFACE, DBT_DEVTYP_HANDLE,
        DBT_DEVTYP_OEM, DBT_DEVTYP_PORT, DBT_DEVTYP_VOLUME, DEV_BROADCAST_HDR,
    },
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Registers the device or type of device for which a window will receive notifications.
    ///
    /// # Parameters
    ///  * `recipient` - A handle to the window or service that will receive device events for the
    ///                  devices specified in the `notification_filter` parameter. The same window
    ///                  handle can be used in multiple calls to [`RegisterDeviceNotification`].
    ///                  Services can specify either a window handle or service status handle.
    ///  * `notification_filter` - A pointer to a block of data that specifies the type of device
    ///                            for which notifications should be sent. This block always begins
    ///                            with the [`DEV_BROADCAST_HDR`] structure. The data following
    ///                            this header is dependent on the value of the `device_type`
    ///                            member, which can be [`DBT_DEVTYP_DEVICEINTERFACE`] or
    ///                            [`DBT_DEVTYP_HANDLE`].
    ///  * `flags` - This parameter can be one of the following values. In addition, you can
    ///              specify [`DEVICE_NOTIFY_ALL_INTERFACE_CLASSES`].
    ///    * [`DEVICE_NOTIFY_WINDOW_HANDLE`] - The `recipient` parameter is a window handle.
    ///    * [`DEVICE_NOTIFY_SERVICE_HANDLE`] - The `recipient` parameter is a service status
    ///                                         handle.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a device notification handle.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// Applications send event notifications using the [`BroadcastSystemMessage`] function. Any
    /// application with a top-level window can receive basic notifications by processing the
    /// [`WM_DEVICECHANGE`] message. Applications can use the [`RegisterDeviceNotification`]
    /// function to register to receive device notifications.
    ///
    /// Services can use the [`RegisterDeviceNotification`] function to register to receive device
    /// notifications. If a service specifies a window handle in the hRecipient parameter, the
    /// notifications are sent to the window procedure. If hRecipient is a service status handle,
    /// [`SERVICE_CONTROL_DEVICEEVENT`] notifications are sent to the service control handler. For
    /// more information about the service control handler, see [`HandlerEx`].
    ///
    /// Be sure to handle Plug and Play device events as quickly as possible. Otherwise, the system
    /// may become unresponsive. If your event handler is to perform an operation that may block
    /// execution (such as I/O), it is best to start another thread to perform the operation
    /// asynchronously.
    ///
    /// Device notification handles returned by [`RegisterDeviceNotification`] must be closed by
    /// calling the [`UnregisterDeviceNotification`] function when they are no longer needed.
    ///
    /// The [`DBT_DEVICEARRIVAL`] and [`DBT_DEVICEREMOVECOMPLETE`] events are automatically
    /// broadcast to all top-level windows for port devices. Therefore, it is not necessary to call
    /// [`RegisterDeviceNotification`] for ports, and the function fails if the `device_type`
    /// member is [`DBT_DEVTYP_PORT`]. Volume notifications are also broadcast to top-level
    /// windows, so the function fails if the `device_type` member is [`DBT_DEVTYP_VOLUME`].
    /// OEM-defined devices are not used directly by the system, so the function fails if the
    /// `device_type` member is [`DBT_DEVTYP_OEM`].
    pub fn RegisterDeviceNotificationW(
        recipient: HANDLE,
        notification_filter: LPVOID,
        flags: DWORD,
    ) -> HDEVNOTIFY;
}
