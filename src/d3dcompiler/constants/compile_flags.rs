use crate::UINT;

/// Directs the compiler to insert debug file/line/type/symbol information into the output code.
pub const D3DCOMPILE_DEBUG: UINT = 1 << 0;

/// Directs the compiler not to validate the generated code against known capabilities and
/// constraints. We recommend that you use this constant only with shaders that have been
/// successfully compiled in the past. DirectX always validates shaders before it sets them to a
/// device.
pub const D3DCOMPILE_SKIP_VALIDATION: UINT = 1 << 1;

/// Directs the compiler to skip optimization steps during code generation. We recommend that you
/// set this constant for debug purposes only.
pub const D3DCOMPILE_SKIP_OPTIMIZATION: UINT = 1 << 2;

/// Directs the compiler to pack matrices in row-major order on input and output from the shader.
pub const D3DCOMPILE_PACK_MATRIX_ROW_MAJOR: UINT = 1 << 3;

/// Directs the compiler to pack matrices in column-major order on input and output from the
/// shader. This type of packing is generally more efficient because a series of dot-products can
/// then perform vector-matrix multiplication.
pub const D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR: UINT = 1 << 4;

/// Directs the compiler to perform all computations with partial precision. If you set this
/// constant, the compiled code might run faster on some hardware.
pub const D3DCOMPILE_PARTIAL_PRECISION: UINT = 1 << 5;

/// Directs the compiler to compile a vertex shader for the next highest shader profile. This
/// constant turns debugging on and optimizations off.
pub const D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT: UINT = 1 << 6;

/// Directs the compiler to compile a pixel shader for the next highest shader profile. This
/// constant also turns debugging on and optimizations off.
pub const D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT: UINT = 1 << 7;

/// Directs the compiler to disable Preshaders. If you set this constant, the compiler does not
/// pull out static expression for evaluation.
pub const D3DCOMPILE_NO_PRESHADER: UINT = 1 << 8;

/// Directs the compiler to not use flow-control constructs where possible.
pub const D3DCOMPILE_AVOID_FLOW_CONTROL: UINT = 1 << 9;

/// Forces strict compile, which might not allow for legacy syntax. By default, the compiler
/// disables strictness on deprecated syntax.
pub const D3DCOMPILE_ENABLE_STRICTNESS: UINT = 1 << 11;

/// Forces the IEEE strict compile which avoids optimizations that may break IEEE rules.
pub const D3DCOMPILE_IEEE_STRICTNESS: UINT = 1 << 13;

/// Directs the compiler to enable older shaders to compile to 5_0 targets.
pub const D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY: UINT = 1 << 12;

/// Directs the compiler to use the lowest optimization level. If you set this constant, the
/// compiler might produce slower code but produces the code more quickly. Set this constant when
/// you develop the shader iteratively.
pub const D3DCOMPILE_OPTIMIZATION_LEVEL0: UINT = 1 << 14;

/// Directs the compiler to use the second lowest optimization level.
pub const D3DCOMPILE_OPTIMIZATION_LEVEL1: UINT = 0;

/// Directs the compiler to use the second highest optimization level.
pub const D3DCOMPILE_OPTIMIZATION_LEVEL2: UINT = (1 << 14) | (1 << 15);

/// Directs the compiler to use the highest optimization level. If you set this constant, the
/// compiler produces the best possible code but might take significantly longer to do so. Set this
/// constant for final builds of an application when performance is the most important factor.
pub const D3DCOMPILE_OPTIMIZATION_LEVEL3: UINT = 1 << 15;

/// Directs the compiler to treat all warnings as errors when it compiles the shader code. We
/// recommend that you use this constant for new shader code, so that you can resolve all warnings
/// and lower the number of hard-to-find code defects.
pub const D3DCOMPILE_WARNINGS_ARE_ERRORS: UINT = 1 << 18;

/// Directs the compiler to assume that unordered access views (UAVs) and shader resource views
/// (SRVs) may alias for cs_5_0.
pub const D3DCOMPILE_RESOURCES_MAY_ALIAS: UINT = 1 << 19;

/// Directs the compiler to enable unbounded descriptor tables.
pub const D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: UINT = 1 << 20;

/// Directs the compiler to ensure all resources are bound.
pub const D3DCOMPILE_ALL_RESOURCES_BOUND: UINT = 1 << 21;

/// When generating debug PDBs this makes use of the source file and binary for the hash.
pub const D3DCOMPILE_DEBUG_NAME_FOR_SOURCE: UINT = 1 << 22;

/// When generating debug PDBs this makes use of the binary file name only for the hash.
pub const D3DCOMPILE_DEBUG_NAME_FOR_BINARY: UINT = 1 << 23;
