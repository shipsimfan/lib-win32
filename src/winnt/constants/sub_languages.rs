use crate::DWORD;

/// Language neutral
pub const SUBLANG_NEUTRAL: DWORD = 0x00;

/// User default
pub const SUBLANG_DEFAULT: DWORD = 0x01;

/// System default
pub const SUBLANG_SYS_DEFAULT: DWORD = 0x02;

/// Default custom language/locale
pub const SUBLANG_CUSTOM_DEFAULT: DWORD = 0x03;

/// Custom language/locale
pub const SUBLANG_CUSTOM_UNSPECIFIED: DWORD = 0x04;

/// Default custom MUI language/locale
pub const SUBLANG_UI_CUSTOM_DEFAULT: DWORD = 0x05;

/// Afrikaans (South Africa) 0x0436 af-ZA
pub const SUBLANG_AFRIKAANS_SOUTH_AFRICA: DWORD = 0x01;

/// Albanian (Albania) 0x041c sq-AL
pub const SUBLANG_ALBANIAN_ALBANIA: DWORD = 0x01;

/// Alsatian (France) 0x0484
pub const SUBLANG_ALSATIAN_FRANCE: DWORD = 0x01;

/// Amharic (Ethiopia) 0x045e
pub const SUBLANG_AMHARIC_ETHIOPIA: DWORD = 0x01;

/// Arabic (Saudi Arabia)
pub const SUBLANG_ARABIC_SAUDI_ARABIA: DWORD = 0x01;

/// Arabic (Iraq)
pub const SUBLANG_ARABIC_IRAQ: DWORD = 0x02;

/// Arabic (Egypt)
pub const SUBLANG_ARABIC_EGYPT: DWORD = 0x03;

/// Arabic (Libya)
pub const SUBLANG_ARABIC_LIBYA: DWORD = 0x04;

/// Arabic (Algeria)
pub const SUBLANG_ARABIC_ALGERIA: DWORD = 0x05;

/// Arabic (Morocco)
pub const SUBLANG_ARABIC_MOROCCO: DWORD = 0x06;

/// Arabic (Tunisia)
pub const SUBLANG_ARABIC_TUNISIA: DWORD = 0x07;

/// Arabic (Oman)
pub const SUBLANG_ARABIC_OMAN: DWORD = 0x08;

/// Arabic (Yemen)
pub const SUBLANG_ARABIC_YEMEN: DWORD = 0x09;

/// Arabic (Syria)
pub const SUBLANG_ARABIC_SYRIA: DWORD = 0x0A;

/// Arabic (Jordan)
pub const SUBLANG_ARABIC_JORDAN: DWORD = 0x0B;

/// Arabic (Lebanon)
pub const SUBLANG_ARABIC_LEBANON: DWORD = 0x0C;

/// Arabic (Kuwait)
pub const SUBLANG_ARABIC_KUWAIT: DWORD = 0x0D;

/// Arabic (U.A.E)
pub const SUBLANG_ARABIC_UAE: DWORD = 0x0E;

/// Arabic (Bahrain)
pub const SUBLANG_ARABIC_BAHRAIN: DWORD = 0x0F;

/// Arabic (Qatar)
pub const SUBLANG_ARABIC_QATAR: DWORD = 0x10;

/// Armenian (Armenia) 0x042b hy-AM
pub const SUBLANG_ARMENIAN_ARMENIA: DWORD = 0x01;

/// Assamese (India) 0x044d
pub const SUBLANG_ASSAMESE_INDIA: DWORD = 0x01;

/// Azeri (Latin) - for Azerbaijani, SUBLANG_AZERBAIJANI_AZERBAIJAN_LATIN preferred
pub const SUBLANG_AZERI_LATIN: DWORD = 0x01;

/// Azeri (Cyrillic) - for Azerbaijani, SUBLANG_AZERBAIJANI_AZERBAIJAN_CYRILLIC preferred
pub const SUBLANG_AZERI_CYRILLIC: DWORD = 0x02;

/// Azerbaijani (Azerbaijan, Latin)
pub const SUBLANG_AZERBAIJANI_AZERBAIJAN_LATIN: DWORD = 0x01;

