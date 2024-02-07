mod format_message_w;
mod local_alloc;
mod local_free;
mod local_lock;

pub use format_message_w::{FormatMessageW, FormatMessageW as FormatMessage};
pub use local_alloc::LocalAlloc;
pub use local_free::LocalFree;
pub use local_lock::LocalLock;
