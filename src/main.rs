use std::process::ExitCode;

use google_translate::{Language, Translator};

const USAGE: &str = "\
usage: google_translate [--from CODE] [--to CODE] TEXT...
       google_translate --languages

Translates TEXT (all remaining arguments, joined with spaces).
  --from CODE   source language, default auto (detect)
  --to CODE     target language, default en
  --languages   list every language code";

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let mut source = Language::Auto;
    let mut target = Language::English;
    let mut words = Vec::new();
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(());
            }
            "--languages" => {
                for language in Language::ALL {
                    println!("{:<10} {}", language.code(), language.name());
                }
                return Ok(());
            }
            "--from" => source = parse_language(args.next(), "--from")?,
            "--to" => target = parse_language(args.next(), "--to")?,
            _ => words.push(arg),
        }
    }
    if words.is_empty() {
        return Err(USAGE.to_owned());
    }
    let translator = Translator::new().map_err(|err| err.to_string())?;
    let translation = translator
        .translate(&words.join(" "), source, target)
        .map_err(|err| err.to_string())?;
    println!("{}", translation.text);
    Ok(())
}

fn parse_language(value: Option<String>, flag: &str) -> Result<Language, String> {
    let value = value.ok_or_else(|| format!("{flag} needs a language code\n{USAGE}"))?;
    value
        .parse()
        .map_err(|err| format!("{err}; run with --languages to list the codes"))
}
