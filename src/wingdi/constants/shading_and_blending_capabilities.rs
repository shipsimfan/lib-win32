use std::ffi::c_int;

/// Device does not support any of these capabilities.
pub const SB_NONE: c_int = 0x00000000;

/// Handles the `source_constant_alpha` member of the [`BLENDFUNCTION`] structure, which is
/// referenced by the `blend_function` parameter of the [`AlphaBlend`] function.
pub const SB_CONST_ALPHA: c_int = 0x00000001;

/// Capable of handling per-pixel alpha in [`AlphaBlend`].
pub const SB_PIXEL_ALPHA: c_int = 0x00000002;

/// Capable of handling premultiplied alpha in [`AlphaBlend`].
pub const SB_PREMULT_ALPHA: c_int = 0x00000004;

/// Capable of doing [`GradientFill`] rectangles.
pub const SB_GRAD_RECT: c_int = 0x00000010;

/// Capable of doing [`GradientFill`] triangles.
pub const SB_GRAD_TRI: c_int = 0x00000020;
