use crate::{unknwn::LPUNKNOWN, DWORD, HRESULT, LPVOID, REFCLSID, REFIID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    unknwn::IUnknown, CLASS_E_NOAGGREGATION, E_NOINTERFACE, E_POINTER, REGDB_E_CLASSNOTREG, S_OK,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Ole32")]
unsafe extern "system" {
    /// Creates and default-initializes a single object of the class associated with a specified
    /// [`CLSID`].
    ///
    /// Call [`CoCreateInstance`] when you want to create only one object on the local system. To
    /// create a single object on a remote system, call the [`CoCreateInstanceEx`] function. To
    /// create multiple objects based on a single [`CLSID`], call the [`CoGetClassObject`]
    /// function.
    ///
    /// # Parameters
    ///  * `clsid` - The [`CLSID`] associated with the data and code that will be used to create
    ///              the object.
    ///  * `unk_outer` - If [`null_mut`], indicates that the object is not being created as part of
    ///                  an aggregate. If not [`null_mut`], pointer to the aggregate object's
    ///                  [`IUnknown`] interface (the controlling [`IUnknown`]).
    ///  * `cls_context` - Context in which the code that manages the newly created object will
    ///                    run. The values are taken from the enumeration [`CLSCTX`].
    ///  * `iid` - A reference to the identifier of the interface to be used to communicate with
    ///            the object.
    ///  * `ptr` - Address of pointer variable that receives the interface pointer requested in
    ///            `iid`. Upon successful return, `*ptr` contains the requested interface pointer.
    ///            Upon failure, `*ptr` contains [`null_mut`].
    ///
    /// # Return Value
    /// This function can return the following values:
    ///  * [`S_OK`] - An instance of the specified object class was successfully created.
    ///  * [`REGDB_E_CLASSNOTREG`] - A specified class is not registered in the registration
    ///                              database. Also can indicate that the type of server you
    ///                              requested in the [`CLSCTX`] enumeration is not registered or
    ///                              the values for the server types in the registry are corrupt.
    ///  * [`CLASS_E_NOAGGREGATION`] - This class cannot be created as part of an aggregate.
    ///  * [`E_NOINTERFACE`] - The specified class does not implement the requested interface, or
    ///                        the controlling [`IUnknown`] does not expose the requested
    ///                        interface.
    ///  * [`E_POINTER`] - The `ptr` parameter is [`null_mut`].
    pub fn CoCreateInstance(
        clsid: REFCLSID,
        unk_outer: LPUNKNOWN,
        cls_context: DWORD,
        iid: REFIID,
        ptr: *mut LPVOID,
    ) -> HRESULT;
}
