use crate::UINT;

/// Compile the effects (.fx) file to a child effect. Child effects have no initializers for any
/// shared values because these child effects are initialized in the master effect (the effect
/// pool).
pub const D3DCOMPILE_EFFECT_CHILD_EFFECT: UINT = 1 << 0;

/// Disables performance mode and allows for mutable state objects.
///
/// By default, performance mode is enabled. Performance mode disallows mutable state objects by
/// preventing non-literal expressions from appearing in state object definitions.
pub const D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS: UINT = 1 << 1;
