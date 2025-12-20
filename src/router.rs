use axum::{
    Router,
    extract::Form,
    routing::{get, post},
};
use serde::Deserialize;

use crate::templates::{IndexTemplate, TranslateTemplate};

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

async fn translate(form: Form<Input>) -> TranslateTemplate {
    let input = form.input.lines().collect::<Vec<_>>();
    println!(">>> {:?}", input);
    TranslateTemplate {}
}