/// Azerbaijani (Azerbaijan, Cyrillic)
pub const SUBLANG_AZERBAIJANI_AZERBAIJAN_CYRILLIC: DWORD = 0x02;

/// Bangla (India)
pub const SUBLANG_BANGLA_INDIA: DWORD = 0x01;

/// Bangla (Bangladesh)
pub const SUBLANG_BANGLA_BANGLADESH: DWORD = 0x02;

/// Bashkir (Russia) 0x046d ba-RU
pub const SUBLANG_BASHKIR_RUSSIA: DWORD = 0x01;

/// Basque (Basque) 0x042d eu-ES
pub const SUBLANG_BASQUE_BASQUE: DWORD = 0x01;

/// Belarusian (Belarus) 0x0423 be-BY
pub const SUBLANG_BELARUSIAN_BELARUS: DWORD = 0x01;

/// Bengali (India) - Note some prefer SUBLANG_BANGLA_INDIA
pub const SUBLANG_BENGALI_INDIA: DWORD = 0x01;

/// Bengali (Bangladesh) - Note some prefer SUBLANG_BANGLA_BANGLADESH
pub const SUBLANG_BENGALI_BANGLADESH: DWORD = 0x02;

/// Bosnian (Bosnia and Herzegovina - Latin) 0x141a bs-BA-Latn
pub const SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_LATIN: DWORD = 0x05;

/// Bosnian (Bosnia and Herzegovina - Cyrillic) 0x201a bs-BA-Cyrl
pub const SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_CYRILLIC: DWORD = 0x08;

/// Breton (France) 0x047e
pub const SUBLANG_BRETON_FRANCE: DWORD = 0x01;

/// Bulgarian (Bulgaria) 0x0402
pub const SUBLANG_BULGARIAN_BULGARIA: DWORD = 0x01;

/// Catalan (Catalan) 0x0403
pub const SUBLANG_CATALAN_CATALAN: DWORD = 0x01;

/// Central Kurdish (Iraq) 0x0492 ku-Arab-IQ
pub const SUBLANG_CENTRAL_KURDISH_IRAQ: DWORD = 0x01;

/// Cherokee (Cherokee) 0x045c chr-Cher-US
pub const SUBLANG_CHEROKEE_CHEROKEE: DWORD = 0x01;

/// Chinese (Taiwan) 0x0404 zh-TW
pub const SUBLANG_CHINESE_TRADITIONAL: DWORD = 0x01;

/// Chinese (PR China) 0x0804 zh-CN
pub const SUBLANG_CHINESE_SIMPLIFIED: DWORD = 0x02;

/// Chinese (Hong Kong S.A.R., P.R.C.) 0x0c04 zh-HK
pub const SUBLANG_CHINESE_HONGKONG: DWORD = 0x03;

/// Chinese (Singapore) 0x1004 zh-SG
pub const SUBLANG_CHINESE_SINGAPORE: DWORD = 0x04;

/// Chinese (Macau S.A.R.) 0x1404 zh-MO
pub const SUBLANG_CHINESE_MACAU: DWORD = 0x05;

/// Corsican (France) 0x0483
pub const SUBLANG_CORSICAN_FRANCE: DWORD = 0x01;

/// Czech (Czech Republic) 0x0405
pub const SUBLANG_CZECH_CZECH_REPUBLIC: DWORD = 0x01;

/// Croatian (Croatia)
pub const SUBLANG_CROATIAN_CROATIA: DWORD = 0x01;

/// Croatian (Bosnia and Herzegovina - Latin) 0x101a hr-BA
pub const SUBLANG_CROATIAN_BOSNIA_HERZEGOVINA_LATIN: DWORD = 0x04;

/// Danish (Denmark) 0x0406
pub const SUBLANG_DANISH_DENMARK: DWORD = 0x01;

/// Dari (Afghanistan)
pub const SUBLANG_DARI_AFGHANISTAN: DWORD = 0x01;

/// Divehi (Maldives) 0x0465 div-MV
pub const SUBLANG_DIVEHI_MALDIVES: DWORD = 0x01;

/// Dutch
pub const SUBLANG_DUTCH: DWORD = 0x01;

