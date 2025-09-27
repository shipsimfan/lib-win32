use crate::HMENU;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// Retrieves a handle to the drop-down menu or submenu activated by the specified menu item.
    ///
    /// # Parameters
    ///  * `menu` - A handle to the menu.
    ///  * `pos` - The zero-based relative position in the specified menu of an item that activates
    ///            a drop-down menu or submenu.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the drop-down menu or submenu
    /// activated by the menu item. If the menu item does not activate a drop-down menu or submenu,
    /// the return value is [`null_mut`].
    pub fn GetSubMenu(menu: HMENU, pos: c_int) -> HMENU;
}
