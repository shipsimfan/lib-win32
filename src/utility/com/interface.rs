use crate::{unknwn::IUnknownTrait, IID};

/// An interface presented through the component object model
pub trait COMInterface: IUnknownTrait {
    /// The identifier for this interface
    const IID: IID;
}
