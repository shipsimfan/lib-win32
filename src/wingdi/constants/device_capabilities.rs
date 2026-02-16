use std::ffi::c_int;

/// Device driver version                   
pub const DRIVERVERSION: c_int = 0;

/// Device classification                   
pub const TECHNOLOGY: c_int = 2;

/// Horizontal size in millimeters          
pub const HORZSIZE: c_int = 4;

/// Vertical size in millimeters            
pub const VERTSIZE: c_int = 6;

/// Horizontal width in pixels              
pub const HORZRES: c_int = 8;

/// Vertical height in pixels               
pub const VERTRES: c_int = 10;

/// Number of bits per pixel                
pub const BITSPIXEL: c_int = 12;

/// Number of planes                        
pub const PLANES: c_int = 14;

/// Number of brushes the device has        
pub const NUMBRUSHES: c_int = 16;

/// Number of pens the device has           
pub const NUMPENS: c_int = 18;

/// Number of markers the device has        
pub const NUMMARKERS: c_int = 20;

/// Number of fonts the device has          
pub const NUMFONTS: c_int = 22;

/// Number of colors the device supports    
pub const NUMCOLORS: c_int = 24;

/// Size required for device descriptor     
pub const PDEVICESIZE: c_int = 26;

/// Curve capabilities                      
pub const CURVECAPS: c_int = 28;

/// Line capabilities                       
pub const LINECAPS: c_int = 30;

/// Polygonal capabilities                  
pub const POLYGONALCAPS: c_int = 32;

/// Text capabilities                       
pub const TEXTCAPS: c_int = 34;

/// Clipping capabilities                   
pub const CLIPCAPS: c_int = 36;

/// Bitblt capabilities                     
pub const RASTERCAPS: c_int = 38;

/// Length of the X leg                     
pub const ASPECTX: c_int = 40;

/// Length of the Y leg                     
pub const ASPECTY: c_int = 42;

/// Length of the hypotenuse                
pub const ASPECTXY: c_int = 44;

/// Logical pixels/inch in X                
pub const LOGPIXELSX: c_int = 88;

/// Logical pixels/inch in Y                
pub const LOGPIXELSY: c_int = 90;

/// Number of entries in physical palette   
pub const SIZEPALETTE: c_int = 104;

/// Number of reserved entries in palette   
pub const NUMRESERVED: c_int = 106;

/// Actual color resolution                 
pub const COLORRES: c_int = 108;

/// Physical Width in device units          
pub const PHYSICALWIDTH: c_int = 110;

/// Physical Height in device units         
pub const PHYSICALHEIGHT: c_int = 111;

/// Physical Printable Area x margin        
pub const PHYSICALOFFSETX: c_int = 112;

/// Physical Printable Area y margin        
pub const PHYSICALOFFSETY: c_int = 113;

/// Scaling factor x                        
pub const SCALINGFACTORX: c_int = 114;

/// Scaling factor y                        
pub const SCALINGFACTORY: c_int = 115;

/// Current vertical refresh rate of the display device (for displays only) in Hz  
pub const VREFRESH: c_int = 116;

/// Horizontal width of entire desktop in pixels
pub const DESKTOPVERTRES: c_int = 117;

/// Vertical height of entire desktop in pixels
pub const DESKTOPHORZRES: c_int = 118;

/// Preferred blt alignment                
pub const BLTALIGNMENT: c_int = 119;

/// Shading and blending caps              
pub const SHADEBLENDCAPS: c_int = 120;

/// Color Management caps                  
pub const COLORMGMTCAPS: c_int = 121;
