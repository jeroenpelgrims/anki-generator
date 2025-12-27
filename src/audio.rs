use urlencoding::encode;

pub fn get_url(text: &str, language_code: &str) -> String {
    format!(
        "https://translate.google.com/translate_tts?ie=UTF-8&tl={}&client=tw-ob&q={}",
        language_code,
        encode(text)
    )
}

pub async fn get_audio(text: &str, language_code: &str) -> Result<Vec<u8>, anyhow::Error> {
    let url = get_url(text, language_code);
    let response = reqwest::get(&url).await.expect("Failed to fetch audio");
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
