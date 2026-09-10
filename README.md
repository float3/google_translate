# google_translate

Rust client for the unofficial Google Translate RPC endpoint. Blocking, no API key.

```toml
[dependencies]
google_translate = { git = "https://github.com/float3/google_translate" }
```

```rust
use google_translate::{translate, LanguageCode};

let segments = translate("Guten Morgen. Wie geht es dir?", LanguageCode::de, LanguageCode::en)?;
assert_eq!(segments, ["Good morning.", "How are you doing?"]);
```

Input is limited to 5000 characters per call. Language codes are the ISO codes Google uses, as `LanguageCode` variants.
