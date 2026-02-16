mod create_dc_w;
mod delete_dc;
mod get_device_caps;

pub use create_dc_w::{CreateDCW, CreateDCW as CreateDC};
pub use delete_dc::DeleteDC;
pub use get_device_caps::GetDeviceCaps;
