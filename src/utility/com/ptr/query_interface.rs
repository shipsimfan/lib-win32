use crate::{try_hresult, unknwn::IUnknown, ComInterface, ComPtr};

impl<T: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface> ComPtr<T> {
    /// Tries to get this as `T2`
    pub fn query_interface<T2: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface>(
        &mut self,
    ) -> crate::Result<ComPtr<T2>> {
        let mut ptr = self as *mut Self as *mut _;
        try_hresult!(IUnknown::query_interface(self.as_mut(), &T2::IID, &mut ptr))
            .map(|_| ComPtr::new(ptr.cast()))
    }
}
