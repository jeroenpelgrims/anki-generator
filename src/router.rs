use axum::{
    Router,
    extract::Form,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;

use crate::{
    error::AppError,
    llm,
    templates::{IndexTemplate, TranslateTemplate},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/translate", post(translate))
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
    Ok(TranslateTemplate { items: result })
}
