// Copyright (C) 2022  float3

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// use phf::phf_map;
use reqwest::{blocking::Response, header::HeaderMap};
use serde_json::Value;
use strum_macros::Display;
use urlencoding::encode;

const GOOGLETTSRPC: &str = "MkEWBc";
const GOOGLETRANSLATEURL: &str =
    "https://translate.google.com/_/TranslateWebserverUi/data/batchexecute";

// all languages listed in view-source:https://translate.google.com as of 2022-11-15
#[allow(non_camel_case_types)]
#[derive(Display, Debug)]
pub enum LanguageCode {
    auto,
    af,
    sq,
    am,
    ar,
    hy,
    r#as,
    ay,
    az,
    bm,
    eu,
    be,
    bn,
    bho,
    bs,
    bg,
    ca,
    ceb,
    ny,
    zh_CN,
    zh_TW,
    co,
    hr,
    cs,
    da,
    dv,
    doi,
    nl,
    en,
    eo,
    et,
    ee,
    tl,
    fi,
    fr,
    fy,
    gl,
    ka,
    de,
    el,
    gn,
    gu,
    ht,
    ha,
    haw,
    iw,
    hi,
    hmn,
    hu,
    is,
    ig,
    ilo,
    id,
    ga,
    it,
    ja,
    jw,
    kn,
    kk,
    km,
    rw,
    gom,
    ko,
    kri,
    ku,
    ckb,
    ky,
    lo,
    la,
    lv,
    ln,
    lt,
    lg,
    lb,
    mk,
    mai,
    mg,
    ms,
    ml,
    mt,
    mi,
    mr,
    mni_Mtei,
    lus,
    mn,
    my,
    ne,
    no,
    or,
    om,
    ps,
    fa,
    pl,
    pt,
    pa,
    qu,
    ro,
    ru,
    sm,
    sa,
    gd,
    nso,
    sr,
    st,
    sn,
    sd,
    si,
    sk,
    sl,
    so,
    es,
    su,
    sw,
    sv,
    tg,
    ta,
    tt,
    te,
    th,
    ti,
    ts,
    tr,
    tk,
    ak,
    uk,
    ur,
    ug,
    uz,
    vi,
    cy,
    xh,
    yi,
    yo,
    zu,
}

// all languages listed in view-source:https://translate.google.com as of 2022-11-15
// pub static LANGUAGE_CODES: phf::Map<&'static str, &'static str> = phf_map! {
//     "auto" => "Detect language",
//     "af" => "Afrikaans",
//     "sq" => "Albanian",
//     "am" => "Amharic",
//     "ar" => "Arabic",
//     "hy" => "Armenian",
//     "as" => "Assamese",
//     "ay" => "Aymara",
//     "az" => "Azerbaijani",
//     "bm" => "Bambara",
//     "eu" => "Basque",
//     "be" => "Belarusian",
//     "bn" => "Bengali",
//     "bho" => "Bhojpuri",
//     "bs" => "Bosnian",
//     "bg" => "Bulgarian",
//     "ca" => "Catalan",
//     "ceb" => "Cebuano",
//     "ny" => "Chichewa",
//     "zh-CN" => "Chinese (Simplified)",
//     "zh-TW" => "Chinese (Traditional)",
//     "co" => "Corsican",
//     "hr" => "Croatian",
//     "cs" => "Czech",
//     "da" => "Danish",
//     "dv" => "Dhivehi",
//     "doi" => "Dogri",
//     "nl" => "Dutch",
//     "en" => "English",
//     "eo" => "Esperanto",
//     "et" => "Estonian",
//     "ee" => "Ewe",
//     "tl" => "Filipino",
//     "fi" => "Finnish",
//     "fr" => "French",
//     "fy" => "Frisian",
//     "gl" => "Galician",
//     "ka" => "Georgian",
//     "de" => "German",
//     "el" => "Greek",
//     "gn" => "Guarani",
//     "gu" => "Gujarati",
//     "ht" => "Haitian Creole",
//     "ha" => "Hausa",
//     "haw" => "Hawaiian",
//     "iw" => "Hebrew",
//     "hi" => "Hindi",
//     "hmn" => "Hmong",
//     "hu" => "Hungarian",
//     "is" => "Icelandic",
//     "ig" => "Igbo",
//     "ilo" => "Ilocano",
//     "id" => "Indonesian",
//     "ga" => "Irish",
//     "it" => "Italian",
//     "ja" => "Japanese",
//     "jw" => "Javanese",
//     "kn" => "Kannada",
//     "kk" => "Kazakh",
//     "km" => "Khmer",
//     "rw" => "Kinyarwanda",
//     "gom" => "Konkani",
//     "ko" => "Korean",
//     "kri" => "Krio",
//     "ku" => "Kurdish (Kurmanji)",
//     "ckb" => "Kurdish (Sorani)",
//     "ky" => "Kyrgyz",
//     "lo" => "Lao",
//     "la" => "Latin",
//     "lv" => "Latvian",
//     "ln" => "Lingala",
//     "lt" => "Lithuanian",
//     "lg" => "Luganda",
//     "lb" => "Luxembourgish",
//     "mk" => "Macedonian",
//     "mai" => "Maithili",
//     "mg" => "Malagasy",
//     "ms" => "Malay",
//     "ml" => "Malayalam",
//     "mt" => "Maltese",
//     "mi" => "Maori",
//     "mr" => "Marathi",
//     "mni-Mtei" => "Meiteilon (Manipuri)",
//     "lus" => "Mizo",
//     "mn" => "Mongolian",
//     "my" => "Myanmar (Burmese)",
//     "ne" => "Nepali",
//     "no" => "Norwegian",
//     "or" => "Odia (Oriya)",
//     "om" => "Oromo",
//     "ps" => "Pashto",
//     "fa" => "Persian",
//     "pl" => "Polish",
//     "pt" => "Portuguese",
//     "pa" => "Punjabi",
//     "qu" => "Quechua",
//     "ro" => "Romanian",
//     "ru" => "Russian",
//     "sm" => "Samoan",
//     "sa" => "Sanskrit",
//     "gd" => "Scots Gaelic",
//     "nso" => "Sepedi",
//     "sr" => "Serbian",
//     "st" => "Sesotho",
//     "sn" => "Shona",
//     "sd" => "Sindhi",
//     "si" => "Sinhala",
//     "sk" => "Slovak",
//     "sl" => "Slovenian",
//     "so" => "Somali",
//     "es" => "Spanish",
//     "su" => "Sundanese",
//     "sw" => "Swahili",
//     "sv" => "Swedish",
//     "tg" => "Tajik",
//     "ta" => "Tamil",
//     "tt" => "Tatar",
//     "te" => "Telugu",
//     "th" => "Thai",
//     "ti" => "Tigrinya",
//     "ts" => "Tsonga",
//     "tr" => "Turkish",
//     "tk" => "Turkmen",
//     "ak" => "Twi",
//     "uk" => "Ukrainian",
//     "ur" => "Urdu",
//     "ug" => "Uyghur",
//     "uz" => "Uzbek",
//     "vi" => "Vietnamese",
//     "cy" => "Welsh",
//     "xh" => "Xhosa",
//     "yi" => "Yiddish",
//     "yo" => "Yoruba",
//     "zu" => "Zulu"
// };