/// Dutch (Belgian)
pub const SUBLANG_DUTCH_BELGIAN: DWORD = 0x02;

/// English (USA)
pub const SUBLANG_ENGLISH_US: DWORD = 0x01;

/// English (UK)
pub const SUBLANG_ENGLISH_UK: DWORD = 0x02;

/// English (Australian)
pub const SUBLANG_ENGLISH_AUS: DWORD = 0x03;

/// English (Canadian)
pub const SUBLANG_ENGLISH_CAN: DWORD = 0x04;

/// English (New Zealand)
pub const SUBLANG_ENGLISH_NZ: DWORD = 0x05;

/// English (Irish)
pub const SUBLANG_ENGLISH_EIRE: DWORD = 0x06;

/// English (South Africa)
pub const SUBLANG_ENGLISH_SOUTH_AFRICA: DWORD = 0x07;

/// English (Jamaica)
pub const SUBLANG_ENGLISH_JAMAICA: DWORD = 0x08;

/// English (Caribbean)
pub const SUBLANG_ENGLISH_CARIBBEAN: DWORD = 0x09;

/// English (Belize)
pub const SUBLANG_ENGLISH_BELIZE: DWORD = 0x0A;

/// English (Trinidad)
pub const SUBLANG_ENGLISH_TRINIDAD: DWORD = 0x0B;

/// English (Zimbabwe)
pub const SUBLANG_ENGLISH_ZIMBABWE: DWORD = 0x0C;

/// English (Philippines)
pub const SUBLANG_ENGLISH_PHILIPPINES: DWORD = 0x0D;

/// English (India)
pub const SUBLANG_ENGLISH_INDIA: DWORD = 0x10;

/// English (Malaysia)
pub const SUBLANG_ENGLISH_MALAYSIA: DWORD = 0x11;

/// English (Singapore)
pub const SUBLANG_ENGLISH_SINGAPORE: DWORD = 0x12;

/// Estonian (Estonia) 0x0425 et-EE
pub const SUBLANG_ESTONIAN_ESTONIA: DWORD = 0x01;

/// Faroese (Faroe Islands) 0x0438 fo-FO
pub const SUBLANG_FAEROESE_FAROE_ISLANDS: DWORD = 0x01;

/// Filipino (Philippines) 0x0464 fil-PH
pub const SUBLANG_FILIPINO_PHILIPPINES: DWORD = 0x01;

/// Finnish (Finland) 0x040b
pub const SUBLANG_FINNISH_FINLAND: DWORD = 0x01;

/// French
pub const SUBLANG_FRENCH: DWORD = 0x01;

/// French (Belgian)
pub const SUBLANG_FRENCH_BELGIAN: DWORD = 0x02;

/// French (Canadian)
pub const SUBLANG_FRENCH_CANADIAN: DWORD = 0x03;

/// French (Swiss)
pub const SUBLANG_FRENCH_SWISS: DWORD = 0x04;

/// French (Luxembourg)
pub const SUBLANG_FRENCH_LUXEMBOURG: DWORD = 0x05;

/// French (Monaco)
pub const SUBLANG_FRENCH_MONACO: DWORD = 0x06;

/// Frisian (Netherlands) 0x0462 fy-NL
pub const SUBLANG_FRISIAN_NETHERLANDS: DWORD = 0x01;

/// Fulah (Senegal) 0x0867 ff-Latn-SN
pub const SUBLANG_FULAH_SENEGAL: DWORD = 0x02;

/// Galician (Galician) 0x0456 gl-ES
pub const SUBLANG_GALICIAN_GALICIAN: DWORD = 0x01;

/// Georgian (Georgia) 0x0437 ka-GE
pub const SUBLANG_GEORGIAN_GEORGIA: DWORD = 0x01;

/// German
pub const SUBLANG_GERMAN: DWORD = 0x01;

/// German (Swiss)
pub const SUBLANG_GERMAN_SWISS: DWORD = 0x02;

/// German (Austrian)
pub const SUBLANG_GERMAN_AUSTRIAN: DWORD = 0x03;

/// German (Luxembourg)
pub const SUBLANG_GERMAN_LUXEMBOURG: DWORD = 0x04;

