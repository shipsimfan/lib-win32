use crate::WPARAM;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    BROADCAST_QUERY_DENY, HKEY_CURRENT_CONFIG, RegisterDeviceNotification, TRUE, WM_DEVICECHANGE,
    dbt::{_DEV_BROADCAST_USERDEFINED, DEV_BROADCAST_HDR},
};

/// The system broadcasts the [`DBT_DEVNODES_CHANGED`] device event when a device has been added to
/// or removed from the system. Applications that maintain lists of devices in the system should
/// refresh their lists.
///
/// To broadcast this device event, the system uses the [`WM_DEVICECHANGE`] message with `w_param`
/// set to [`DBT_DEVNODES_CHANGED`] and `l_param` set to zero.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_DEVNODES_CHANGED`].
///  * `l_param` - Set to zero.
///
/// # Return Value
/// Return [`TRUE`].
///
/// # Remarks
/// There is no additional information about which device has been added to or removed from the
/// system. Applications that require more information should register for device notification
/// using the [`RegisterDeviceNotification`] function.
pub const DBT_DEVNODES_CHANGED: WPARAM = 0x0007;

/// The system broadcasts the [`DBT_QUERYCHANGECONFIG`] device event to request permission to
/// change the current configuration (dock or undock). Any application can deny this request and
/// cancel the change.
///
/// To broadcast this device event, the system uses the [`WM_DEVICECHANGE`] message with `w_param`
/// set to [`DBT_QUERYCHANGECONFIG`] and `l_param` set to zero.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_QUERYCHANGECONFIG`].
///  * `l_param` - Set to zero.
///
/// # Return Value
/// Return [`TRUE`] to grant permission to change the configuration.
///
/// Return [`BROADCAST_QUERY_DENY`] to deny permission to change the configuration.
pub const DBT_QUERYCHANGECONFIG: WPARAM = 0x0017;

/// The system broadcasts the [`DBT_CONFIGCHANGED`] device event to indicate that the current
/// configuration has changed, due to a dock or undock. An application or driver that stores data
/// in the registry under the [`HKEY_CURRENT_CONFIG`] key should update the data.
///
/// To broadcast this device event, the system uses the [`WM_DEVICECHANGE`] message with `w_param`
/// set to [`DBT_CONFIGCHANGED`] and `l_param` set to zero.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_CONFIGCHANGED`].
///  * `l_param` - Set to zero.
///
/// # Return Value
/// Return [`TRUE`].
pub const DBT_CONFIGCHANGED: WPARAM = 0x0018;

/// The system broadcasts the [`DBT_CONFIGCHANGECANCELED`] device event when a request to change
/// the current configuration (dock or undock) has been canceled.
///
/// To broadcast this device event, the system uses the [`WM_DEVICECHANGE`] message with `w_param`
/// set to [`DBT_CONFIGCHANGECANCELED`] and `l_param` set to zero.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_CONFIGCHANGECANCELED`].
///  * `l_param` - Set to zero.
///
/// # Return Value
/// Return [`TRUE`].
pub const DBT_CONFIGCHANGECANCELED: WPARAM = 0x0019;

/// The system broadcasts the [`DBT_DEVICEARRIVAL`] device event when a device or piece of media
/// has been inserted and becomes available.
///
/// To broadcast this device event, the system uses the [`WM_DEVICECHANGE`] message with `w_param`
/// set to [`DBT_DEVICEARRIVAL`] and `l_param` set as described following.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_DEVICEARRIVAL`].
///  * `l_param` - A pointer to a structure identifying the device inserted. The structure consists
///                of an event-independent header, followed by event-dependent members that
///                describe the device. To use this structure, treat the structure as a
///                [`DEV_BROADCAST_HDR`] structure, then check its `device_type` member to
///                determine the device type.
///
/// # Return Value
/// Return [`TRUE`].
///
/// # Remarks
/// If media is being inserted, the type of device arriving is a volume (the `device_type`
/// member is [`DBT_DEVTYP_VOLUME`]) and the change effects the media (the `flags` member is
/// [`DBTF_MEDIA`]).
pub const DBT_DEVICEARRIVAL: WPARAM = 0x8000;

/// The system broadcasts the [`DBT_DEVICEQUERYREMOVE`] device event to request permission to
/// remove a device or piece of media. This message is the last chance for applications and drivers
/// to prepare for this removal. However, any application can deny this request and cancel the
/// operation.
///
/// To broadcast this device event, the system uses the [`WM_DEVICECHANGE`] message with `w_param`
/// set to [`DBT_DEVICEQUERYREMOVE`] and `l_param` set as described following.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_DEVICEQUERYREMOVE`].
///  * `l_param` - A pointer to a structure identifying the device to remove. The structure
///                consists of an event-independent header, followed by event-dependent members
///                that describe the device. To use this structure, treat the structure as a
///                [`DEV_BROADCAST_HDR`] structure, then check its `device_type` member to
///                determine the device type.
///
/// # Return Value
/// Return [`TRUE`] to grant permission to remove a device.
///
/// Return [`BROADCAST_QUERY_DENY`] to deny permission to remove a device.
///
/// # Remarks
/// You must close all handles to the device or the device removal will fail.
pub const DBT_DEVICEQUERYREMOVE: WPARAM = 0x8001;

