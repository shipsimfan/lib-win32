use crate::{
    d3dcommon::{ID3DBlob, ID3DInclude, D3D_SHADER_MACRO},
    HRESULT, LPCSTR, LPCVOID, SIZE_T, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3dcompiler::D3D_COMPILE_STANDARD_FILE_INCLUDE;
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "D3dcompiler")]
extern "system" {
    /// Compile HLSL code or an effect file into bytecode for a given target.
    ///
    /// # Parameters
    ///  * `src_data` - A pointer to uncompiled shader data; either ASCII HLSL code or a compiled
    ///                 effect.
    ///  * `src_data_size` - Length of `src_data`.
    ///  * `source_name` - You can use this parameter for strings that specify error messages. If
    ///                    not used, set to [`null`].
    ///  * `defines` - An optional array of [`D3D_SHADER_MACRO`] structures that define shader
    ///                macros. Each macro definition contains a name and a null-terminated
    ///                definition. If not used, set to [`null`]. The last structure in the array
    ///                serves as a terminator and must have all members set to [`null`].
    ///  * `include` - Optional. A pointer to an [`ID3DInclude`] for handling include files.
    ///                Setting this to [`null_mut`] will cause a compile error if a shader contains
    ///                a `#include`. You can pass the [`D3D_COMPILE_STANDARD_FILE_INCLUDE`] macro,
    ///                which is a pointer to a default include handler. This default include
    ///                handler includes files that are relative to the current directory and files
    ///                that are relative to the directory of the initial source file. When you use
    ///                [`D3D_COMPILE_STANDARD_FILE_INCLUDE`], you must specify the source file name
    ///                in the `source_name` parameter; the compiler will derive the initial
    ///                relative directory from `source_name`.
    ///  * `entry_point` - The name of the shader entry point function where shader execution
    ///                    begins. When you compile using a `fx` profile (for example, `fx_4_0`,
    ///                    `fx_5_0`, and so on), [`D3DCompile`] ignores `entry_point`. In this
    ///                    case, we recommend that you set `entry_point` to [`null`] because it is
    ///                    good programming practice to set a pointer parameter to [`null`] if the
    ///                    called function will not use it. For all other shader profiles, a valid
    ///                    `entry_point` is required.
    ///  * `target` - A string that specifies the shader target or set of shader features to
    ///               compile against. The shader target can be shader model 2, shader model 3,
    ///               shader model 4, or shader model 5. The target can also be an effect type (for
    ///               example, `fx_4_1`).
    ///  * `flags1` - Flags defined by D3D compile constants.
    ///  * `flags2` - Flags defined by D3D compile effect constants. When you compile a shader and
    ///               not an effect file, [`D3DCompile`] ignores `flags2`; we recommend that you
    ///               set `flags2` to zero because it is good programming practice to set a
    ///               nonpointer parameter to zero if the called function will not use it.
    ///  * `code` - A pointer to a variable that receives a pointer to the [`ID3DBlob`] interface
    ///             that you can use to access the compiled code.
    ///  * `error_msgs` - A pointer to a variable that receives a pointer to the [`ID3DBlob`]
    ///                   interface that you can use to access compiler error messages, or
    ///                   [`null_mut`] if there are no errors.
    ///
    /// # Return Value
    /// Returns one of the Direct3D 11 return codes.
    ///
    /// # Remarks
    /// The difference between [`D3DCompile`] and [`D3DCompile2`] is that the latter method takes
    /// some optional parameters that can be used to control some aspects of how bytecode is
    /// generated. If this extra flexibility is not required, there is no performance gain from
    /// using [`D3DCompile2`].
    pub fn D3DCompile(
        src_data: LPCVOID,
        src_data_size: SIZE_T,
        source_name: LPCSTR,
        defines: *const D3D_SHADER_MACRO,
        include: *mut ID3DInclude,
        entry_point: LPCSTR,
        target: LPCSTR,
        flags1: UINT,
        flags2: UINT,
        code: *mut *mut ID3DBlob,
        error_msgs: *mut *mut ID3DBlob,
    ) -> HRESULT;
}
