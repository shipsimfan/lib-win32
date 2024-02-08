mod format_message_w;
mod local_alloc;
mod local_flags;
mod local_free;
mod local_lock;
mod local_realloc;
mod local_unlock;

pub use format_message_w::{FormatMessageW, FormatMessageW as FormatMessage};
pub use local_alloc::LocalAlloc;
pub use local_flags::LocalFlags;
pub use local_free::LocalFree;
pub use local_lock::LocalLock;
pub use local_realloc::LocalReAlloc;
pub use local_unlock::LocalUnlock;