/// German (Liechtenstein)
pub const SUBLANG_GERMAN_LIECHTENSTEIN: DWORD = 0x05;

/// Greek (Greece)
pub const SUBLANG_GREEK_GREECE: DWORD = 0x01;

/// Greenlandic (Greenland) 0x046f kl-GL
pub const SUBLANG_GREENLANDIC_GREENLAND: DWORD = 0x01;

/// Gujarati (India (Gujarati Script)) 0x0447 gu-IN
pub const SUBLANG_GUJARATI_INDIA: DWORD = 0x01;

/// Hausa (Latin, Nigeria) 0x0468 ha-NG-Latn
pub const SUBLANG_HAUSA_NIGERIA_LATIN: DWORD = 0x01;

/// Hawiian (US) 0x0475 haw-US
pub const SUBLANG_HAWAIIAN_US: DWORD = 0x01;

/// Hebrew (Israel) 0x040d
pub const SUBLANG_HEBREW_ISRAEL: DWORD = 0x01;

/// Hindi (India) 0x0439 hi-IN
pub const SUBLANG_HINDI_INDIA: DWORD = 0x01;

/// Hungarian (Hungary) 0x040e
pub const SUBLANG_HUNGARIAN_HUNGARY: DWORD = 0x01;

/// Icelandic (Iceland) 0x040f
pub const SUBLANG_ICELANDIC_ICELAND: DWORD = 0x01;

/// Igbo (Nigeria) 0x0470 ig-NG
pub const SUBLANG_IGBO_NIGERIA: DWORD = 0x01;

/// Indonesian (Indonesia) 0x0421 id-ID
pub const SUBLANG_INDONESIAN_INDONESIA: DWORD = 0x01;

/// Inuktitut (Syllabics) (Canada) 0x045d iu-CA-Cans
pub const SUBLANG_INUKTITUT_CANADA: DWORD = 0x01;

/// Inuktitut (Canada - Latin)
pub const SUBLANG_INUKTITUT_CANADA_LATIN: DWORD = 0x02;

/// Irish (Ireland)
pub const SUBLANG_IRISH_IRELAND: DWORD = 0x02;

/// Italian
pub const SUBLANG_ITALIAN: DWORD = 0x01;

/// Italian (Swiss)
pub const SUBLANG_ITALIAN_SWISS: DWORD = 0x02;

/// Japanese (Japan) 0x0411
pub const SUBLANG_JAPANESE_JAPAN: DWORD = 0x01;

/// Kannada (India (Kannada Script)) 0x044b kn-IN
pub const SUBLANG_KANNADA_INDIA: DWORD = 0x01;

/// Kashmiri (South Asia)
pub const SUBLANG_KASHMIRI_SASIA: DWORD = 0x02;

/// For app compatibility only
pub const SUBLANG_KASHMIRI_INDIA: DWORD = 0x02;

/// Kazakh (Kazakhstan) 0x043f kk-KZ
pub const SUBLANG_KAZAK_KAZAKHSTAN: DWORD = 0x01;

/// Khmer (Cambodia) 0x0453 kh-KH
pub const SUBLANG_KHMER_CAMBODIA: DWORD = 0x01;

/// K'iche (Guatemala)
pub const SUBLANG_KICHE_GUATEMALA: DWORD = 0x01;

/// Kinyarwanda (Rwanda) 0x0487 rw-RW
pub const SUBLANG_KINYARWANDA_RWANDA: DWORD = 0x01;

/// Konkani (India) 0x0457 kok-IN
pub const SUBLANG_KONKANI_INDIA: DWORD = 0x01;

/// Korean (Extended Wansung)
pub const SUBLANG_KOREAN: DWORD = 0x01;

/// Kyrgyz (Kyrgyzstan) 0x0440 ky-KG
pub const SUBLANG_KYRGYZ_KYRGYZSTAN: DWORD = 0x01;

/// Lao (Lao PDR) 0x0454 lo-LA
pub const SUBLANG_LAO_LAO: DWORD = 0x01;

/// Latvian (Latvia) 0x0426 lv-LV
pub const SUBLANG_LATVIAN_LATVIA: DWORD = 0x01;

