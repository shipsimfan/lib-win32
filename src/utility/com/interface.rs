use crate::IID;

/// An interface presented through the component object model
pub trait ComInterface {
    /// The virtual function call table type
    type VTable;

    /// The identifier for this interface
    const IID: IID;
}
