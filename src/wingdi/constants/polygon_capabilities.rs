use std::ffi::c_int;

/// Polygonals not supported        
pub const PC_NONE: c_int = 0;

/// Can do polygons                 
pub const PC_POLYGON: c_int = 1;

/// Can do rectangles               
pub const PC_RECTANGLE: c_int = 2;

/// Can do winding polygons         
pub const PC_WINDPOLYGON: c_int = 4;

/// Can do trapezoids               
pub const PC_TRAPEZOID: c_int = 4;

/// Can do scanlines                
pub const PC_SCANLINE: c_int = 8;

/// Can do wide borders             
pub const PC_WIDE: c_int = 16;

/// Can do styled borders           
pub const PC_STYLED: c_int = 32;

/// Can do wide styled borders      
pub const PC_WIDESTYLED: c_int = 64;

/// Can do interiors                
pub const PC_INTERIORS: c_int = 128;

/// Can do polypolygons             
pub const PC_POLYPOLYGON: c_int = 256;

/// Can do paths                    
pub const PC_PATHS: c_int = 512;
