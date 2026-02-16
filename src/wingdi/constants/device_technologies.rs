use std::ffi::c_int;

/// Vector plotter                  
pub const DT_PLOTTER: c_int = 0;

/// Raster display                  
pub const DT_RASDISPLAY: c_int = 1;

/// Raster printer                  
pub const DT_RASPRINTER: c_int = 2;

/// Raster camera                   
pub const DT_RASCAMERA: c_int = 3;

/// Character-stream, PLP           
pub const DT_CHARSTREAM: c_int = 4;

/// Metafile, VDM                   
pub const DT_METAFILE: c_int = 5;

/// Display-file                    
pub const DT_DISPFILE: c_int = 6;
