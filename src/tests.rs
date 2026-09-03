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

#[test]
fn parses_a_single_segment_response() {
    let json = include_str!("testdata/single_segment.json");
    let translations = super::parse_json(json).expect("single segment response should parse");
    assert_eq!(translations, vec!["Good morning".to_string()]);
}

#[test]
fn parses_a_multi_segment_response() {
    let json = include_str!("testdata/multi_segment.json");
    let translations = super::parse_json(json).expect("multi segment response should parse");
    assert_eq!(
        translations,
        vec!["Hello.".to_string(), "How are you doing?".to_string()]
    );
}

#[test]
fn rejects_a_response_it_does_not_understand() {
    assert!(super::parse_json(r#"[["wrb.fr","MkEWBc","[[]]",null,null,null,"generic"]]"#).is_err());
    assert!(super::parse_json("not json at all").is_err());
}

#[test]
#[ignore = "talks to translate.google.com"]
fn translates_over_the_network() {
    let translations = super::translate(
        "Guten Morgen",
        super::lang::LanguageCode::de,
        super::lang::LanguageCode::en,
    )
    .expect("translate should succeed");
    assert_eq!(translations, vec!["Good morning".to_string()]);
}

#[test]
fn enum_to_string() {
    let lang = super::lang::LanguageCode::zh_CN;
    let lang_str = lang.iso_639();
    assert_eq!(lang_str, "zh-CN");
}
