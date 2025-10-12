use crate::{try_hresult, unknwn::IUnknownTrait, ComPtr};

impl<T: IUnknownTrait> ComPtr<T> {
    /// Tries to get this as `T2`
    pub fn query_interface<T2: IUnknownTrait>(&mut self) -> crate::Result<ComPtr<T2>> {
        let mut ptr = self as *mut Self as *mut _;
        try_hresult!(IUnknownTrait::query_interface(
            &mut **self,
            &T2::IID,
            &mut ptr
        ))
        .map(|_| ComPtr::new(ptr.cast()))
    }
}
