mod coord;
mod focus_event_record;
mod input_record;
mod key_event_record;
mod menu_event_record;
mod mouse_event_record;
mod small_rect;
mod window_buffer_size_record;

pub use coord::COORD;
pub use focus_event_record::FOCUS_EVENT_RECORD;
pub use input_record::{INPUT_RECORD, INPUT_RECORD_UNION};
pub use key_event_record::KEY_EVENT_RECORD;
pub use menu_event_record::MENU_EVENT_RECORD;
pub use mouse_event_record::MOUSE_EVENT_RECORD;
pub use small_rect::SMALL_RECT;
pub use window_buffer_size_record::WINDOW_BUFFER_SIZE_RECORD;
