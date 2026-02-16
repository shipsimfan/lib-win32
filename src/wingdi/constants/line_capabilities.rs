use std::ffi::c_int;

/// Lines not supported             
pub const LC_NONE: c_int = 0;

/// Can do polylines                
pub const LC_POLYLINE: c_int = 2;

/// Can do markers                  
pub const LC_MARKER: c_int = 4;

/// Can do polymarkers              
pub const LC_POLYMARKER: c_int = 8;

/// Can do wide lines               
pub const LC_WIDE: c_int = 16;

/// Can do styled lines             
pub const LC_STYLED: c_int = 32;

/// Can do wide styled lines        
pub const LC_WIDESTYLED: c_int = 64;

/// Can do interiors                
pub const LC_INTERIORS: c_int = 128;
