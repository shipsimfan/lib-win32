use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{ID3D11Device, D3D11_FEATURE, D3D11_MAP},
    d3d11_1::{ID3D11DeviceContext1, D3D11_RASTERIZER_DESC1},
    d3dcommon::D3D_FEATURE_LEVEL,
    FALSE, TRUE,
};

/// Describes Direct3D 11.1 feature options in the current graphics driver.
///
/// # Remarks
/// If a Microsoft Direct3D device supports feature level 11.1 ([`D3D_FEATURE_LEVEL::_11_1`]), when
/// you call [`ID3D11Device::check_feature_support`] with [`D3D11_FEATURE::D3D11Options`],
/// [`ID3D11Device::check_feature_support`] returns a pointer to
/// [`D3D11_FEATURE_DATA_D3D11_OPTIONS`] with all member set to [`TRUE`] except the
/// `sad4_shader_instructions` and `extended_double_shader_instructions` members, which are
/// optionally supported by the hardware and driver and therefore can be [`TRUE`] or [`FALSE`].
///
/// Feature level 11.1 provides the following additional features:
///  - UAVs at every shader stage with 64 UAV bind slots instead of 8.
///  - Target-independent rasterization, which enables you to set the `forced_sample_count` member
///    of [`D3D11_RASTERIZER_DESC1`] to 1, 4, 8, or 16 and to render to RTVs with a single sample.
///  - UAV-only rendering with the `forced_sample_count` member of [`D3D11_RASTERIZER_DESC1`] set
///    to up to 16 (only up to 8 for feature level 11).
///
/// The runtime always sets the following groupings of members identically. That is, all the values
/// in a grouping are [`TRUE`] or [`FALSE`] together:
///  - `discard_apis_seen_by_driver` and `flags_for_update_and_copy_seen_by_driver`
///  - `clear_view`, `copy_with_overlap`, `constant_buffer_partial_update`,
///    `constant_buffer_offsetting`, and `map_no_overwrite_on_dynamic_constant_buffer`
///  - `map_no_overwrite_on_dynamic_buffer_srv` and `multisample_rtv_with_forced_sample_count_one`

