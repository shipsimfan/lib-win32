use crate::{com_interface, HRESULT, REFIID, ULONG};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{COMInterface, E_NOINTERFACE, E_POINTER, IID, S_OK};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// Enables clients to get pointers to other interfaces on a given object through the
    /// `query_interface` method, and manage the existence of the object through the `add_ref` and
    /// `release` methods. All other COM interfaces are inherited, directly or indirectly, from
    /// [`IUnknown`]. Therefore, the three methods in [`IUnknown`] are the first entries in the
    /// vtable for every interface.
    pub abstract IUnknown(IUnknownVTable/IUnknownTrait) {
        const IID = 0x00000000-0x0000-0x0000-0xC000-0x000000000046;

        /// Queries a COM object for a pointer to one of its interface; identifying the interface
        /// by a reference to its interface identifier ([`IID`]). If the COM object implements the
        /// interface, then it returns a pointer to that interface after calling
        /// [`IUnknownTrait::add_ref`] on it.
        ///
        /// # Parameters
        ///  * `riid` - A reference to the interface identifier ([`IID`]) of the interface being
        ///             queried for.
        ///  * `object` - The address of a pointer to an interface with the [`IID`] specified in
        ///               the `riid` parameter. Because you pass the address of an interface
        ///               pointer, the method can overwrite that address with the pointer to the
        ///               interface being queried for. Upon successful return, `*object` (the
        ///               dereferenced address) contains a pointer to the requested interface. If
        ///               the object doesn't support the interface, the method sets `*object` (the
        ///               dereferenced address) to [`null_mut`].
        ///
        /// # Return Value
        /// This method returns [`S_OK`] if the interface is supported, and [`E_NOINTERFACE`]
        /// otherwise. If `object` (the address) is [`null_mut`], then this method returns
        /// [`E_POINTER`].
        ///
        /// # Remarks
        /// For any given COM object (also known as a COM component), a specific query for the
        /// [`IUnknown`] interface on any of the object's interfaces must always return the same
        /// pointer value. This enables a client to determine whether two pointers point to the
        /// same component by calling `query_interface` with [`COMInterface::IID`] and comparing
        /// the results. It is specifically not the case that queries for interfaces other than
        /// [`IUnknown`] (even the same interface through the same pointer) must return the same
        /// pointer value.
        ///
        /// There are four requirements for implementations of `query_interface` (In these cases,
        /// "must succeed" means "must succeed barring catastrophic failure.").
        ///  - The set of interfaces accessible on an object through `query_interface` must be
        ///    static, not dynamic. This means that if a call to `query_interface` for a pointer
        ///    to a specified interface succeeds the first time, then it must succeed again. If the
        ///    call fails the first time, then it must fail on all subsequent calls.
        ///  - It must be reflexive — if a client holds a pointer to an interface on an object, and
        ///    the client queries for that interface, then the call must succeed.
        ///  - It must be symmetric — if a client holding a pointer to one interface queries
        ///    successfully for another, then a query through the obtained pointer for the first
        ///    interface must succeed.
        ///  - It must be transitive — if a client holding a pointer to one interface queries
        ///    successfully for a second, and through that pointer queries successfully for a third
        ///    interface, then a query for the first interface through the pointer for the third
        ///    interface must succeed.
        fn query_interface(&mut self, riid: REFIID, object: *mut *mut c_void) -> HRESULT;

        /// Increments the reference count for an interface pointer to a COM object. You should
        /// call this method whenever you make a copy of an interface pointer.
        ///
        /// # Return Value
        /// The method returns the new reference count. This value is intended to be used only for
        /// test purposes.
        ///
        /// # Remarks
        /// A COM object uses a per-interface reference-counting mechanism to ensure that the
        /// object doesn't outlive references to it. You use `add_ref` to stabilize a copy of an
        /// interface pointer. It can also be called when the life of a cloned pointer must extend
        /// beyond the lifetime of the original pointer. The cloned pointer must be released by
        /// calling [`IUnknownTrait::release`] on it.
        ///
        /// The internal reference counter that AddRef maintains should be a 32-bit unsigned
        /// integer.
        fn add_ref(&mut self) -> ULONG;

        /// Decrements the reference count for an interface on a COM object.
        ///
        /// # Return Value
        /// The method returns the new reference count. This value is intended to be used only for
        /// test purposes.
        ///
        /// # Remarks
        /// When the reference count on an object reaches zero, Release must cause the interface
        /// pointer to free itself. When the released pointer is the only (formerly) outstanding
        /// reference to an object (whether the object supports single or multiple interfaces), the
        /// implementation must free the object.
        ///
        /// Note that aggregation of objects restricts the ability to recover interface pointers.
        fn release(&mut self) -> ULONG;
    }
);
