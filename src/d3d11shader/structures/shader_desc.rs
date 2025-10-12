use crate::{
    d3dcommon::{
        D3D_PRIMITIVE, D3D_PRIMITIVE_TOPOLOGY, D3D_TESSELLATOR_DOMAIN,
        D3D_TESSELLATOR_OUTPUT_PRIMITIVE, D3D_TESSELLATOR_PARTITIONING,
    },
    LPCSTR, UINT,
};
use std::ptr::null;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11shader::ID3D11ShaderReflection;

/// Describes a shader.
///
/// # Remarks
/// A shader is written in HLSL and compiled into an intermediate language by the HLSL compiler.
/// The shader description returns information about the compiled shader. Get a shader description
/// by calling [`ID3D11ShaderReflection::get_desc`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_SHADER_DESC {
    /// Shader version.
    pub version: UINT,

    /// The name of the originator of the shader.
    pub creator: LPCSTR,

    /// Shader compilation/parse flags.
    pub flags: UINT,

    /// The number of shader-constant buffers.
    pub constant_buffers: UINT,

    /// The number of resource (textures and buffers) bound to a shader.
    pub bound_resources: UINT,

    /// The number of parameters in the input signature.
    pub input_parameters: UINT,

    /// The number of parameters in the output signature.
    pub output_parameters: UINT,

    /// The number of intermediate-language instructions in the compiled shader.
    pub instruction_count: UINT,

    /// The number of temporary registers in the compiled shader.
    pub temp_register_count: UINT,

    /// Number of temporary arrays used.
    pub temp_array_count: UINT,

    /// Number of constant defines.
    pub def_count: UINT,

    /// Number of declarations (input + output).
    pub dcl_count: UINT,

    /// Number of non-categorized texture instructions.
    pub texture_normal_instructions: UINT,

    /// Number of texture load instructions
    pub texture_load_instructions: UINT,

    /// Number of texture comparison instructions
    pub texture_comp_instructions: UINT,

    /// Number of texture bias instructions
    pub texture_bias_instructions: UINT,

    /// Number of texture gradient instructions.
    pub texture_gradient_instructions: UINT,

    /// Number of floating point arithmetic instructions used.
    pub float_instruction_count: UINT,

    /// Number of signed integer arithmetic instructions used.
    pub int_instruction_count: UINT,

    /// Number of unsigned integer arithmetic instructions used.
    pub uint_instruction_count: UINT,

    /// Number of static flow control instructions used.
    pub static_flow_control_count: UINT,

    /// Number of dynamic flow control instructions used.
    pub dynamic_flow_control_count: UINT,

    /// Number of macro instructions used.
    pub macro_instruction_count: UINT,

    /// Number of array instructions used.
    pub array_instruction_count: UINT,

    /// Number of cut instructions used.
    pub cut_instruction_count: UINT,

    /// Number of emit instructions used.
    pub emit_instruction_count: UINT,

    /// The [`D3D_PRIMITIVE_TOPOLOGY`]-typed value that represents the geometry shader output
    /// topology.
    pub gs_output_topology: D3D_PRIMITIVE_TOPOLOGY,

    /// Geometry shader maximum output vertex count.
    pub gs_max_output_vertex_count: UINT,

    /// The [`D3D_PRIMITIVE`]-typed value that represents the input primitive for a geometry shader
    /// or hull shader.
    pub input_primitive: D3D_PRIMITIVE,

    /// Number of parameters in the patch-constant signature.
    pub patch_constant_parameters: UINT,

    /// Number of geometry shader instances.
    pub gs_instance_count: UINT,

    /// Number of control points in the hull shader and domain shader.
    pub control_points: UINT,

    /// The [`D3D_TESSELLATOR_OUTPUT_PRIMITIVE`]-typed value that represents the tessellator
    /// output-primitive type.
    pub output_primitive: D3D_TESSELLATOR_OUTPUT_PRIMITIVE,

    /// The [`D3D_TESSELLATOR_PARTITIONING`]-typed value that represents the tessellator
    /// partitioning mode.
    pub partitioning: D3D_TESSELLATOR_PARTITIONING,

    /// The [`D3D_TESSELLATOR_DOMAIN`]-typed value that represents the tessellator domain.
    pub tessellator_domain: D3D_TESSELLATOR_DOMAIN,

    /// Number of barrier instructions in a compute shader.
    pub barrier_instructions: UINT,

    /// Number of interlocked instructions in a compute shader.
    pub interlocked_instructions: UINT,

    /// Number of texture writes in a compute shader.
    pub texture_store_instructions: UINT,
}

impl Default for D3D11_SHADER_DESC {
    fn default() -> Self {
        D3D11_SHADER_DESC {
            version: 0,
            creator: null(),
            flags: 0,
            constant_buffers: 0,
            bound_resources: 0,
            input_parameters: 0,
            output_parameters: 0,
            instruction_count: 0,
            temp_register_count: 0,
            temp_array_count: 0,
            def_count: 0,
            dcl_count: 0,
            texture_normal_instructions: 0,
            texture_load_instructions: 0,
            texture_comp_instructions: 0,
            texture_bias_instructions: 0,
            texture_gradient_instructions: 0,
            float_instruction_count: 0,
            int_instruction_count: 0,
            uint_instruction_count: 0,
            static_flow_control_count: 0,
            dynamic_flow_control_count: 0,
            macro_instruction_count: 0,
            array_instruction_count: 0,
            cut_instruction_count: 0,
            emit_instruction_count: 0,
            gs_output_topology: D3D_PRIMITIVE_TOPOLOGY::Undefined,
            gs_max_output_vertex_count: 0,
            input_primitive: D3D_PRIMITIVE::Undefined,
            patch_constant_parameters: 0,
            gs_instance_count: 0,
            control_points: 0,
            output_primitive: D3D_TESSELLATOR_OUTPUT_PRIMITIVE::Undefined,
            partitioning: D3D_TESSELLATOR_PARTITIONING::Undefined,
            tessellator_domain: D3D_TESSELLATOR_DOMAIN::Undefined,
            barrier_instructions: 0,
            interlocked_instructions: 0,
            texture_store_instructions: 0,
        }
    }
}
