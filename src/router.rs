use axum::{
    Router,
    extract::Form,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;

use crate::{
    audio,
    error::AppError,
    llm,
    templates::{IndexTemplate, TranslateTemplate},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/translate", post(translate))
        .route("/audio/{language}/{text}", get(audio))
}

async fn index() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Debug, Deserialize)]
struct Input {
    input: String,
    target_language: String,
}

#[axum::debug_handler]
async fn translate(form: Form<Input>) -> Result<impl IntoResponse, AppError> {
    let input = form
        .input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    let result = llm::translate(input, form.target_language.clone()).await?;
    Ok(TranslateTemplate {
        items: result,
        target_language: form.target_language.clone(),
    })
}

async fn audio(
    axum::extract::Path((language, text)): axum::extract::Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let audio_data = audio::get_audio(&text, &language).await?;
    let response = ([("content-type", "audio/octet-stream")], audio_data);
    Ok(response)
    // Ok((axum::http::header::CONTENT_TYPE, "audio/mpeg", audio_data))
}
