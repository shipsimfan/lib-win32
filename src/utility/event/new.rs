use crate::{
    try_get_last_error, CreateEvent, Event, Result, FALSE, HANDLE, SECURITY_ATTRIBUTES, TRUE,
};
use std::ptr::{null, null_mut};

impl Event {
    /// Creates a new [`Event`]
    pub fn new(
        attributes: Option<&mut SECURITY_ATTRIBUTES>,
        manual_reset: bool,
        initial_state: bool,
        name: Option<&[u16]>,
    ) -> Result<Self> {
        try_get_last_error!(CreateEvent(
            match attributes {
                Some(attributes) => attributes,
                None => null_mut(),
            },
            if manual_reset { TRUE } else { FALSE },
            if initial_state { TRUE } else { FALSE },
            name.map(|str| str.as_ptr()).unwrap_or(null())
        ))
        .map(|handle| Event { handle })
    }

    /// Creates a new [`Event`] with default settings
    pub fn default() -> Result<Self> {
        Event::new(None, false, false, None)
    }

    /// Creates a new [`Event`] from a raw `handle`
    pub unsafe fn from_raw(handle: HANDLE) -> Event {
        Event { handle }
    }
}
