//! This header is used by System Services

mod functions;
mod types;

pub use functions::{
    CancelWaitableTimer, CreateEvent, CreateEventW, CreateWaitableTimer, CreateWaitableTimerW,
    ResetEvent, SetEvent, SetWaitableTimer, SleepEx, WaitForMultipleObjectsEx,
};
pub use types::PTIMERAPCROUTINE;
