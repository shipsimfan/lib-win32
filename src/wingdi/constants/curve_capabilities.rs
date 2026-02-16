use std::ffi::c_int;

/// Curves not supported            
pub const CC_NONE: c_int = 0;

/// Can do circles                  
pub const CC_CIRCLES: c_int = 1;

/// Can do pie wedges               
pub const CC_PIE: c_int = 2;

/// Can do chord arcs               
pub const CC_CHORD: c_int = 4;

/// Can do ellipese                 
pub const CC_ELLIPSES: c_int = 8;

/// Can do wide lines               
pub const CC_WIDE: c_int = 16;

/// Can do styled lines             
pub const CC_STYLED: c_int = 32;

/// Can do wide styled lines        
pub const CC_WIDESTYLED: c_int = 64;

/// Can do interiors                
pub const CC_INTERIORS: c_int = 128;

#[allow(missing_docs)]
pub const CC_ROUNDRECT: c_int = 256;
