//! Translate text with Google Translate's web endpoint. No API key, blocking I/O.
//!
//! ```no_run
//! use google_translate::{Language, translate};
//!
//! let result = translate("Guten Morgen. Wie geht es dir?", Language::German, Language::English)?;
//! assert_eq!(result.text, "Good morning. How are you doing?");
//! # Ok::<(), google_translate::Error>(())
//! ```
//!
//! This drives the same `batchexecute` RPC the translate.google.com page uses, so it can break
//! whenever Google changes that page. Keep requests modest.

use std::fmt;
use std::time::Duration;

use reqwest::blocking::Client;
use serde_json::{Value, json};

pub use lang::{Language, UnknownLanguage};

pub mod lang;

const RPC_ID: &str = "MkEWBc";
const ENDPOINT: &str = "https://translate.google.com/_/TranslateWebserverUi/data/batchexecute";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36";

/// The longest text Google accepts in one request, in characters.
pub const MAX_TEXT_CHARS: usize = 5000;

/// Why a translation could not be produced.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The text was empty or whitespace only.
    EmptyText,
    /// The text had more than [`MAX_TEXT_CHARS`] characters.
    TextTooLong {
        /// How many characters were given.
        chars: usize,
    },
    /// [`Language::Auto`] was given as the target language.
    AutoTarget,
    /// The request could not be sent or the response not read.
    Http(reqwest::Error),
    /// The response was not shaped the way this crate expects.
    UnexpectedResponse(&'static str),
    /// The response contained invalid JSON.
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EmptyText => f.write_str("text is empty"),
            Error::TextTooLong { chars } => {
                write!(
                    f,
                    "text is {chars} characters, the limit is {MAX_TEXT_CHARS}"
                )
            }
            Error::AutoTarget => f.write_str("the target language cannot be Auto"),
            Error::Http(err) => write!(f, "request failed: {err}"),
            Error::UnexpectedResponse(what) => write!(f, "unexpected response: {what}"),
            Error::Json(err) => write!(f, "response is not valid JSON: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Http(err) => Some(err),
            Error::Json(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

/// A finished translation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Translation {
    /// The whole translation, segments joined with a space.
    pub text: String,
    /// The translation split the way Google returns it, usually one sentence per entry.
    pub segments: Vec<String>,
    /// The language Google detected the input to be in, when it reported one.
    pub detected_source: Option<Language>,
    /// The language the text was translated into.
    pub target: Language,
}

/// A reusable client. Cheaper than [`translate`] when making several requests.
#[derive(Debug, Clone)]
pub struct Translator {
    client: Client,
}

impl Translator {
    /// Builds a client with a 30 second timeout.
    pub fn new() -> Result<Self, Error> {
        Self::with_timeout(Duration::from_secs(30))
    }

    /// Builds a client that gives up on a request after `timeout`.
    pub fn with_timeout(timeout: Duration) -> Result<Self, Error> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(timeout)
            .build()?;
        Ok(Self { client })
    }

    /// Translates `text` from `source` into `target`.
    ///
    /// Use [`Language::Auto`] as `source` to let Google detect the language.
    pub fn translate(
        &self,
        text: &str,
        source: Language,
        target: Language,
    ) -> Result<Translation, Error> {
        let request = build_request(text, source, target)?;
        let body = self
            .client
            .post(ENDPOINT)
            .header(reqwest::header::REFERER, "https://translate.google.com/")
            .form(&[("f.req", request)])
            .send()?
            .error_for_status()?
            .text()?;
        parse_response(&body, target)
    }
}

/// Translates `text` from `source` into `target` with a one-off client.
pub fn translate(text: &str, source: Language, target: Language) -> Result<Translation, Error> {
    Translator::new()?.translate(text, source, target)
}

fn build_request(text: &str, source: Language, target: Language) -> Result<String, Error> {
    if text.trim().is_empty() {
        return Err(Error::EmptyText);
    }
    let chars = text.chars().count();
    if chars > MAX_TEXT_CHARS {
        return Err(Error::TextTooLong { chars });
    }
    if target == Language::Auto {
        return Err(Error::AutoTarget);
    }
    let inner = json!([[text, source.code(), target.code(), true], [1]]).to_string();
    Ok(json!([[[RPC_ID, inner, Value::Null, "generic"]]]).to_string())
}

/// Parses the raw body of a `batchexecute` response.
fn parse_response(body: &str, target: Language) -> Result<Translation, Error> {
    let line = body
        .lines()
        .find(|line| line.starts_with("[[\"wrb.fr\""))
        .or_else(|| body.lines().rev().find(|line| !line.trim().is_empty()))
        .ok_or(Error::UnexpectedResponse("empty body"))?;
    parse_envelope(line, target)
}

/// Parses the JSON line that carries the RPC result.
fn parse_envelope(line: &str, target: Language) -> Result<Translation, Error> {
    let envelope: Value = serde_json::from_str(line)?;
    let payload = envelope
        .as_array()
        .into_iter()
        .flatten()
        .find(|entry| {
            entry.get(0).and_then(Value::as_str) == Some("wrb.fr")
                && entry.get(1).and_then(Value::as_str) == Some(RPC_ID)
        })
        .and_then(|entry| entry.get(2))
        .and_then(Value::as_str)
        .ok_or(Error::UnexpectedResponse("no result for the translate RPC"))?;
    let result: Value = serde_json::from_str(payload)?;

    let segments = result
        .pointer("/1/0/0/5")
        .and_then(Value::as_array)
        .ok_or(Error::UnexpectedResponse("no translated segments"))?
        .iter()
        .map(|segment| {
            segment
                .get(0)
                .and_then(Value::as_str)
                .map(str::to_owned)
                .ok_or(Error::UnexpectedResponse("segment without text"))
        })
        .collect::<Result<Vec<String>, Error>>()?;
    if segments.is_empty() {
        return Err(Error::UnexpectedResponse("no translated segments"));
    }

    let detected_source = ["/2", "/0/2", "/0/1/1/0"]
        .iter()
        .find_map(|pointer| result.pointer(pointer).and_then(Value::as_str))
        .and_then(Language::from_code);

    Ok(Translation {
        text: segments.join(" "),
        segments,
        detected_source,
        target,
    })
}

#[cfg(test)]
mod tests;