// https://kovatch.medium.com/deciphering-google-batchexecute-74991e4e446c
fn package_rpc(text: &str, source_language: LanguageCode, target_language: LanguageCode) -> String {
    format!(
        "f.req={}&",
        encode(
            format!(
                "[[[\"{}\",\"[[\\\"{}\\\",\\\"{}\\\",\\\"{}\\\",true],[1]]\",null,\"generic\"]]]",
                GOOGLETTSRPC,
                text,
                source_language.to_string().replace('_', "-"),
                target_language.to_string().replace('_', "-")
            )
            .as_str(),
        )
    )
}

fn web_request(bytes: Vec<u8>) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        reqwest::header::REFERER,
        reqwest::header::HeaderValue::from_static("https://translate.google.com/"),
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/47.0.2526.106 Safari/537.36"
        )
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static(
            "application/x-www-form-urlencoded;charset=utf-8",
        ),
    );
    headers.insert(
        reqwest::header::CONTENT_LENGTH,
        reqwest::header::HeaderValue::from_str(bytes.len().to_string().as_str())?,
    );
    let response: Response = client
        .post(GOOGLETRANSLATEURL)
        .headers(headers)
        .body(bytes)
        .send()?;
    Result::Ok(response)
}

fn parse_json(json: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let outerjson: Value = serde_json::from_str(json)?;
    let innerjson: Value =
        serde_json::from_str(outerjson[0][2].as_str().ok_or("as_str() failed")?)?;
    let mut translations: Vec<String> = vec![];
    let innermost_json: &Value = innerjson[1][0][0][5][0]
        .get(4)
        .ok_or("unexpected json structure")?;
    match innermost_json {
        Value::Array(innermost_json) => {
            for node in innermost_json {
                match &node[0] {
                    Value::String(translation) => translations.push(translation.to_string()),
                    _ => println!("other"),
                }
            }
        }
        _ => println!("other"),
    }
    Result::Ok(translations)
}

// text has to between in the range of [1,5000]
pub fn translate(
    text: &str,
    source_language: LanguageCode,
    target_language: LanguageCode,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if text.is_empty() {
        return Result::Err("text is empty".into());
    };
    if text.len() > 5000 {
        return Result::Err("text can not be longer than 5000 characters".into());
    };
    let bytes = package_rpc(text, source_language, target_language).into_bytes();
    let response = web_request(bytes)?;
    let response_text = response.text()?;
    let json = response_text.split('\n').last().ok_or("no last")?;
    let translations = parse_json(json)?;

    Result::Ok(translations)
}

#[cfg(test)]
mod tests;
