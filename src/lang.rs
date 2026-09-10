//! Every language Google Translate lists, generated from the language picker on
//! translate.google.com (fetched 2026-09-10).

use std::fmt;
use std::str::FromStr;

/// A language Google Translate can translate from or to.
///
/// [`Language::Auto`] asks Google to detect the source language and is only valid as a source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum Language {
    /// Detect language (`auto`)
    #[default]
    Auto,
    /// Abkhaz (`ab`)
    Abkhaz,
    /// Acehnese (`ace`)
    Acehnese,
    /// Acholi (`ach`)
    Acholi,
    /// Afar (`aa`)
    Afar,
    /// Afrikaans (`af`)
    Afrikaans,
    /// Albanian (`sq`)
    Albanian,
    /// Alur (`alz`)
    Alur,
    /// Amharic (`am`)
    Amharic,
    /// Arabic (`ar`)
    Arabic,
    /// Armenian (`hy`)
    Armenian,
    /// Assamese (`as`)
    Assamese,
    /// Avar (`av`)
    Avar,
    /// Awadhi (`awa`)
    Awadhi,
    /// Aymara (`ay`)
    Aymara,
    /// Azerbaijani (`az`)
    Azerbaijani,
    /// Balinese (`ban`)
    Balinese,
    /// Baluchi (`bal`)
    Baluchi,
    /// Bambara (`bm`)
    Bambara,
    /// Baoulé (`bci`)
    Baoule,
    /// Bashkir (`ba`)
    Bashkir,
    /// Basque (`eu`)
    Basque,
    /// Batak Karo (`btx`)
    BatakKaro,
    /// Batak Simalungun (`bts`)
    BatakSimalungun,
    /// Batak Toba (`bbc`)
    BatakToba,
    /// Belarusian (`be`)
    Belarusian,
    /// Bemba (`bem`)
    Bemba,
    /// Bengali (`bn`)
    Bengali,
    /// Betawi (`bew`)
    Betawi,
    /// Bhojpuri (`bho`)
    Bhojpuri,
    /// Bikol (`bik`)
    Bikol,
    /// Bosnian (`bs`)
    Bosnian,
    /// Breton (`br`)
    Breton,
    /// Bulgarian (`bg`)
    Bulgarian,
    /// Buryat (`bua`)
    Buryat,
    /// Cantonese (`yue`)
    Cantonese,
    /// Catalan (`ca`)
    Catalan,
    /// Cebuano (`ceb`)
    Cebuano,
    /// Chamorro (`ch`)
    Chamorro,
    /// Chechen (`ce`)
    Chechen,
    /// Chichewa (`ny`)
    Chichewa,
    /// Chinese (Simplified) (`zh-CN`)
    ChineseSimplified,
    /// Chinese (Traditional) (`zh-TW`)
    ChineseTraditional,
    /// Chuukese (`chk`)
    Chuukese,
    /// Chuvash (`cv`)
    Chuvash,
    /// Corsican (`co`)
    Corsican,
    /// Crimean Tatar (Cyrillic) (`crh`)
    CrimeanTatarCyrillic,
    /// Crimean Tatar (Latin) (`crh-Latn`)
    CrimeanTatarLatin,
    /// Croatian (`hr`)
    Croatian,
    /// Czech (`cs`)
    Czech,
    /// Danish (`da`)
    Danish,
    /// Dari (`fa-AF`)
    Dari,
    /// Dhivehi (`dv`)
    Dhivehi,
    /// Dinka (`din`)
    Dinka,
    /// Dogri (`doi`)
    Dogri,
    /// Dombe (`dov`)
    Dombe,
    /// Dutch (`nl`)
    Dutch,
    /// Dyula (`dyu`)
    Dyula,
    /// Dzongkha (`dz`)
    Dzongkha,
    /// English (`en`)
    English,
    /// Esperanto (`eo`)
    Esperanto,
    /// Estonian (`et`)
    Estonian,
    /// Ewe (`ee`)
    Ewe,
    /// Faroese (`fo`)
    Faroese,
    /// Fijian (`fj`)
    Fijian,
    /// Filipino (`tl`)
    Filipino,
    /// Finnish (`fi`)
    Finnish,
    /// Fon (`fon`)
    Fon,
    /// French (`fr`)
    French,
    /// French (Canada) (`fr-CA`)
    FrenchCanada,
    /// Frisian (`fy`)
    Frisian,
    /// Friulian (`fur`)
    Friulian,
    /// Fulani (`ff`)
    Fulani,
    /// Ga (`gaa`)
    Ga,
    /// Galician (`gl`)
    Galician,
    /// Georgian (`ka`)
    Georgian,
    /// German (`de`)
    German,
    /// Greek (`el`)
    Greek,
    /// Guarani (`gn`)
    Guarani,
    /// Gujarati (`gu`)
    Gujarati,
    /// Haitian Creole (`ht`)
    HaitianCreole,
    /// Hakha Chin (`cnh`)
    HakhaChin,
    /// Hausa (`ha`)
    Hausa,
    /// Hawaiian (`haw`)
    Hawaiian,
    /// Hebrew (`iw`)
    Hebrew,
    /// Hiligaynon (`hil`)
    Hiligaynon,
    /// Hindi (`hi`)
    Hindi,
    /// Hmong (`hmn`)
    Hmong,
    /// Hungarian (`hu`)
    Hungarian,
    /// Hunsrik (`hrx`)
    Hunsrik,
    /// Iban (`iba`)
    Iban,
    /// Icelandic (`is`)
    Icelandic,
    /// Igbo (`ig`)
    Igbo,
    /// Ilocano (`ilo`)
    Ilocano,
    /// Indonesian (`id`)
    Indonesian,
    /// Inuktut (Latin) (`iu-Latn`)
    InuktutLatin,
    /// Inuktut (Syllabics) (`iu`)
    InuktutSyllabics,
    /// Irish (`ga`)
    Irish,
    /// Italian (`it`)
    Italian,
    /// Jamaican Patois (`jam`)
    JamaicanPatois,
    /// Japanese (`ja`)
    Japanese,
    /// Javanese (`jw`)
    Javanese,
    /// Jingpo (`kac`)
    Jingpo,
    /// Kalaallisut (`kl`)
    Kalaallisut,
    /// Kannada (`kn`)
    Kannada,
    /// Kanuri (`kr`)
    Kanuri,
    /// Kapampangan (`pam`)
    Kapampangan,
    /// Kazakh (`kk`)
    Kazakh,
    /// Khasi (`kha`)
    Khasi,
    /// Khmer (`km`)
    Khmer,
    /// Kiga (`cgg`)
    Kiga,
    /// Kikongo (`kg`)
    Kikongo,
    /// Kinyarwanda (`rw`)
    Kinyarwanda,
    /// Kituba (`ktu`)
    Kituba,
    /// Kokborok (`trp`)
    Kokborok,
    /// Komi (`kv`)
    Komi,
    /// Konkani (`gom`)
    Konkani,
    /// Korean (`ko`)
    Korean,
    /// Krio (`kri`)
    Krio,
    /// Kurdish (Kurmanji) (`ku`)
    KurdishKurmanji,
    /// Kurdish (Sorani) (`ckb`)
    KurdishSorani,
    /// Kyrgyz (`ky`)
    Kyrgyz,
    /// Lao (`lo`)
    Lao,
    /// Latgalian (`ltg`)
    Latgalian,
    /// Latin (`la`)
    Latin,
    /// Latvian (`lv`)
    Latvian,
    /// Ligurian (`lij`)
    Ligurian,
    /// Limburgish (`li`)
    Limburgish,
    /// Lingala (`ln`)
    Lingala,
    /// Lithuanian (`lt`)
    Lithuanian,
    /// Lombard (`lmo`)
    Lombard,
    /// Luganda (`lg`)
    Luganda,
    /// Luo (`luo`)
    Luo,
    /// Luxembourgish (`lb`)
    Luxembourgish,
    /// Macedonian (`mk`)
    Macedonian,
    /// Madurese (`mad`)
    Madurese,
    /// Maithili (`mai`)
    Maithili,
    /// Makassar (`mak`)
    Makassar,
    /// Malagasy (`mg`)
    Malagasy,
    /// Malay (`ms`)
    Malay,
    /// Malay (Jawi) (`ms-Arab`)
    MalayJawi,
    /// Malayalam (`ml`)
    Malayalam,
    /// Maltese (`mt`)
    Maltese,
    /// Mam (`mam`)
    Mam,
    /// Manx (`gv`)
    Manx,
    /// Maori (`mi`)
    Maori,
    /// Marathi (`mr`)
    Marathi,
    /// Marshallese (`mh`)
    Marshallese,
    /// Marwadi (`mwr`)
    Marwadi,
    /// Mauritian Creole (`mfe`)
    MauritianCreole,
    /// Meadow Mari (`chm`)
    MeadowMari,
    /// Meiteilon (Manipuri) (`mni-Mtei`)
    MeiteilonManipuri,
    /// Minang (`min`)
    Minang,
    /// Mizo (`lus`)
    Mizo,
    /// Mongolian (`mn`)
    Mongolian,
    /// Myanmar (Burmese) (`my`)
    MyanmarBurmese,
    /// NKo (`bm-Nkoo`)
    NKo,
    /// Nahuatl (Eastern Huasteca) (`nhe`)
    NahuatlEasternHuasteca,
    /// Ndau (`ndc-ZW`)
    Ndau,
    /// Ndebele (South) (`nr`)
    NdebeleSouth,
    /// Nepalbhasa (Newari) (`new`)
    NepalbhasaNewari,
    /// Nepali (`ne`)
    Nepali,
    /// Norwegian (`no`)
    Norwegian,
    /// Nuer (`nus`)
    Nuer,
    /// Occitan (`oc`)
    Occitan,
    /// Odia (Oriya) (`or`)
    OdiaOriya,
    /// Oromo (`om`)
    Oromo,
    /// Ossetian (`os`)
    Ossetian,
    /// Pangasinan (`pag`)
    Pangasinan,
    /// Papiamento (`pap`)
    Papiamento,
    /// Pashto (`ps`)
    Pashto,
    /// Persian (`fa`)
    Persian,
    /// Polish (`pl`)
    Polish,
    /// Portuguese (Brazil) (`pt`)
    PortugueseBrazil,
    /// Portuguese (Portugal) (`pt-PT`)
    PortuguesePortugal,
    /// Punjabi (Gurmukhi) (`pa`)
    PunjabiGurmukhi,
    /// Punjabi (Shahmukhi) (`pa-Arab`)
    PunjabiShahmukhi,
    /// Qʼeqchiʼ (`kek`)
    QEqchi,
    /// Quechua (`qu`)
    Quechua,
    /// Romani (`rom`)
    Romani,
    /// Romanian (`ro`)
    Romanian,
    /// Rundi (`rn`)
    Rundi,
    /// Russian (`ru`)
    Russian,
    /// Sami (North) (`se`)
    SamiNorth,
    /// Samoan (`sm`)
    Samoan,
    /// Sango (`sg`)
    Sango,
    /// Sanskrit (`sa`)
    Sanskrit,
    /// Santali (Latin) (`sat-Latn`)
    SantaliLatin,
    /// Santali (Ol Chiki) (`sat`)
    SantaliOlChiki,
    /// Scots Gaelic (`gd`)
    ScotsGaelic,
    /// Sepedi (`nso`)
    Sepedi,
    /// Serbian (`sr`)
    Serbian,
    /// Sesotho (`st`)
    Sesotho,
    /// Seychellois Creole (`crs`)
    SeychelloisCreole,
    /// Shan (`shn`)
    Shan,
    /// Shona (`sn`)
    Shona,
    /// Sicilian (`scn`)
    Sicilian,
    /// Silesian (`szl`)
    Silesian,
    /// Sindhi (`sd`)
    Sindhi,
    /// Sinhala (`si`)
    Sinhala,
    /// Slovak (`sk`)
    Slovak,
    /// Slovenian (`sl`)
    Slovenian,
    /// Somali (`so`)
    Somali,
    /// Spanish (`es`)
    Spanish,
    /// Sundanese (`su`)
    Sundanese,
    /// Susu (`sus`)
    Susu,
    /// Swahili (`sw`)
    Swahili,
    /// Swati (`ss`)
    Swati,
    /// Swedish (`sv`)
    Swedish,
    /// Tahitian (`ty`)
    Tahitian,
    /// Tajik (`tg`)
    Tajik,
    /// Tamazight (`ber-Latn`)
    Tamazight,
    /// Tamazight (Tifinagh) (`ber`)
    TamazightTifinagh,
    /// Tamil (`ta`)
    Tamil,
    /// Tatar (`tt`)
    Tatar,
    /// Telugu (`te`)
    Telugu,
    /// Tetum (`tet`)
    Tetum,
    /// Thai (`th`)
    Thai,
    /// Tibetan (`bo`)
    Tibetan,
    /// Tigrinya (`ti`)
    Tigrinya,
    /// Tiv (`tiv`)
    Tiv,
    /// Tok Pisin (`tpi`)
    TokPisin,
    /// Tongan (`to`)
    Tongan,
    /// Tshiluba (`lua`)
    Tshiluba,
    /// Tsonga (`ts`)
    Tsonga,
    /// Tswana (`tn`)
    Tswana,
    /// Tulu (`tcy`)
    Tulu,
    /// Tumbuka (`tum`)
    Tumbuka,
    /// Turkish (`tr`)
    Turkish,
    /// Turkmen (`tk`)
    Turkmen,
    /// Tuvan (`tyv`)
    Tuvan,
    /// Twi (`ak`)
    Twi,
    /// Udmurt (`udm`)
    Udmurt,
    /// Ukrainian (`uk`)
    Ukrainian,
    /// Urdu (`ur`)
    Urdu,
    /// Uyghur (`ug`)
    Uyghur,
    /// Uzbek (`uz`)
    Uzbek,
    /// Venda (`ve`)
    Venda,
    /// Venetian (`vec`)
    Venetian,
    /// Vietnamese (`vi`)
    Vietnamese,
    /// Waray (`war`)
    Waray,
    /// Welsh (`cy`)
    Welsh,
    /// Wolof (`wo`)
    Wolof,
    /// Xhosa (`xh`)
    Xhosa,
    /// Yakut (`sah`)
    Yakut,
    /// Yiddish (`yi`)
    Yiddish,
    /// Yoruba (`yo`)
    Yoruba,
    /// Yucatec Maya (`yua`)
    YucatecMaya,
    /// Zapotec (`zap`)
    Zapotec,
    /// Zulu (`zu`)
    Zulu,
}

