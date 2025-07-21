use crate::{Error, HRESULT};

impl Error {
    /// Gets the underlying OS result
    pub const fn get(&self) -> HRESULT {
        self.0
    }
}
