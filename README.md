# google_translate

Rust client for the unofficial Google Translate web endpoint. Blocking, no API key, 249 languages.

## Library

```toml
[dependencies]
google_translate = { git = "https://github.com/float3/google_translate" }
```

```rust
use google_translate::{Language, Translator};

let translator = Translator::new()?;
let result = translator.translate("Guten Morgen. Wie geht es dir?", Language::Auto, Language::English)?;
assert_eq!(result.text, "Good morning. How are you doing?");
assert_eq!(result.detected_source, Some(Language::German));
```

`Language` parses from and prints as Google's codes (`"zh-CN".parse::<Language>()`). Requests are capped at 5000 characters.

## CLI

```sh
cargo install --git https://github.com/float3/google_translate
google_translate --to ja "good morning"
google_translate --from de --to en Guten Morgen
google_translate --languages
```

## Development

```sh
cargo test
cargo test -- --ignored   # also hits translate.google.com
```

`src/lang.rs` is generated from the language picker on translate.google.com; `langs.txt` is the list it was generated from.
