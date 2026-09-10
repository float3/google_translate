use super::{Error, Language, Translation, build_request, parse_envelope, parse_response};

#[test]
fn parses_a_single_segment_response() {
    let json = include_str!("testdata/single_segment.json");
    let translation =
        parse_envelope(json, Language::English).expect("single segment response should parse");
    assert_eq!(translation.segments, ["Good morning"]);
    assert_eq!(translation.text, "Good morning");
    assert_eq!(translation.target, Language::English);
}

#[test]
fn parses_a_multi_segment_response() {
    let json = include_str!("testdata/multi_segment.json");
    let translation =
        parse_envelope(json, Language::English).expect("multi segment response should parse");
    assert_eq!(translation.segments, ["Hello.", "How are you doing?"]);
    assert_eq!(translation.text, "Hello. How are you doing?");
}

#[test]
fn parses_a_whole_body_and_reports_the_detected_language() {
    let body = include_str!("testdata/full_response.txt");
    let translation = parse_response(body, Language::German).expect("recorded body should parse");
    assert_eq!(translation.segments, ["test"]);
    assert_eq!(translation.detected_source, Some(Language::English));
    assert_eq!(translation.target, Language::German);
}

#[test]
fn rejects_a_response_it_does_not_understand() {
    let bad = parse_envelope(
        r#"[["wrb.fr","MkEWBc","[[]]",null,null,null,"generic"]]"#,
        Language::English,
    );
    assert!(matches!(bad, Err(Error::UnexpectedResponse(_))));
    assert!(matches!(
        parse_envelope("not json at all", Language::English),
        Err(Error::Json(_))
    ));
    assert!(matches!(
        parse_response("", Language::English),
        Err(Error::UnexpectedResponse(_))
    ));
}

#[test]
fn validates_the_request_before_sending_it() {
    assert!(matches!(
        build_request("   ", Language::Auto, Language::English),
        Err(Error::EmptyText)
    ));
    let long = "a".repeat(5001);
    assert!(matches!(
        build_request(&long, Language::Auto, Language::English),
        Err(Error::TextTooLong { chars: 5001 })
    ));
    assert!(matches!(
        build_request("hi", Language::Auto, Language::Auto),
        Err(Error::AutoTarget)
    ));
}

#[test]
fn escapes_the_text_inside_the_request() {
    let request = build_request(r#"say "hi" \ bye"#, Language::Auto, Language::German)
        .expect("valid request");
    let outer: serde_json::Value = serde_json::from_str(&request).expect("request is JSON");
    let inner = outer
        .pointer("/0/0/1")
        .and_then(|v| v.as_str())
        .expect("inner payload is a string");
    let inner: serde_json::Value = serde_json::from_str(inner).expect("inner payload is JSON");
    assert_eq!(
        inner.pointer("/0/0").and_then(|v| v.as_str()),
        Some(r#"say "hi" \ bye"#)
    );
    assert_eq!(inner.pointer("/0/1").and_then(|v| v.as_str()), Some("auto"));
    assert_eq!(inner.pointer("/0/2").and_then(|v| v.as_str()), Some("de"));
}

#[test]
fn language_codes_round_trip() {
    for language in Language::ALL {
        assert_eq!(
            language.code().parse::<Language>().as_ref(),
            Ok(language),
            "{}",
            language.code()
        );
    }
    assert_eq!("zh-CN".parse(), Ok(Language::ChineseSimplified));
    assert_eq!("ZH_cn".parse(), Ok(Language::ChineseSimplified));
    assert_eq!(Language::ChineseTraditional.to_string(), "zh-TW");
    assert_eq!(Language::ChineseTraditional.name(), "Chinese (Traditional)");
    assert!("klingon".parse::<Language>().is_err());
    assert_eq!(Language::default(), Language::Auto);
}

#[test]
#[ignore = "talks to translate.google.com"]
fn translates_over_the_network() {
    let translation: Translation =
        super::translate("Guten Morgen", Language::Auto, Language::English)
            .expect("translate should succeed");
    assert_eq!(translation.text, "Good morning");
    assert_eq!(translation.detected_source, Some(Language::German));
}
