use axum::{
    Router,
    extract::Query,
    http::HeaderValue,
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::extract::Form;
use serde::Deserialize;

use crate::{
    audio,
    error::AppError,
    llm,
    templates::{IndexTemplate, TranslateTemplate},
    zip::generate_zip,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/translate", post(translate))
        .route("/tsv", post(get_tsv))
        .route("/audio/{language}/{text}", get(audio))
        .route("/audio/preview", get(audio_preview))
}

async fn index() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Debug, Deserialize)]
struct TranslateForm {
    input: String,
    target_language: String,
}

#[axum::debug_handler]
async fn translate(form: Form<TranslateForm>) -> Result<impl IntoResponse, AppError> {
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
    let response = (
        [(axum::http::header::CONTENT_TYPE, "audio/octet-stream")],
        audio_data,
    );
    Ok(response)
}

#[derive(Debug, Deserialize)]
pub struct TsvForm {
    pub target_language: String,
    pub source_articles: Vec<String>,
    pub source_words: Vec<String>,
    pub translated_articles: Vec<String>,
    pub translated_words: Vec<String>,
}

#[axum::debug_handler]
async fn get_tsv(Form(form): Form<TsvForm>) -> Result<impl IntoResponse, AppError> {
    let zip_data = generate_zip(form).await?;
    let mut response = (axum::http::StatusCode::OK, zip_data).into_response();
    response.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/zip"),
    );
    response.headers_mut().insert(
        axum::http::header::CONTENT_DISPOSITION,
        HeaderValue::from_static("attachment; filename=archive.zip"),
    );

    Ok(response)
}

#[derive(Debug, Deserialize)]
pub struct AudioPreviewParams {
    pub target_language: String,
    pub translated_words: String,
}
async fn audio_preview(
    Query(params): Query<AudioPreviewParams>,
) -> Result<impl IntoResponse, AppError> {
    let language = params.target_language;
    let text = params.translated_words;

    let response = format!(
        "<audio autoplay src=\"/audio/{}/{}\"></audio>",
        language, text
    );
    Ok(response)
}
