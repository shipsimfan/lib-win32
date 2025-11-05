use crate::USHORT;

/// Mouse movement data is relative to the last mouse position.
pub const MOUSE_MOVE_RELATIVE: USHORT = 0x00;

/// Mouse movement data is based on absolute position.
pub const MOUSE_MOVE_ABSOLUTE: USHORT = 0x01;

/// Mouse coordinates are mapped to the virtual desktop (for a multiple monitor system).
pub const MOUSE_VIRTUAL_DESKTOP: USHORT = 0x02;

/// Mouse attributes changed; application needs to query the mouse attributes.
pub const MOUSE_ATTRIBUTES_CHANGED: USHORT = 0x04;

/// This mouse movement event was not coalesced. Mouse movement events can be coalesced by default.
pub const MOUSE_MOVE_NOCOALESCE: USHORT = 0x08;
