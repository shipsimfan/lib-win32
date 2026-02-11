use crate::{LONG, ULONG, USHORT};
use std::ops::{Deref, DerefMut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    SystemParametersInfo, MOUSE_ATTRIBUTES_CHANGED, MOUSE_MOVE_ABSOLUTE, MOUSE_MOVE_NOCOALESCE,
    MOUSE_MOVE_RELATIVE, MOUSE_VIRTUAL_DESKTOP, RI_MOUSE_BUTTON_1_DOWN, RI_MOUSE_BUTTON_1_UP,
    RI_MOUSE_BUTTON_2_DOWN, RI_MOUSE_BUTTON_2_UP, RI_MOUSE_BUTTON_3_DOWN, RI_MOUSE_BUTTON_3_UP,
    RI_MOUSE_BUTTON_4_DOWN, RI_MOUSE_BUTTON_4_UP, RI_MOUSE_BUTTON_5_DOWN, RI_MOUSE_BUTTON_5_UP,
    RI_MOUSE_HWHEEL, RI_MOUSE_LEFT_BUTTON_DOWN, RI_MOUSE_LEFT_BUTTON_UP,
    RI_MOUSE_MIDDLE_BUTTON_DOWN, RI_MOUSE_MIDDLE_BUTTON_UP, RI_MOUSE_RIGHT_BUTTON_DOWN,
    RI_MOUSE_RIGHT_BUTTON_UP, RI_MOUSE_WHEEL, SPI_GETWHEELSCROLLCHARS, SPI_GETWHEELSCROLLLINES,
    WHEEL_DELTA, WM_MOUSEMOVE,
};

/// Contains information about the state of the mouse.
///
/// # Remarks
/// If the mouse has moved, indicated by [`MOUSE_MOVE_RELATIVE`] or [`MOUSE_MOVE_ABSOLUTE`],
/// `last_x` and `last_y` specify information about that movement. The information is specified as
/// relative or absolute integer values.
///
/// If [`MOUSE_MOVE_RELATIVE`] value is specified, `last_x` and `last_y` specify movement relative
/// to the previous mouse event (the last reported position). Positive values mean the mouse moved
/// right (or down); negative values mean the mouse moved left (or up).
///
/// If [`MOUSE_MOVE_ABSOLUTE`] value is specified, `last_x` and `last_y` contain normalized
/// absolute coordinates between 0 and 65,535. Coordinate (0,0) maps onto the upper-left corner of
/// the display surface; coordinate (65535,65535) maps onto the lower-right corner. In a
/// multimonitor system, the coordinates map to the primary monitor.
///
/// If [`MOUSE_VIRTUAL_DESKTOP`] is specified in addition to [`MOUSE_MOVE_ABSOLUTE`], the
/// coordinates map to the entire virtual desktop.
///
/// In contrast to legacy [`WM_MOUSEMOVE`] window messages Raw Input mouse events is not subject to
/// the effects of the mouse speed set in the Control Panel's Mouse Properties sheet.
///
/// If mouse wheel is moved, indicated by [`RI_MOUSE_WHEEL`] or [`RI_MOUSE_HWHEEL`] in
/// `buttons_flags`, then `button_data` contains a signed short value that specifies the distance
/// the wheel is rotated.
///
/// The wheel rotation will be a multiple of [`WHEEL_DELTA`], which is set at 120. This is the
/// threshold for action to be taken, and one such action (for example, scrolling one increment)
/// should occur for each delta.
///
/// The delta was set to 120 to allow Microsoft or other vendors to build finer-resolution wheels
/// (a freely-rotating wheel with no notches) to send more messages per rotation, but with a
/// smaller value in each message. To use this feature, you can either add the incoming delta
/// values until [`WHEEL_DELTA`] is reached (so for a delta-rotation you get the same response), or
/// scroll partial lines in response to the more frequent messages. You can also choose your scroll
/// granularity and accumulate deltas until it is reached.
///
/// The application could also retrieve the current lines-to-scroll and characters-to-scroll user
/// setting by using the [`SystemParametersInfo`] API with [`SPI_GETWHEELSCROLLLINES`] or
/// [`SPI_GETWHEELSCROLLCHARS`] parameter.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RAWMOUSE {
    /// The mouse state. This member can be any reasonable combination of the following:
    ///  * [`MOUSE_MOVE_RELATIVE`] - Mouse movement data is relative to the last mouse position.
    ///  * [`MOUSE_MOVE_ABSOLUTE`] - Mouse movement data is based on absolute position.
    ///  * [`MOUSE_MOVE_VIRTUAL_DESKTOP`] - Mouse coordinates are mapped to the virtual desktop
    ///                                     (for a multiple monitor system).
    ///  * [`MOUSE_ATTRIBUTES_CHANGED`] - Mouse attributes changed; application needs to query the
    ///                                   mouse attributes.
    ///  * [`MOUSE_MOVE_NOCOALESCE`] - This mouse movement event was not coalesced. Mouse movement
    ///                                events can be coalesced by default.
    pub flags: USHORT,

    #[allow(missing_docs)]
    pub dummy: RAWMOUSE_UNION,

    /// The raw state of the mouse buttons. The Win32 subsystem does not use this member.
    pub raw_buttons: ULONG,

    /// The motion in the X direction. This is signed relative motion or absolute motion, depending
    /// on the value of `flags`.
    pub last_x: LONG,

    /// The motion in the Y direction. This is signed relative motion or absolute motion, depending
    /// on the value of `flags`.
    pub last_y: LONG,

    /// Additional device-specific information for the event.
    pub extra_information: ULONG,
}

