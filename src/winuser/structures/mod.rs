mod client_create_struct;
mod create_struct_w;
mod mdi_create_struct_w;
mod msg;
mod paint_struct;
mod wnd_class_ex_w;
mod wnd_class_w;

pub use client_create_struct::CLIENTCREATESTRUCT;
pub use create_struct_w::{CREATESTRUCTW, CREATESTRUCTW as CREATESTRUCT};
pub use mdi_create_struct_w::{MDICREATESTRUCTW, MDICREATESTRUCTW as MDICREATESTRUCT};
pub use msg::MSG;
pub use paint_struct::PAINTSTRUCT;
pub use wnd_class_ex_w::{WNDCLASSEXW, WNDCLASSEXW as WNDCLASSEX};
pub use wnd_class_w::{WNDCLASSW, WNDCLASSW as WNDCLASS};
