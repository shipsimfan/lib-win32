use crate::WORD;

/// x86
pub const PROCESSOR_ARCHITECTURE_INTEL: WORD = 0;

/// ARM
pub const PROCESSOR_ARCHITECTURE_ARM: WORD = 5;

/// Intel Itanium-based
pub const PROCESSOR_ARCHITECTURE_IA64: WORD = 6;

/// x64 (AMD or Intel)
pub const PROCESSOR_ARCHITECTURE_AMD64: WORD = 9;

/// ARM64
pub const PROCESSOR_ARCHITECTURE_ARM64: WORD = 12;

/// Unknown architecture
pub const PROCESSOR_ARCHITECTURE_UNKNOWN: WORD = 0xFFFF;
