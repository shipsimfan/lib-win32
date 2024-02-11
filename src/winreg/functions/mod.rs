mod reg_copy_tree_w;
mod reg_open_current_user;
mod reg_restore_key_w;
mod reg_save_key_ex_w;

pub use reg_copy_tree_w::{RegCopyTreeW, RegCopyTreeW as RegCopyTree};
pub use reg_open_current_user::RegOpenCurrentUser;
pub use reg_restore_key_w::{RegRestoreKeyW, RegRestoreKeyW as RegRestoreKey};
pub use reg_save_key_ex_w::{RegSaveKeyExW, RegSaveKeyExW as RegSaveKeyEx};