/// Lithuanian
pub const SUBLANG_LITHUANIAN: DWORD = 0x01;

/// Lower Sorbian (Germany) 0x082e wee-DE
pub const SUBLANG_LOWER_SORBIAN_GERMANY: DWORD = 0x02;

/// Luxembourgish (Luxembourg) 0x046e lb-LU
pub const SUBLANG_LUXEMBOURGISH_LUXEMBOURG: DWORD = 0x01;

/// Macedonian (Macedonia (FYROM)) 0x042f mk-MK
pub const SUBLANG_MACEDONIAN_MACEDONIA: DWORD = 0x01;

/// Malay (Malaysia)
pub const SUBLANG_MALAY_MALAYSIA: DWORD = 0x01;

/// Malay (Brunei Darussalam)
pub const SUBLANG_MALAY_BRUNEI_DARUSSALAM: DWORD = 0x02;

/// Malayalam (India (Malayalam Script) ) 0x044c ml-IN
pub const SUBLANG_MALAYALAM_INDIA: DWORD = 0x01;

/// Maltese (Malta) 0x043a mt-MT
pub const SUBLANG_MALTESE_MALTA: DWORD = 0x01;

/// Maori (New Zealand) 0x0481 mi-NZ
pub const SUBLANG_MAORI_NEW_ZEALAND: DWORD = 0x01;

/// Mapudungun (Chile) 0x047a arn-CL
pub const SUBLANG_MAPUDUNGUN_CHILE: DWORD = 0x01;

/// Marathi (India) 0x044e mr-IN
pub const SUBLANG_MARATHI_INDIA: DWORD = 0x01;

/// Mohawk (Mohawk) 0x047c moh-CA
pub const SUBLANG_MOHAWK_MOHAWK: DWORD = 0x01;

/// Mongolian (Cyrillic, Mongolia)
pub const SUBLANG_MONGOLIAN_CYRILLIC_MONGOLIA: DWORD = 0x01;

/// Mongolian (PRC)
pub const SUBLANG_MONGOLIAN_PRC: DWORD = 0x02;

/// Nepali (India)
pub const SUBLANG_NEPALI_INDIA: DWORD = 0x02;

/// Nepali (Nepal) 0x0461 ne-NP
pub const SUBLANG_NEPALI_NEPAL: DWORD = 0x01;

/// Norwegian (Bokmal)
pub const SUBLANG_NORWEGIAN_BOKMAL: DWORD = 0x01;

/// Norwegian (Nynorsk)
pub const SUBLANG_NORWEGIAN_NYNORSK: DWORD = 0x02;

/// Occitan (France) 0x0482 oc-FR
pub const SUBLANG_OCCITAN_FRANCE: DWORD = 0x01;

/// Odia (India (Odia Script)) 0x0448 or-IN
pub const SUBLANG_ODIA_INDIA: DWORD = 0x01;

/// Deprecated: use SUBLANG_ODIA_INDIA instead
pub const SUBLANG_ORIYA_INDIA: DWORD = 0x01;

/// Pashto (Afghanistan)
pub const SUBLANG_PASHTO_AFGHANISTAN: DWORD = 0x01;

/// Persian (Iran) 0x0429 fa-IR
pub const SUBLANG_PERSIAN_IRAN: DWORD = 0x01;

/// Polish (Poland) 0x0415
pub const SUBLANG_POLISH_POLAND: DWORD = 0x01;

/// Portuguese
pub const SUBLANG_PORTUGUESE: DWORD = 0x02;

/// Portuguese (Brazil)
pub const SUBLANG_PORTUGUESE_BRAZILIAN: DWORD = 0x01;

/// Deprecated: Use SUBLANG_FULAH_SENEGAL instead
pub const SUBLANG_PULAR_SENEGAL: DWORD = 0x02;

/// Punjabi (India (Gurmukhi Script)) 0x0446 pa-IN
pub const SUBLANG_PUNJABI_INDIA: DWORD = 0x01;

/// Punjabi (Pakistan (Arabic Script)) 0x0846 pa-Arab-PK
pub const SUBLANG_PUNJABI_PAKISTAN: DWORD = 0x02;

