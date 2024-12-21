use crate::DWORD;

/// The standard input device. Initially, this is the console input buffer, "CONIN$".
pub const STD_INPUT_HANDLE: DWORD = -10i32 as DWORD;

/// The standard output device. Initially, this is the active console screen buffer, "CONOUT$".
pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;

/// The standard error device. Initially, this is the active console screen buffer, "CONOUT$".
pub const STD_ERROR_HANDLE: DWORD = -12i32 as DWORD;
