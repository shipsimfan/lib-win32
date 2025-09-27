/// In the Windows API (with some exceptions discussed in the following paragraphs), the maximum
/// length for a path is [`MAX_PATH`], which is defined as 260 characters. A local path is
/// structured in the following order: drive letter, colon, backslash, name components separated by
/// backslashes, and a terminating null character. For example, the maximum path on drive D is
/// "D:\some 256-character path string<\NUL>" where "<\NUL>" represents the invisible terminating
/// null character for the current system codepage. (The characters < > are used here for visual
/// clarity and cannot be part of a valid path string.)
pub const MAX_PATH: usize = 260;