/// Quechua (Bolivia)
pub const SUBLANG_QUECHUA_BOLIVIA: DWORD = 0x01;

/// Quechua (Ecuador)
pub const SUBLANG_QUECHUA_ECUADOR: DWORD = 0x02;

/// Quechua (Peru)
pub const SUBLANG_QUECHUA_PERU: DWORD = 0x03;

/// Romanian (Romania) 0x0418
pub const SUBLANG_ROMANIAN_ROMANIA: DWORD = 0x01;

/// Romansh (Switzerland) 0x0417 rm-CH
pub const SUBLANG_ROMANSH_SWITZERLAND: DWORD = 0x01;

/// Russian (Russia) 0x0419
pub const SUBLANG_RUSSIAN_RUSSIA: DWORD = 0x01;

/// Sakha (Russia) 0x0485 sah-RU
pub const SUBLANG_SAKHA_RUSSIA: DWORD = 0x01;

/// Northern Sami (Norway)
pub const SUBLANG_SAMI_NORTHERN_NORWAY: DWORD = 0x01;

/// Northern Sami (Sweden)
pub const SUBLANG_SAMI_NORTHERN_SWEDEN: DWORD = 0x02;

/// Northern Sami (Finland)
pub const SUBLANG_SAMI_NORTHERN_FINLAND: DWORD = 0x03;

/// Lule Sami (Norway)
pub const SUBLANG_SAMI_LULE_NORWAY: DWORD = 0x04;

/// Lule Sami (Sweden)
pub const SUBLANG_SAMI_LULE_SWEDEN: DWORD = 0x05;

/// Southern Sami (Norway)
pub const SUBLANG_SAMI_SOUTHERN_NORWAY: DWORD = 0x06;

/// Southern Sami (Sweden)
pub const SUBLANG_SAMI_SOUTHERN_SWEDEN: DWORD = 0x07;

/// Skolt Sami (Finland)
pub const SUBLANG_SAMI_SKOLT_FINLAND: DWORD = 0x08;

/// Inari Sami (Finland)
pub const SUBLANG_SAMI_INARI_FINLAND: DWORD = 0x09;

/// Sanskrit (India) 0x044f sa-IN
pub const SUBLANG_SANSKRIT_INDIA: DWORD = 0x01;

/// Scottish Gaelic (United Kingdom) 0x0491 gd-GB
pub const SUBLANG_SCOTTISH_GAELIC: DWORD = 0x01;

/// Serbian (Bosnia and Herzegovina - Latin)
pub const SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_LATIN: DWORD = 0x06;

/// Serbian (Bosnia and Herzegovina - Cyrillic)
pub const SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_CYRILLIC: DWORD = 0x07;

/// Serbian (Montenegro - Latn)
pub const SUBLANG_SERBIAN_MONTENEGRO_LATIN: DWORD = 0x0B;

/// Serbian (Montenegro - Cyrillic)
pub const SUBLANG_SERBIAN_MONTENEGRO_CYRILLIC: DWORD = 0x0C;

/// Serbian (Serbia - Latin)
pub const SUBLANG_SERBIAN_SERBIA_LATIN: DWORD = 0x09;

/// Serbian (Serbia - Cyrillic)
pub const SUBLANG_SERBIAN_SERBIA_CYRILLIC: DWORD = 0x0A;

/// Croatian (Croatia) 0x041a hr-HR
pub const SUBLANG_SERBIAN_CROATIA: DWORD = 0x01;

/// Serbian (Latin)
pub const SUBLANG_SERBIAN_LATIN: DWORD = 0x02;

/// Serbian (Cyrillic)
pub const SUBLANG_SERBIAN_CYRILLIC: DWORD = 0x03;

/// Sindhi (India) reserved 0x0459
pub const SUBLANG_SINDHI_INDIA: DWORD = 0x01;

/// Sindhi (Pakistan) 0x0859 sd-Arab-PK
pub const SUBLANG_SINDHI_PAKISTAN: DWORD = 0x02;

/// For app compatibility only
pub const SUBLANG_SINDHI_AFGHANISTAN: DWORD = 0x02;

/// Sinhalese (Sri Lanka)
pub const SUBLANG_SINHALESE_SRI_LANKA: DWORD = 0x01;