#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS {
    /// Specifies whether logic operations are available in blend state. The runtime sets this
    /// member to [`TRUE`] if logic operations are available in blend state and [`FALSE`]
    /// otherwise. This member is [`FALSE`] for feature level 9.1, 9.2, and 9.3. This member is
    /// optional for feature level 10, 10.1, and 11. This member is [`TRUE`] for feature level
    /// 11.1.
    pub output_merger_logic_op: BOOL,

    /// Specifies whether the driver can render with no render target views (RTVs) or depth stencil
    /// views (DSVs), and only unordered access views (UAVs) bound. The runtime sets this member to
    /// [`TRUE`] if the driver can render with no RTVs or DSVs and only UAVs bound and [`FALSE`]
    /// otherwise. If [`TRUE`], you can set the `forced_sample_count` member of
    /// [`D3D11_RASTERIZER_DESC1`] to 1, 4, or 8 when you render with no RTVs or DSV and only UAVs
    /// bound. For feature level 11.1, this member is always [`TRUE`] and you can also set
    /// `forced_sample_count` to 16 in addition to 1, 4, or 8. The default value of
    /// `forced_sample_count` is 0, which means the same as if the value is set to 1. You can
    /// always set `forced_sample_count` to 0 or 1 for UAV-only rendering independently of how this
    /// member is set.
    pub uav_only_rendering_forced_sample_count: BOOL,

    /// Specifies whether the driver supports the [`ID3D11DeviceContext1::discard_view`] and
    /// [`ID3D11DeviceContext1::discard_resource`] methods. The runtime sets this member to
    /// [`TRUE`] if the driver supports these methods and [`FALSE`] otherwise. How this member is
    /// set does not indicate whether the driver actually uses these methods; that is, the driver
    /// might ignore these methods if they are not useful to the hardware. If [`FALSE`], the
    /// runtime does not expose these methods to the driver because the driver does not support
    /// them. You can monitor this member during development to rule out legacy drivers on hardware
    /// where these methods might have otherwise been beneficial. You are not required to write
    /// separate code paths based on whether this member is [`TRUE`] or [`FALSE`]; you can call
    /// these methods whenever applicable.
    pub discard_apis_seen_by_driver: BOOL,

    /// Specifies whether the driver supports new semantics for copy and update that are exposed by
    /// the [`ID3D11DeviceContext1::copy_subresource_region1`] and
    /// [`ID3D11DeviceContext1::update_subresource1`] methods. The runtime sets this member to
    /// [`TRUE`] if the driver supports new semantics for copy and update. The runtime sets this
    /// member to [`FALSE`] only for legacy drivers. The runtime handles this member similarly to
    /// the `discard_apis_seen_by_driver` member.
    pub flags_for_update_and_copy_seen_by_driver: BOOL,

    /// Specifies whether the driver supports the [`ID3D11DeviceContext1::clear_view`] method. The
    /// runtime sets this member to [`TRUE`] if the driver supports this method and [`FALSE`]
    /// otherwise. If [`FALSE`], the runtime does not expose this method to the driver because the
    /// driver does not support it.
    pub clear_view: BOOL,

    /// Specifies whether you can call [`ID3D11DeviceContext1::copy_subresource_region1`] with
    /// overlapping source and destination rectangles. The runtime sets this member to [`TRUE`] if
    /// you can call [`ID3D11DeviceContext1::copy_subresource_region1`] with overlapping source and
    /// destination rectangles and [`FALSE`] otherwise. If [`FALSE`], the runtime does not expose
    /// this method to the driver because the driver does not support it.
    pub copy_with_overlap: BOOL,

    /// Specifies whether the driver supports partial updates of constant buffers. The runtime sets
    /// this member to [`TRUE`] if the driver supports partial updates of constant buffers and
    /// [`FALSE`] otherwise. If [`FALSE`], the runtime does not expose this operation to the driver
    /// because the driver does not support it.
    pub constant_buffer_partial_update: BOOL,

    /// Specifies whether the driver supports new semantics for setting offsets in constant buffers
    /// for a shader. The runtime sets this member to [`TRUE`] if the driver supports allowing you
    /// to specify offsets when you call new methods like the
    /// [`ID3D11DeviceContext1::vs_set_constant_buffers1`] method and [`FALSE`] otherwise. If
    /// [`FALSE`], the runtime does not expose this operation to the driver because the driver does
    /// not support it.
    pub constant_buffer_offsetting: BOOL,

    /// Specifies whether you can call [`ID3D11DeviceContext::map`] with
    /// [`D3D11_MAP::WriteNoOverwrite`] on a dynamic constant buffer (that is, whether the driver
    /// supports this operation). The runtime sets this member to [`TRUE`] if the driver supports
    /// this operation and [`FALSE`] otherwise. If [`FALSE`], the runtime fails this method because
    /// the driver does not support the operation.
    pub map_no_overwrite_on_dynamic_constant_buffer: BOOL,

    /// Specifies whether you can call [`ID3D11DeviceContext::map`] with
    /// [`D3D11_MAP::WriteNoOverwrite`] on a dynamic buffer SRV (that is, whether the driver
    /// supports this operation). The runtime sets this member to [`TRUE`] if the driver supports
    /// this operation and [`FALSE`] otherwise. If [`FALSE`], the runtime fails this method because
    /// the driver does not support the operation.
    pub map_no_overwrite_on_dynamic_buffer_srv: BOOL,

    /// Specifies whether the driver supports multisample rendering when you render with RTVs
    /// bound. If [`TRUE`], you can set the `forced_sample_count` member of
    /// [`D3D11_RASTERIZER_DESC1`] to 1 with a multisample RTV bound. The driver can support this
    /// option on feature level 10 and higher. If [`FALSE`], the rasterizer-state creation will
    /// fail because the driver is legacy or the feature level is too low.
    pub multisample_rtv_with_forced_sample_count_one: BOOL,

    /// Specifies whether the hardware and driver support the msad4 intrinsic function in shaders.
    /// The runtime sets this member to [`TRUE`] if the hardware and driver support calls to msad4
    /// intrinsic functions in shaders. If [`FALSE`], the driver is legacy or the hardware does not
    /// support the option; the runtime will fail shader creation for shaders that use msad4.
    pub sad4_shader_instructions: BOOL,

    /// Specifies whether the hardware and driver support the fma intrinsic function and other
    /// extended doubles instructions (DDIV and DRCP) in shaders. The fma intrinsic function emits
    /// an extended doubles DFMA instruction. The runtime sets this member to [`TRUE`] if the
    /// hardware and driver support extended doubles instructions in shaders (shader model 5 and
    /// higher). Support of this option implies support of basic double-precision shader
    /// instructions as well. You can use the [`D3D11_FEATURE::Doubles`] value to query for support
    /// of double-precision shaders. If [`FALSE`], the hardware and driver do not support the
    /// option; the runtime will fail shader creation for shaders that use extended doubles
    /// instructions.
    pub extended_double_shader_instructions: BOOL,

    /// Specifies whether the hardware and driver have extended support for shared Texture 2D
    /// resource types and formats. The runtime sets this member to [`TRUE`] if the hardware and
    /// driver support extended Texture2D resource sharing.
    pub extended_resource_sharing: BOOL,
}

impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS {
    fn default() -> Self {
        D3D11_FEATURE_DATA_D3D11_OPTIONS {
            output_merger_logic_op: 0,
            uav_only_rendering_forced_sample_count: 0,
            discard_apis_seen_by_driver: 0,
            flags_for_update_and_copy_seen_by_driver: 0,
            clear_view: 0,
            copy_with_overlap: 0,
            constant_buffer_partial_update: 0,
            constant_buffer_offsetting: 0,
            map_no_overwrite_on_dynamic_constant_buffer: 0,
            map_no_overwrite_on_dynamic_buffer_srv: 0,
            multisample_rtv_with_forced_sample_count_one: 0,
            sad4_shader_instructions: 0,
            extended_double_shader_instructions: 0,
            extended_resource_sharing: 0,
        }
    }
}
