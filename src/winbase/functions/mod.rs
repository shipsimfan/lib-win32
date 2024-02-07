mod format_message_w;
mod local_alloc;
mod local_free;

pub use format_message_w::{FormatMessageW, FormatMessageW as FormatMessage};
pub use local_alloc::LocalAlloc;
pub use local_free::LocalFree;