/// Northern Sotho (South Africa)
pub const SUBLANG_SOTHO_NORTHERN_SOUTH_AFRICA: DWORD = 0x01;

/// Slovak (Slovakia) 0x041b sk-SK
pub const SUBLANG_SLOVAK_SLOVAKIA: DWORD = 0x01;

/// Slovenian (Slovenia) 0x0424 sl-SI
pub const SUBLANG_SLOVENIAN_SLOVENIA: DWORD = 0x01;

/// Spanish (Castilian)
pub const SUBLANG_SPANISH: DWORD = 0x01;

/// Spanish (Mexico)
pub const SUBLANG_SPANISH_MEXICAN: DWORD = 0x02;

/// Spanish (Modern)
pub const SUBLANG_SPANISH_MODERN: DWORD = 0x03;

/// Spanish (Guatemala)
pub const SUBLANG_SPANISH_GUATEMALA: DWORD = 0x04;

/// Spanish (Costa Rica)
pub const SUBLANG_SPANISH_COSTA_RICA: DWORD = 0x05;

/// Spanish (Panama)
pub const SUBLANG_SPANISH_PANAMA: DWORD = 0x06;

/// Spanish (Dominican Republic)
pub const SUBLANG_SPANISH_DOMINICAN_REPUBLIC: DWORD = 0x07;

/// Spanish (Venezuela)
pub const SUBLANG_SPANISH_VENEZUELA: DWORD = 0x08;

/// Spanish (Colombia)
pub const SUBLANG_SPANISH_COLOMBIA: DWORD = 0x09;

/// Spanish (Peru)
pub const SUBLANG_SPANISH_PERU: DWORD = 0x0A;

/// Spanish (Argentina)
pub const SUBLANG_SPANISH_ARGENTINA: DWORD = 0x0B;

/// Spanish (Ecuador)
pub const SUBLANG_SPANISH_ECUADOR: DWORD = 0x0C;

/// Spanish (Chile)
pub const SUBLANG_SPANISH_CHILE: DWORD = 0x0D;

/// Spanish (Uruguay)
pub const SUBLANG_SPANISH_URUGUAY: DWORD = 0x0E;

/// Spanish (Paraguay)
pub const SUBLANG_SPANISH_PARAGUAY: DWORD = 0x0F;

/// Spanish (Bolivia)
pub const SUBLANG_SPANISH_BOLIVIA: DWORD = 0x10;

/// Spanish (El Salvador)
pub const SUBLANG_SPANISH_EL_SALVADOR: DWORD = 0x11;

/// Spanish (Honduras)
pub const SUBLANG_SPANISH_HONDURAS: DWORD = 0x12;

/// Spanish (Nicaragua)
pub const SUBLANG_SPANISH_NICARAGUA: DWORD = 0x13;

/// Spanish (Puerto Rico)
pub const SUBLANG_SPANISH_PUERTO_RICO: DWORD = 0x14;

/// Spanish (United States)
pub const SUBLANG_SPANISH_US: DWORD = 0x15;

/// Swahili (Kenya) 0x0441 sw-KE
pub const SUBLANG_SWAHILI_KENYA: DWORD = 0x01;

/// Swedish
pub const SUBLANG_SWEDISH: DWORD = 0x01;

/// Swedish (Finland)
pub const SUBLANG_SWEDISH_FINLAND: DWORD = 0x02;

/// Syriac (Syria) 0x045a syr-SY
pub const SUBLANG_SYRIAC_SYRIA: DWORD = 0x01;

/// Tajik (Tajikistan) 0x0428 tg-TJ-Cyrl
pub const SUBLANG_TAJIK_TAJIKISTAN: DWORD = 0x01;

/// Tamazight (Latin, Algeria) 0x085f tzm-Latn-DZ
pub const SUBLANG_TAMAZIGHT_ALGERIA_LATIN: DWORD = 0x02;

/// Tamazight (Tifinagh) 0x105f tzm-Tfng-MA
pub const SUBLANG_TAMAZIGHT_MOROCCO_TIFINAGH: DWORD = 0x04;

/// Tamil (India)
pub const SUBLANG_TAMIL_INDIA: DWORD = 0x01;

