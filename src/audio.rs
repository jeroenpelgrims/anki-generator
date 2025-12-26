use urlencoding::encode;

pub fn get_url(text: &str, language_code: &str) -> String {
    format!(
        "https://translate.google.com/translate_tts?ie=UTF-8&tl={}&client=tw-ob&q={}",
        language_code,
        encode(text)
    )
}
