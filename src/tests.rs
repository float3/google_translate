#[test]
fn it_works() {
    println!("Translating \"test\" into german:");
    let text = "test";
    let source_language = super::LanguageCode::de;
    let target_language = super::LanguageCode::en;
    let result = super::translate(text, source_language, target_language);
    match result {
        Result::Ok(result) => {
            for res in result {
                println!("{}", res)
            }
        }
        _ => println!("failed"),
    }
}