impl Default for RAWMOUSE {
    fn default() -> Self {
        RAWMOUSE {
            flags: 0,
            dummy: RAWMOUSE_UNION::default(),
            raw_buttons: 0,
            last_x: 0,
            last_y: 0,
            extra_information: 0,
        }
    }
}

impl Deref for RAWMOUSE {
    type Target = RAWMOUSE_UNION;

    fn deref(&self) -> &Self::Target {
        &self.dummy
    }
}

impl DerefMut for RAWMOUSE {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dummy
    }
}

/// Union for [`RAWMOUSE`]
#[repr(C)]
#[derive(Clone, Copy)]
pub union RAWMOUSE_UNION {
    /// Reserved.
    pub buttons: ULONG,

    #[allow(missing_docs)]
    pub dummy: RAWMOUSE_STRUCT,
}

impl Default for RAWMOUSE_UNION {
    fn default() -> Self {
        RAWMOUSE_UNION { buttons: 0 }
    }
}

impl Deref for RAWMOUSE_UNION {
    type Target = RAWMOUSE_STRUCT;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.dummy }
    }
}

impl DerefMut for RAWMOUSE_UNION {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.dummy }
    }
}

/// Struct for [`RAWMOUSEUNION`]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RAWMOUSE_STRUCT {
    /// The transition state of the mouse buttons. This member can be one or more of the following
    /// values:
    ///  * [`RI_MOUSE_BUTTON_1_DOWN`] or [`RI_MOUSE_LEFT_BUTTON_DOWN`] - Left button changed to
    ///                                                                  down.
    ///  * [`RI_MOUSE_BUTTON_1_UP`] or [`RI_MOUSE_LEFT_BUTTON_UP`] - Left button changed to up.
    ///  * [`RI_MOUSE_BUTTON_2_DOWN`] or [`RI_MOUSE_RIGHT_BUTTON_DOWN`] - Right button changed to
    ///                                                                   down.
    ///  * [`RI_MOUSE_BUTTON_2_UP`] or [`RI_MOUSE_RIGHT_BUTTON_UP`] - Right button changed to up.
    ///  * [`RI_MOUSE_BUTTON_3_DOWN`] or [`RI_MOUSE_MIDDLE_BUTTON_DOWN`] - Middle button changed to
    ///                                                                    down.
    ///  * [`RI_MOUSE_BUTTON_3_UP`] or [`RI_MOUSE_MIDDLE_BUTTON_UP`] - Middle button changed to up.
    ///  * [`RI_MOUSE_BUTTON_4_DOWN`] - XBUTTON1 changed to down.
    ///  * [`RI_MOUSE_BUTTON_4_UP`] - XBUTTON1 changed to up.
    ///  * [`RI_MOUSE_BUTTON_5_DOWN`] - XBUTTON2 changed to down.
    ///  * [`RI_MOUSE_BUTTON_5_UP`] - XBUTTON2 changed to up.
    ///  * [`RI_MOUSE_WHEEL`] - Raw input comes from a mouse wheel. The wheel delta is stored in
    ///                         `button_data`. A positive value indicates that the wheel was rotated
    ///                         forward, away from the user; a negative value indicates that the
    ///                         wheel was rotated backward, toward the user.
    ///  * [`RI_MOUSE_HWHEEL`] - Raw input comes from a horizontal mouse wheel. The wheel delta is
    ///                          stored in `button_data`. A positive value indicates that the wheel
    ///                          was rotated to the right; a negative value indicates that the
    ///                          wheel was rotated to the left.
    pub button_flags: USHORT,

    /// If `button_flags` has [`RI_MOUSE_WHEEL`] or [`RI_MOUSE_HWHEEL`], this member specifies the
    /// distance the wheel is rotated.
    pub button_data: USHORT,
}

impl Default for RAWMOUSE_STRUCT {
    fn default() -> Self {
        RAWMOUSE_STRUCT {
            button_flags: 0,
            button_data: 0,
        }
    }
}
