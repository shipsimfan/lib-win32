use crate::LANGID;

#[link(name = "Kernel32")]
extern "system" {
    /// Returns the language identifier for the user UI language for the current user. If the
    /// current user has not set a language, [`GetUserDefaultUILanguage`] returns the preferred
    /// language set for the system. If there is no preferred language set for the system, then the
    /// system default UI language (also known as "install language") is returned.
    ///
    /// # Return Value
    /// Returns the language identifier for the user UI language for the current user.
    ///
    /// # Remarks
    /// This function returns only a language identifier. An application can retrieve the language
    /// name using the [`GetUserPreferredUILanguages`] function.
    ///
    /// If the user UI language is part of a Language Interface Pack (LIP) and corresponds to a
    /// supplemental locale, this function returns [`LOCALE_CUSTOM_UI_DEFAULT`].
    ///
    /// Windows Me, Windows 2000, Windows XP, Windows Server 2003: The [`GetUserDefaultUILanguage`]
    /// function retrieves the language identifier for the current user language. If MUI is not
    /// installed on the operating system, the function retrieves the default computer user
    /// interface language.
    pub fn GetUserDefaultUILanguage() -> LANGID;
}