/// Tamil (Sri Lanka) 0x0849 ta-LK
pub const SUBLANG_TAMIL_SRI_LANKA: DWORD = 0x02;

/// Tatar (Russia) 0x0444 tt-RU
pub const SUBLANG_TATAR_RUSSIA: DWORD = 0x01;

/// Telugu (India (Telugu Script)) 0x044a te-IN
pub const SUBLANG_TELUGU_INDIA: DWORD = 0x01;

/// Thai (Thailand) 0x041e th-TH
pub const SUBLANG_THAI_THAILAND: DWORD = 0x01;

/// Tibetan (PRC)
pub const SUBLANG_TIBETAN_PRC: DWORD = 0x01;

/// Tigrigna (Eritrea)
pub const SUBLANG_TIGRIGNA_ERITREA: DWORD = 0x02;

/// Tigrinya (Eritrea) 0x0873 ti-ER (preferred spelling)
pub const SUBLANG_TIGRINYA_ERITREA: DWORD = 0x02;

/// Tigrinya (Ethiopia) 0x0473 ti-ET
pub const SUBLANG_TIGRINYA_ETHIOPIA: DWORD = 0x01;

/// Setswana / Tswana (Botswana) 0x0832 tn-BW
pub const SUBLANG_TSWANA_BOTSWANA: DWORD = 0x02;

/// Setswana / Tswana (South Africa) 0x0432 tn-ZA
pub const SUBLANG_TSWANA_SOUTH_AFRICA: DWORD = 0x01;

/// Turkish (Turkey) 0x041f tr-TR
pub const SUBLANG_TURKISH_TURKEY: DWORD = 0x01;

/// Turkmen (Turkmenistan) 0x0442 tk-TM
pub const SUBLANG_TURKMEN_TURKMENISTAN: DWORD = 0x01;

/// Uighur (PRC) 0x0480 ug-CN
pub const SUBLANG_UIGHUR_PRC: DWORD = 0x01;

/// Ukrainian (Ukraine) 0x0422 uk-UA
pub const SUBLANG_UKRAINIAN_UKRAINE: DWORD = 0x01;

/// Upper Sorbian (Germany) 0x042e wen-DE
pub const SUBLANG_UPPER_SORBIAN_GERMANY: DWORD = 0x01;

/// Urdu (Pakistan)
pub const SUBLANG_URDU_PAKISTAN: DWORD = 0x01;

/// Urdu (India)
pub const SUBLANG_URDU_INDIA: DWORD = 0x02;

/// Uzbek (Latin)
pub const SUBLANG_UZBEK_LATIN: DWORD = 0x01;

/// Uzbek (Cyrillic)
pub const SUBLANG_UZBEK_CYRILLIC: DWORD = 0x02;

/// Valencian (Valencia) 0x0803 ca-ES-Valencia
pub const SUBLANG_VALENCIAN_VALENCIA: DWORD = 0x02;

/// Vietnamese (Vietnam) 0x042a vi-VN
pub const SUBLANG_VIETNAMESE_VIETNAM: DWORD = 0x01;

/// Welsh (United Kingdom) 0x0452 cy-GB
pub const SUBLANG_WELSH_UNITED_KINGDOM: DWORD = 0x01;

/// Wolof (Senegal)
pub const SUBLANG_WOLOF_SENEGAL: DWORD = 0x01;

/// IsiXhosa / Xhosa (South Africa) 0x0434 xh-ZA
pub const SUBLANG_XHOSA_SOUTH_AFRICA: DWORD = 0x01;

/// Deprecated: use SUBLANG_SAKHA_RUSSIA instead
pub const SUBLANG_YAKUT_RUSSIA: DWORD = 0x01;

/// Yi (PRC)) 0x0478
pub const SUBLANG_YI_PRC: DWORD = 0x01;

/// Yoruba (Nigeria) 046a yo-NG
pub const SUBLANG_YORUBA_NIGERIA: DWORD = 0x01;

/// IsiZulu / Zulu (South Africa) 0x0435 zu-ZA
pub const SUBLANG_ZULU_SOUTH_AFRICA: DWORD = 0x01;