impl Language {
    /// Every language, `Auto` first, then alphabetical by English name.
    pub const ALL: &'static [Language] = &[
        Language::Auto,
        Language::Abkhaz,
        Language::Acehnese,
        Language::Acholi,
        Language::Afar,
        Language::Afrikaans,
        Language::Albanian,
        Language::Alur,
        Language::Amharic,
        Language::Arabic,
        Language::Armenian,
        Language::Assamese,
        Language::Avar,
        Language::Awadhi,
        Language::Aymara,
        Language::Azerbaijani,
        Language::Balinese,
        Language::Baluchi,
        Language::Bambara,
        Language::Baoule,
        Language::Bashkir,
        Language::Basque,
        Language::BatakKaro,
        Language::BatakSimalungun,
        Language::BatakToba,
        Language::Belarusian,
        Language::Bemba,
        Language::Bengali,
        Language::Betawi,
        Language::Bhojpuri,
        Language::Bikol,
        Language::Bosnian,
        Language::Breton,
        Language::Bulgarian,
        Language::Buryat,
        Language::Cantonese,
        Language::Catalan,
        Language::Cebuano,
        Language::Chamorro,
        Language::Chechen,
        Language::Chichewa,
        Language::ChineseSimplified,
        Language::ChineseTraditional,
        Language::Chuukese,
        Language::Chuvash,
        Language::Corsican,
        Language::CrimeanTatarCyrillic,
        Language::CrimeanTatarLatin,
        Language::Croatian,
        Language::Czech,
        Language::Danish,
        Language::Dari,
        Language::Dhivehi,
        Language::Dinka,
        Language::Dogri,
        Language::Dombe,
        Language::Dutch,
        Language::Dyula,
        Language::Dzongkha,
        Language::English,
        Language::Esperanto,
        Language::Estonian,
        Language::Ewe,
        Language::Faroese,
        Language::Fijian,
        Language::Filipino,
        Language::Finnish,
        Language::Fon,
        Language::French,
        Language::FrenchCanada,
        Language::Frisian,
        Language::Friulian,
        Language::Fulani,
        Language::Ga,
        Language::Galician,
        Language::Georgian,
        Language::German,
        Language::Greek,
        Language::Guarani,
        Language::Gujarati,
        Language::HaitianCreole,
        Language::HakhaChin,
        Language::Hausa,
        Language::Hawaiian,
        Language::Hebrew,
        Language::Hiligaynon,
        Language::Hindi,
        Language::Hmong,
        Language::Hungarian,
        Language::Hunsrik,
        Language::Iban,
        Language::Icelandic,
        Language::Igbo,
        Language::Ilocano,
        Language::Indonesian,
        Language::InuktutLatin,
        Language::InuktutSyllabics,
        Language::Irish,
        Language::Italian,
        Language::JamaicanPatois,
        Language::Japanese,
        Language::Javanese,
        Language::Jingpo,
        Language::Kalaallisut,
        Language::Kannada,
        Language::Kanuri,
        Language::Kapampangan,
        Language::Kazakh,
        Language::Khasi,
        Language::Khmer,
        Language::Kiga,
        Language::Kikongo,
        Language::Kinyarwanda,
        Language::Kituba,
        Language::Kokborok,
        Language::Komi,
        Language::Konkani,
        Language::Korean,
        Language::Krio,
        Language::KurdishKurmanji,
        Language::KurdishSorani,
        Language::Kyrgyz,
        Language::Lao,
        Language::Latgalian,
        Language::Latin,
        Language::Latvian,
        Language::Ligurian,
        Language::Limburgish,
        Language::Lingala,
        Language::Lithuanian,
        Language::Lombard,
        Language::Luganda,
        Language::Luo,
        Language::Luxembourgish,
        Language::Macedonian,
        Language::Madurese,
        Language::Maithili,
        Language::Makassar,
        Language::Malagasy,
        Language::Malay,
        Language::MalayJawi,
        Language::Malayalam,
        Language::Maltese,
        Language::Mam,
        Language::Manx,
        Language::Maori,
        Language::Marathi,
        Language::Marshallese,
        Language::Marwadi,
        Language::MauritianCreole,
        Language::MeadowMari,
        Language::MeiteilonManipuri,
        Language::Minang,
        Language::Mizo,
        Language::Mongolian,
        Language::MyanmarBurmese,
        Language::NKo,
        Language::NahuatlEasternHuasteca,
        Language::Ndau,
        Language::NdebeleSouth,
        Language::NepalbhasaNewari,
        Language::Nepali,
        Language::Norwegian,
        Language::Nuer,
        Language::Occitan,
        Language::OdiaOriya,
        Language::Oromo,
        Language::Ossetian,
        Language::Pangasinan,
        Language::Papiamento,
        Language::Pashto,
        Language::Persian,
        Language::Polish,
        Language::PortugueseBrazil,
        Language::PortuguesePortugal,
        Language::PunjabiGurmukhi,
        Language::PunjabiShahmukhi,
        Language::QEqchi,
        Language::Quechua,
        Language::Romani,
        Language::Romanian,
        Language::Rundi,
        Language::Russian,
        Language::SamiNorth,
        Language::Samoan,
        Language::Sango,
        Language::Sanskrit,
        Language::SantaliLatin,
        Language::SantaliOlChiki,
        Language::ScotsGaelic,
        Language::Sepedi,
        Language::Serbian,
        Language::Sesotho,
        Language::SeychelloisCreole,
        Language::Shan,
        Language::Shona,
        Language::Sicilian,
        Language::Silesian,
        Language::Sindhi,
        Language::Sinhala,
        Language::Slovak,
        Language::Slovenian,
        Language::Somali,
        Language::Spanish,
        Language::Sundanese,
        Language::Susu,
        Language::Swahili,
        Language::Swati,
        Language::Swedish,
        Language::Tahitian,
        Language::Tajik,
        Language::Tamazight,
        Language::TamazightTifinagh,
        Language::Tamil,
        Language::Tatar,
        Language::Telugu,
        Language::Tetum,
        Language::Thai,
        Language::Tibetan,
        Language::Tigrinya,
        Language::Tiv,
        Language::TokPisin,
        Language::Tongan,
        Language::Tshiluba,
        Language::Tsonga,
        Language::Tswana,
        Language::Tulu,
        Language::Tumbuka,
        Language::Turkish,
        Language::Turkmen,
        Language::Tuvan,
        Language::Twi,
        Language::Udmurt,
        Language::Ukrainian,
        Language::Urdu,
        Language::Uyghur,
        Language::Uzbek,
        Language::Venda,
        Language::Venetian,
        Language::Vietnamese,
        Language::Waray,
        Language::Welsh,
        Language::Wolof,
        Language::Xhosa,
        Language::Yakut,
        Language::Yiddish,
        Language::Yoruba,
        Language::YucatecMaya,
        Language::Zapotec,
        Language::Zulu,
    ];

    /// The code Google uses for this language, for example `zh-CN`.
    pub const fn code(self) -> &'static str {
        match self {
            Language::Auto => "auto",
            Language::Abkhaz => "ab",
            Language::Acehnese => "ace",
            Language::Acholi => "ach",
            Language::Afar => "aa",
            Language::Afrikaans => "af",
            Language::Albanian => "sq",
            Language::Alur => "alz",
            Language::Amharic => "am",
            Language::Arabic => "ar",
            Language::Armenian => "hy",
            Language::Assamese => "as",
            Language::Avar => "av",
            Language::Awadhi => "awa",
            Language::Aymara => "ay",
            Language::Azerbaijani => "az",
            Language::Balinese => "ban",
            Language::Baluchi => "bal",
            Language::Bambara => "bm",
            Language::Baoule => "bci",
            Language::Bashkir => "ba",
            Language::Basque => "eu",
            Language::BatakKaro => "btx",
            Language::BatakSimalungun => "bts",
            Language::BatakToba => "bbc",
            Language::Belarusian => "be",
            Language::Bemba => "bem",
            Language::Bengali => "bn",
            Language::Betawi => "bew",
            Language::Bhojpuri => "bho",
            Language::Bikol => "bik",
            Language::Bosnian => "bs",
            Language::Breton => "br",
            Language::Bulgarian => "bg",
            Language::Buryat => "bua",
            Language::Cantonese => "yue",
            Language::Catalan => "ca",
            Language::Cebuano => "ceb",
            Language::Chamorro => "ch",
            Language::Chechen => "ce",
            Language::Chichewa => "ny",
            Language::ChineseSimplified => "zh-CN",
            Language::ChineseTraditional => "zh-TW",
            Language::Chuukese => "chk",
            Language::Chuvash => "cv",
            Language::Corsican => "co",
            Language::CrimeanTatarCyrillic => "crh",
            Language::CrimeanTatarLatin => "crh-Latn",
            Language::Croatian => "hr",
            Language::Czech => "cs",
            Language::Danish => "da",
            Language::Dari => "fa-AF",
            Language::Dhivehi => "dv",
            Language::Dinka => "din",
            Language::Dogri => "doi",
            Language::Dombe => "dov",
            Language::Dutch => "nl",
            Language::Dyula => "dyu",
            Language::Dzongkha => "dz",
            Language::English => "en",
            Language::Esperanto => "eo",
            Language::Estonian => "et",
            Language::Ewe => "ee",
            Language::Faroese => "fo",
            Language::Fijian => "fj",
            Language::Filipino => "tl",
            Language::Finnish => "fi",
            Language::Fon => "fon",
            Language::French => "fr",
            Language::FrenchCanada => "fr-CA",
            Language::Frisian => "fy",
            Language::Friulian => "fur",
            Language::Fulani => "ff",
            Language::Ga => "gaa",
            Language::Galician => "gl",
            Language::Georgian => "ka",
            Language::German => "de",
            Language::Greek => "el",
            Language::Guarani => "gn",
            Language::Gujarati => "gu",
            Language::HaitianCreole => "ht",
            Language::HakhaChin => "cnh",
            Language::Hausa => "ha",
            Language::Hawaiian => "haw",
            Language::Hebrew => "iw",
            Language::Hiligaynon => "hil",
            Language::Hindi => "hi",
            Language::Hmong => "hmn",
            Language::Hungarian => "hu",
            Language::Hunsrik => "hrx",
            Language::Iban => "iba",
            Language::Icelandic => "is",
            Language::Igbo => "ig",
            Language::Ilocano => "ilo",
            Language::Indonesian => "id",
            Language::InuktutLatin => "iu-Latn",
            Language::InuktutSyllabics => "iu",
            Language::Irish => "ga",
            Language::Italian => "it",
            Language::JamaicanPatois => "jam",
            Language::Japanese => "ja",
            Language::Javanese => "jw",
            Language::Jingpo => "kac",
            Language::Kalaallisut => "kl",
            Language::Kannada => "kn",
            Language::Kanuri => "kr",
            Language::Kapampangan => "pam",
            Language::Kazakh => "kk",
            Language::Khasi => "kha",
            Language::Khmer => "km",
            Language::Kiga => "cgg",
            Language::Kikongo => "kg",
            Language::Kinyarwanda => "rw",
            Language::Kituba => "ktu",
            Language::Kokborok => "trp",
            Language::Komi => "kv",
            Language::Konkani => "gom",
            Language::Korean => "ko",
            Language::Krio => "kri",
            Language::KurdishKurmanji => "ku",
            Language::KurdishSorani => "ckb",
            Language::Kyrgyz => "ky",
            Language::Lao => "lo",
            Language::Latgalian => "ltg",
            Language::Latin => "la",
            Language::Latvian => "lv",
            Language::Ligurian => "lij",
            Language::Limburgish => "li",
            Language::Lingala => "ln",
            Language::Lithuanian => "lt",
            Language::Lombard => "lmo",
            Language::Luganda => "lg",
            Language::Luo => "luo",
            Language::Luxembourgish => "lb",
            Language::Macedonian => "mk",
            Language::Madurese => "mad",
            Language::Maithili => "mai",
            Language::Makassar => "mak",
            Language::Malagasy => "mg",
            Language::Malay => "ms",
            Language::MalayJawi => "ms-Arab",
            Language::Malayalam => "ml",
            Language::Maltese => "mt",
            Language::Mam => "mam",
            Language::Manx => "gv",
            Language::Maori => "mi",
            Language::Marathi => "mr",
            Language::Marshallese => "mh",
            Language::Marwadi => "mwr",
            Language::MauritianCreole => "mfe",
            Language::MeadowMari => "chm",
            Language::MeiteilonManipuri => "mni-Mtei",
            Language::Minang => "min",
            Language::Mizo => "lus",
            Language::Mongolian => "mn",
            Language::MyanmarBurmese => "my",
            Language::NKo => "bm-Nkoo",
            Language::NahuatlEasternHuasteca => "nhe",
            Language::Ndau => "ndc-ZW",
            Language::NdebeleSouth => "nr",
            Language::NepalbhasaNewari => "new",
            Language::Nepali => "ne",
            Language::Norwegian => "no",
            Language::Nuer => "nus",
            Language::Occitan => "oc",
            Language::OdiaOriya => "or",
            Language::Oromo => "om",
            Language::Ossetian => "os",
            Language::Pangasinan => "pag",
            Language::Papiamento => "pap",
            Language::Pashto => "ps",
            Language::Persian => "fa",
            Language::Polish => "pl",
            Language::PortugueseBrazil => "pt",
            Language::PortuguesePortugal => "pt-PT",
            Language::PunjabiGurmukhi => "pa",
            Language::PunjabiShahmukhi => "pa-Arab",
            Language::QEqchi => "kek",
            Language::Quechua => "qu",
            Language::Romani => "rom",
            Language::Romanian => "ro",
            Language::Rundi => "rn",
            Language::Russian => "ru",
            Language::SamiNorth => "se",
            Language::Samoan => "sm",
            Language::Sango => "sg",
            Language::Sanskrit => "sa",
            Language::SantaliLatin => "sat-Latn",
            Language::SantaliOlChiki => "sat",
            Language::ScotsGaelic => "gd",
            Language::Sepedi => "nso",
            Language::Serbian => "sr",
            Language::Sesotho => "st",
            Language::SeychelloisCreole => "crs",
            Language::Shan => "shn",
            Language::Shona => "sn",
            Language::Sicilian => "scn",
            Language::Silesian => "szl",
            Language::Sindhi => "sd",
            Language::Sinhala => "si",
            Language::Slovak => "sk",
            Language::Slovenian => "sl",
            Language::Somali => "so",
            Language::Spanish => "es",
            Language::Sundanese => "su",
            Language::Susu => "sus",
            Language::Swahili => "sw",
            Language::Swati => "ss",
            Language::Swedish => "sv",
            Language::Tahitian => "ty",
            Language::Tajik => "tg",
            Language::Tamazight => "ber-Latn",
            Language::TamazightTifinagh => "ber",
            Language::Tamil => "ta",
            Language::Tatar => "tt",
            Language::Telugu => "te",
            Language::Tetum => "tet",
            Language::Thai => "th",
            Language::Tibetan => "bo",
            Language::Tigrinya => "ti",
            Language::Tiv => "tiv",
            Language::TokPisin => "tpi",
            Language::Tongan => "to",
            Language::Tshiluba => "lua",
            Language::Tsonga => "ts",
            Language::Tswana => "tn",
            Language::Tulu => "tcy",
            Language::Tumbuka => "tum",
            Language::Turkish => "tr",
            Language::Turkmen => "tk",
            Language::Tuvan => "tyv",
            Language::Twi => "ak",
            Language::Udmurt => "udm",
            Language::Ukrainian => "uk",
            Language::Urdu => "ur",
            Language::Uyghur => "ug",
            Language::Uzbek => "uz",
            Language::Venda => "ve",
            Language::Venetian => "vec",
            Language::Vietnamese => "vi",
            Language::Waray => "war",
            Language::Welsh => "cy",
            Language::Wolof => "wo",
            Language::Xhosa => "xh",
            Language::Yakut => "sah",
            Language::Yiddish => "yi",
            Language::Yoruba => "yo",
            Language::YucatecMaya => "yua",
            Language::Zapotec => "zap",
            Language::Zulu => "zu",
        }
    }

    /// The English name Google shows for this language.
    pub const fn name(self) -> &'static str {
        match self {
            Language::Auto => "Detect language",
            Language::Abkhaz => "Abkhaz",
            Language::Acehnese => "Acehnese",
            Language::Acholi => "Acholi",
            Language::Afar => "Afar",
            Language::Afrikaans => "Afrikaans",
            Language::Albanian => "Albanian",
            Language::Alur => "Alur",
            Language::Amharic => "Amharic",
            Language::Arabic => "Arabic",
            Language::Armenian => "Armenian",
            Language::Assamese => "Assamese",
            Language::Avar => "Avar",
            Language::Awadhi => "Awadhi",
            Language::Aymara => "Aymara",
            Language::Azerbaijani => "Azerbaijani",
            Language::Balinese => "Balinese",
            Language::Baluchi => "Baluchi",
            Language::Bambara => "Bambara",
            Language::Baoule => "Baoulé",
            Language::Bashkir => "Bashkir",
            Language::Basque => "Basque",
            Language::BatakKaro => "Batak Karo",
            Language::BatakSimalungun => "Batak Simalungun",
            Language::BatakToba => "Batak Toba",
            Language::Belarusian => "Belarusian",
            Language::Bemba => "Bemba",
            Language::Bengali => "Bengali",
            Language::Betawi => "Betawi",
            Language::Bhojpuri => "Bhojpuri",
            Language::Bikol => "Bikol",
            Language::Bosnian => "Bosnian",
            Language::Breton => "Breton",
            Language::Bulgarian => "Bulgarian",
            Language::Buryat => "Buryat",
            Language::Cantonese => "Cantonese",
            Language::Catalan => "Catalan",
            Language::Cebuano => "Cebuano",
            Language::Chamorro => "Chamorro",
            Language::Chechen => "Chechen",
            Language::Chichewa => "Chichewa",
            Language::ChineseSimplified => "Chinese (Simplified)",
            Language::ChineseTraditional => "Chinese (Traditional)",
            Language::Chuukese => "Chuukese",
            Language::Chuvash => "Chuvash",
            Language::Corsican => "Corsican",
            Language::CrimeanTatarCyrillic => "Crimean Tatar (Cyrillic)",
            Language::CrimeanTatarLatin => "Crimean Tatar (Latin)",
            Language::Croatian => "Croatian",
            Language::Czech => "Czech",
            Language::Danish => "Danish",
            Language::Dari => "Dari",
            Language::Dhivehi => "Dhivehi",
            Language::Dinka => "Dinka",
            Language::Dogri => "Dogri",
            Language::Dombe => "Dombe",
            Language::Dutch => "Dutch",
            Language::Dyula => "Dyula",
            Language::Dzongkha => "Dzongkha",
            Language::English => "English",
            Language::Esperanto => "Esperanto",
            Language::Estonian => "Estonian",
            Language::Ewe => "Ewe",
            Language::Faroese => "Faroese",
            Language::Fijian => "Fijian",
            Language::Filipino => "Filipino",
            Language::Finnish => "Finnish",
            Language::Fon => "Fon",
            Language::French => "French",
            Language::FrenchCanada => "French (Canada)",
            Language::Frisian => "Frisian",
            Language::Friulian => "Friulian",
            Language::Fulani => "Fulani",
            Language::Ga => "Ga",
            Language::Galician => "Galician",
            Language::Georgian => "Georgian",
            Language::German => "German",
            Language::Greek => "Greek",
            Language::Guarani => "Guarani",
            Language::Gujarati => "Gujarati",
            Language::HaitianCreole => "Haitian Creole",
            Language::HakhaChin => "Hakha Chin",
            Language::Hausa => "Hausa",
            Language::Hawaiian => "Hawaiian",
            Language::Hebrew => "Hebrew",
            Language::Hiligaynon => "Hiligaynon",
            Language::Hindi => "Hindi",
            Language::Hmong => "Hmong",
            Language::Hungarian => "Hungarian",
            Language::Hunsrik => "Hunsrik",
            Language::Iban => "Iban",
            Language::Icelandic => "Icelandic",
            Language::Igbo => "Igbo",
            Language::Ilocano => "Ilocano",
            Language::Indonesian => "Indonesian",
            Language::InuktutLatin => "Inuktut (Latin)",
            Language::InuktutSyllabics => "Inuktut (Syllabics)",
            Language::Irish => "Irish",
            Language::Italian => "Italian",
            Language::JamaicanPatois => "Jamaican Patois",
            Language::Japanese => "Japanese",
            Language::Javanese => "Javanese",
            Language::Jingpo => "Jingpo",
            Language::Kalaallisut => "Kalaallisut",
            Language::Kannada => "Kannada",
            Language::Kanuri => "Kanuri",
            Language::Kapampangan => "Kapampangan",
            Language::Kazakh => "Kazakh",
            Language::Khasi => "Khasi",
            Language::Khmer => "Khmer",
            Language::Kiga => "Kiga",
            Language::Kikongo => "Kikongo",
            Language::Kinyarwanda => "Kinyarwanda",
            Language::Kituba => "Kituba",
            Language::Kokborok => "Kokborok",
            Language::Komi => "Komi",
            Language::Konkani => "Konkani",
            Language::Korean => "Korean",
            Language::Krio => "Krio",
            Language::KurdishKurmanji => "Kurdish (Kurmanji)",
            Language::KurdishSorani => "Kurdish (Sorani)",
            Language::Kyrgyz => "Kyrgyz",
            Language::Lao => "Lao",
            Language::Latgalian => "Latgalian",
            Language::Latin => "Latin",
            Language::Latvian => "Latvian",
            Language::Ligurian => "Ligurian",
            Language::Limburgish => "Limburgish",
            Language::Lingala => "Lingala",
            Language::Lithuanian => "Lithuanian",
            Language::Lombard => "Lombard",
            Language::Luganda => "Luganda",
            Language::Luo => "Luo",
            Language::Luxembourgish => "Luxembourgish",
            Language::Macedonian => "Macedonian",
            Language::Madurese => "Madurese",
            Language::Maithili => "Maithili",
            Language::Makassar => "Makassar",
            Language::Malagasy => "Malagasy",
            Language::Malay => "Malay",
            Language::MalayJawi => "Malay (Jawi)",
            Language::Malayalam => "Malayalam",
            Language::Maltese => "Maltese",
            Language::Mam => "Mam",
            Language::Manx => "Manx",
            Language::Maori => "Maori",
            Language::Marathi => "Marathi",
            Language::Marshallese => "Marshallese",
            Language::Marwadi => "Marwadi",
            Language::MauritianCreole => "Mauritian Creole",
            Language::MeadowMari => "Meadow Mari",
            Language::MeiteilonManipuri => "Meiteilon (Manipuri)",
            Language::Minang => "Minang",
            Language::Mizo => "Mizo",
            Language::Mongolian => "Mongolian",
            Language::MyanmarBurmese => "Myanmar (Burmese)",
            Language::NKo => "NKo",
            Language::NahuatlEasternHuasteca => "Nahuatl (Eastern Huasteca)",
            Language::Ndau => "Ndau",
            Language::NdebeleSouth => "Ndebele (South)",
            Language::NepalbhasaNewari => "Nepalbhasa (Newari)",
            Language::Nepali => "Nepali",
            Language::Norwegian => "Norwegian",
            Language::Nuer => "Nuer",
            Language::Occitan => "Occitan",
            Language::OdiaOriya => "Odia (Oriya)",
            Language::Oromo => "Oromo",
            Language::Ossetian => "Ossetian",
            Language::Pangasinan => "Pangasinan",
            Language::Papiamento => "Papiamento",
            Language::Pashto => "Pashto",
            Language::Persian => "Persian",
            Language::Polish => "Polish",
            Language::PortugueseBrazil => "Portuguese (Brazil)",
            Language::PortuguesePortugal => "Portuguese (Portugal)",
            Language::PunjabiGurmukhi => "Punjabi (Gurmukhi)",
            Language::PunjabiShahmukhi => "Punjabi (Shahmukhi)",
            Language::QEqchi => "Qʼeqchiʼ",
            Language::Quechua => "Quechua",
            Language::Romani => "Romani",
            Language::Romanian => "Romanian",
            Language::Rundi => "Rundi",
            Language::Russian => "Russian",
            Language::SamiNorth => "Sami (North)",
            Language::Samoan => "Samoan",
            Language::Sango => "Sango",
            Language::Sanskrit => "Sanskrit",
            Language::SantaliLatin => "Santali (Latin)",
            Language::SantaliOlChiki => "Santali (Ol Chiki)",
            Language::ScotsGaelic => "Scots Gaelic",
            Language::Sepedi => "Sepedi",
            Language::Serbian => "Serbian",
            Language::Sesotho => "Sesotho",
            Language::SeychelloisCreole => "Seychellois Creole",
            Language::Shan => "Shan",
            Language::Shona => "Shona",
            Language::Sicilian => "Sicilian",
            Language::Silesian => "Silesian",
            Language::Sindhi => "Sindhi",
            Language::Sinhala => "Sinhala",
            Language::Slovak => "Slovak",
            Language::Slovenian => "Slovenian",
            Language::Somali => "Somali",
            Language::Spanish => "Spanish",
            Language::Sundanese => "Sundanese",
            Language::Susu => "Susu",
            Language::Swahili => "Swahili",
            Language::Swati => "Swati",
            Language::Swedish => "Swedish",
            Language::Tahitian => "Tahitian",
            Language::Tajik => "Tajik",
            Language::Tamazight => "Tamazight",
            Language::TamazightTifinagh => "Tamazight (Tifinagh)",
            Language::Tamil => "Tamil",
            Language::Tatar => "Tatar",
            Language::Telugu => "Telugu",
            Language::Tetum => "Tetum",
            Language::Thai => "Thai",
            Language::Tibetan => "Tibetan",
            Language::Tigrinya => "Tigrinya",
            Language::Tiv => "Tiv",
            Language::TokPisin => "Tok Pisin",
            Language::Tongan => "Tongan",
            Language::Tshiluba => "Tshiluba",
            Language::Tsonga => "Tsonga",
            Language::Tswana => "Tswana",
            Language::Tulu => "Tulu",
            Language::Tumbuka => "Tumbuka",
            Language::Turkish => "Turkish",
            Language::Turkmen => "Turkmen",
            Language::Tuvan => "Tuvan",
            Language::Twi => "Twi",
            Language::Udmurt => "Udmurt",
            Language::Ukrainian => "Ukrainian",
            Language::Urdu => "Urdu",
            Language::Uyghur => "Uyghur",
            Language::Uzbek => "Uzbek",
            Language::Venda => "Venda",
            Language::Venetian => "Venetian",
            Language::Vietnamese => "Vietnamese",
            Language::Waray => "Waray",
            Language::Welsh => "Welsh",
            Language::Wolof => "Wolof",
            Language::Xhosa => "Xhosa",
            Language::Yakut => "Yakut",
            Language::Yiddish => "Yiddish",
            Language::Yoruba => "Yoruba",
            Language::YucatecMaya => "Yucatec Maya",
            Language::Zapotec => "Zapotec",
            Language::Zulu => "Zulu",
        }
    }

    /// Looks a language up by its Google code, case-insensitively. Also accepts
    /// the underscore spelling of region codes such as `zh_CN`.
    pub fn from_code(code: &str) -> Option<Language> {
        let wanted = code.trim().replace('_', "-");
        Language::ALL
            .iter()
            .copied()
            .find(|language| language.code().eq_ignore_ascii_case(&wanted))
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

/// The code was not one Google Translate lists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownLanguage(pub String);

impl fmt::Display for UnknownLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown language code {:?}", self.0)
    }
}

impl std::error::Error for UnknownLanguage {}

impl FromStr for Language {
    type Err = UnknownLanguage;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Language::from_code(s).ok_or_else(|| UnknownLanguage(s.to_owned()))
    }
}