/// The system broadcasts the [`DBT_DEVICEQUERYREMOVEFAILED`] device event when a request to remove
/// a device or piece of media has been canceled.
///
/// To broadcast this device event, the system uses the [`WM_DEVICECHANGE`] message with `w_param`
/// set to [`DBT_DEVICEQUERYREMOVEFAILED`] and `l_param` set as described following.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_DEVICEQUERYREMOVEFAILED`].
///  * `l_param` - A pointer to a structure identifying the device. The structure consists of an
///                event-independent header, followed by event-dependent members that describe the
///                device. To use this structure, treat the structure as a [`DEV_BROADCAST_HDR`]
///                structure, then check its `device_type` member to determine the device type.
///
/// # Return Value
/// Return [`TRUE`].
pub const DBT_DEVICEQUERYREMOVEFAILED: WPARAM = 0x8002;

/// The system broadcasts the [`DBT_DEVICEREMOVEPENDING`] device event when a device or piece of
/// media is being removed and is no longer available for use.
///
/// To broadcast this device event, the system uses the [`WM_DEVICECHANGE`] message with `w_param`
/// set to [`DBT_DEVICEREMOVEPENDING`] and `l_param` set as described following.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_DEVICEREMOVEPENDING`].
///  * `l_param` - A pointer to a structure identifying the device. The structure consists of an
///                event-independent header, followed by event-dependent members that describe the
///                device. To use this structure, treat the structure as a [`DEV_BROADCAST_HDR`]
///                structure, then check its `device_type` member to determine the device type.
///
/// # Return Value
/// Return [`TRUE`].
///
/// # Remarks
/// The system may broadcast a [`DBT_DEVICEREMOVEPENDING`] message without sending a corresponding
/// [`DBT_DEVICEQUERYREMOVE`] message. In such cases, the applications and drivers must recover
/// from the loss of the device as best they can.
pub const DBT_DEVICEREMOVEPENDING: WPARAM = 0x8003;

/// The system broadcasts the [`DBT_DEVICEREMOVECOMPLETE`] device event when a device or piece of
/// media has been physically removed.
///
/// To broadcast this device event, the system uses the [`WM_DEVICECHANGE`] message with `w_param`
/// set to [`DBT_DEVICEREMOVECOMPLETE`] and `l_param` set as described following.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_DEVICEREMOVECOMPLETE`].
///  * `l_param` - A pointer to a structure identifying the device removed. The structure consists
///                of an event-independent header, followed by event-dependent members that
///                describe the device. To use this structure, treat the structure as a
///                [`DEV_BROADCAST_HDR`] structure, then check its `device_type` member to
///                determine the device type.
///
/// # Return Value
/// Return [`TRUE`].
///
/// # Remarks
/// The system may broadcast a [`DBT_DEVICEREMOVECOMPLETE`] message without sending corresponding
/// [`DBT_DEVICEQUERYREMOVE`] and [`DBT_DEVICEREMOVEPENDING`] messages. In such cases, the
/// applications and drivers must recover from the loss of the device as best they can.
///
/// If media is being removed, the type of device arriving is a volume (the `device_type`
/// member is [`DBT_DEVTYP_VOLUME`]) and the change effects the media (the `flags` member is
/// [`DBTF_MEDIA`]).
pub const DBT_DEVICEREMOVECOMPLETE: WPARAM = 0x8004;

/// The system broadcasts the [`DBT_DEVICETYPESPECIFIC`] device event when a device-specific event
/// occurs.
///
/// To broadcast this device event, the system uses the [`WM_DEVICECHANGE`] message with `w_param`
/// set to [`DBT_DEVICETYPESPECIFIC`] and `l_param` set as described following.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_DEVICETYPESPECIFIC`].
///  * `l_param` - A pointer to a structure identifying the device. The structure consists of an
///                event-independent header, followed by event-dependent members that describe the
///                device. To use this structure, treat the structure as a [`DEV_BROADCAST_HDR`]
///                structure, then check its `device_type` member to determine the device type.
///
/// # Return Value
/// Return [`TRUE`].
pub const DBT_DEVICETYPESPECIFIC: WPARAM = 0x8005;

/// The system sends the [`DBT_CUSTOMEVENT`] device event when a driver-defined custom event has
/// occurred.
///
/// To broadcast this device event, the system uses the [`WM_DEVICECHANGE`] message with `w_param`
/// set to [`DBT_CUSTOMEVENT`] and `l_param` set as described following.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_CUSTOMEVENT`].
///  * `l_param` - A pointer to a structure identifying the device. The structure consists of an
///                event-independent header, followed by event-dependent members that describe the
///                device. To use this structure, treat the structure as a [`DEV_BROADCAST_HDR`]
///                structure, then check its `device_type` member to determine the device type.
///
/// # Return Value
/// Return [`TRUE`].
pub const DBT_CUSTOMEVENT: WPARAM = 0x8006;

/// The [`DBT_USERDEFINED`] device event identifies a user-defined event.
///
/// To broadcast this device event, call the [`BroadcastSystemMessage`] function with the
/// [`WM_DEVICECHANGE`] message. Set `w_param` to [`DBT_USERDEFINED`] and set `l_param` as
/// described following.
///
/// # Parameters
///  * `wnd` - A handle to a window.
///  * `msg` - The [`WM_DEVICECHANGE`] message identifier.
///  * `w_param` - Set to [`DBT_USERDEFINED`].
///  * `l_param` - A pointer to a [`_DEV_BROADCAST_USERDEFINED`] structure which describes the
///                user-defined broadcast in progress. The `name` member contains the name of the
///                user-defined message, followed by any user-defined data.
///
/// # Return Value
/// Return [`TRUE`].
pub const DBT_USERDEFINED: WPARAM = 0xFFFF;
