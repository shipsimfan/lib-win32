use crate::{try_hresult, unknwn::IUnknownTrait, COMInterface, COMPtr};

impl<T: COMInterface> COMPtr<T> {
    /// Tries to get this as `T2`
    pub fn query_interface<T2: COMInterface>(&mut self) -> crate::Result<COMPtr<T2>> {
        let mut ptr = self as *mut Self as *mut _;
        try_hresult!(IUnknownTrait::query_interface(
            &mut **self,
            &T2::IID,
            &mut ptr
        ))
        .map(|_| COMPtr::new(ptr.cast()))
    }
}
