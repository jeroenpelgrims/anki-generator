use askama::Template;
use askama_web::WebTemplate;

use crate::llm::TranslationResponseItem;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template, WebTemplate)]
#[template(path = "translate.html")]
pub struct TranslateTemplate {
    pub items: Vec<TranslationResponseItem>,
    pub target_language: String,
}
