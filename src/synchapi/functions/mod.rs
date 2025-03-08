mod cancel_waitable_timer;
mod create_event_w;
mod create_waitable_timer_w;
mod reset_event;
mod set_event;
mod set_watiable_timer;
mod sleep_ex;
mod wait_for_multiple_objects_ex;

pub use cancel_waitable_timer::CancelWaitableTimer;
pub use create_event_w::{CreateEventW, CreateEventW as CreateEvent};
pub use create_waitable_timer_w::{
    CreateWaitableTimerW, CreateWaitableTimerW as CreateWaitableTimer,
};
pub use reset_event::ResetEvent;
pub use set_event::SetEvent;
pub use set_watiable_timer::SetWaitableTimer;
pub use sleep_ex::SleepEx;
pub use wait_for_multiple_objects_ex::WaitForMultipleObjectsEx;
