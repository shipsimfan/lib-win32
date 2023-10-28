/// # MAKELANGID macro (winnt.h)
///
/// Creates a language identifier from a primary language identifier and a
/// sublanguage identifier.
///
/// ## Parameters
/// `p`\
/// Primary language identifier. This identifier can be a predefined value or a
/// value for a user-defined primary language. For a user-defined language, the
/// identifier is a value in the range 0x0200 to 0x03FF. All other values are
/// reserved for operating system use.
///
/// `s`\
/// Sublanguage identifier. This parameter can be a predefined sublanguage
/// identifier or a user-defined sublanguage. For a user-defined sublanguage,
/// the identifier is a value in the range 0x20 to 0x3F. All other values are
/// reserved for operating system use.
///
/// ## Return value
/// The computed language identifier
#[macro_export]
macro_rules! MAKELANGID {
    ($p: expr, $s: expr) => {
        (($s as $crate::raw::Word) << 10) | ($p as $crate::raw::Word)
    };
}
