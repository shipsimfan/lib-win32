use std::ffi::c_int;

/// Can do standard BLT.            
pub const RC_BITBLT: c_int = 1;

/// Device requires banding support
pub const RC_BANDING: c_int = 2;

/// Device requires scaling support
pub const RC_SCALING: c_int = 4;

/// Device can support >64K bitmap  
pub const RC_BITMAP64: c_int = 8;

/// has 2.0 output calls        
pub const RC_GDI20_OUTPUT: c_int = 0x0010;

#[allow(missing_docs)]
pub const RC_GDI20_STATE: c_int = 0x0020;

#[allow(missing_docs)]
pub const RC_SAVEBITMAP: c_int = 0x0040;

/// supports DIB to memory      
pub const RC_DI_BITMAP: c_int = 0x0080;

/// supports a palette          
pub const RC_PALETTE: c_int = 0x0100;

/// supports DIBitsToDevice     
pub const RC_DIBTODEV: c_int = 0x0200;

/// supports >64K fonts         
pub const RC_BIGFONT: c_int = 0x0400;

/// supports StretchBlt         
pub const RC_STRETCHBLT: c_int = 0x0800;

/// supports FloodFill          
pub const RC_FLOODFILL: c_int = 0x1000;

/// supports StretchDIBits      
pub const RC_STRETCHDIB: c_int = 0x2000;

#[allow(missing_docs)]
pub const RC_OP_DX_OUTPUT: c_int = 0x4000;

#[allow(missing_docs)]
pub const RC_DEVBITS: c_int = 0x8000;
