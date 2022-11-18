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

use lang::LanguageCode;
use reqwest::{blocking::Response, header::HeaderMap};
use serde_json::Value;
use urlencoding::encode;

mod lang;

const GOOGLETTSRPC: &str = "MkEWBc";
const GOOGLETRANSLATEURL: &str =
    "https://translate.google.com/_/TranslateWebserverUi/data/batchexecute";

// https://kovatch.medium.com/deciphering-google-batchexecute-74991e4e446c
fn package_rpc(
    text: &str,
    source_language: LanguageCode,
    target_language: LanguageCode,
) -> Vec<u8> {
    format!(
        "f.req={}&",
        encode(
            format!(
                "[[[\"{}\",\"[[\\\"{}\\\",\\\"{}\\\",\\\"{}\\\",true],[1]]\",null,\"generic\"]]]",
                GOOGLETTSRPC,
                text,
                source_language.iso_639(),
                target_language.iso_639(),
            )
            .as_str(),
        )
    )
    .into_bytes()
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
    let mut translations: Vec<String> = vec![];
    let err = "unexpected json structure";
    let outerjson: Value = serde_json::from_str(json)?;
    let innerjson: Value = serde_json::from_str(
        outerjson
            .pointer("/0/2")
            .and_then(|e| e.as_str())
            .ok_or(err)?,
    )?;
    let innermost_json: Value = innerjson.pointer("/1/0/0/5/0/4").ok_or(err)?.clone();
    match innermost_json {
        Value::Array(innermost_json) => {
            for node in innermost_json {
                match node.get(0).ok_or(err)? {
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
    let bytes = package_rpc(text, source_language, target_language);
    let response = web_request(bytes)?;
    let response_text = response.text()?;
    let json = response_text.split('\n').last().ok_or("no last")?;
    let translations = parse_json(json)?;

    Result::Ok(translations)
}

#[cfg(test)]
mod tests;
