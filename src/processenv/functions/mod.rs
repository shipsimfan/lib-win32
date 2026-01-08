mod expand_environment_strings_a;
mod expand_environment_strings_w;
mod get_std_handle;

pub use expand_environment_strings_a::ExpandEnvironmentStringsA;
pub use expand_environment_strings_w::{
    ExpandEnvironmentStringsW, ExpandEnvironmentStringsW as ExpandEnvironmentStrings,
};
pub use get_std_handle::GetStdHandle;
