mod reg_copy_tree_w;
mod reg_create_key_ex_w;
mod reg_load_key_w;
mod reg_open_current_user;
mod reg_open_key_ex_w;
mod reg_replace_key_w;
mod reg_restore_key_w;
mod reg_save_key_ex_w;
mod reg_save_key_w;

pub use reg_copy_tree_w::{RegCopyTreeW, RegCopyTreeW as RegCopyTree};
pub use reg_create_key_ex_w::{RegCreateKeyExW, RegCreateKeyExW as RegCreateKeyEx};
pub use reg_load_key_w::{RegLoadKeyW, RegLoadKeyW as RegLoadKey};
pub use reg_open_current_user::RegOpenCurrentUser;
pub use reg_open_key_ex_w::{RegOpenKeyExW, RegOpenKeyExW as RegOpenKeyEx};
pub use reg_replace_key_w::{RegReplaceKeyW, RegReplaceKeyW as RegReplaceKey};
pub use reg_restore_key_w::{RegRestoreKeyW, RegRestoreKeyW as RegRestoreKey};
pub use reg_save_key_ex_w::{RegSaveKeyExW, RegSaveKeyExW as RegSaveKeyEx};
pub use reg_save_key_w::{RegSaveKeyW, RegSaveKeyW as RegSaveKey};
