use axum::{
    Router,
    http::HeaderValue,
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::extract::Form;
use futures::future::join_all;
use itertools::{izip, multizip};
use serde::Deserialize;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};
use zip::write::SimpleFileOptions;

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
        .route("/tsv", post(get_tsv))
        .route("/audio/{language}/{text}", get(audio))
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
struct TsvForm {
    target_language: String,
    source_articles: Vec<String>,
    source_words: Vec<String>,
    translated_articles: Vec<String>,
    translated_words: Vec<String>,
}

#[axum::debug_handler]
async fn get_tsv(Form(form): Form<TsvForm>) -> Result<impl IntoResponse, AppError> {
    let audio_data = form
        .translated_words
        .iter()
        .map(|word| audio::get_audio(word, &form.target_language))
        .collect::<Vec<_>>();
    let audio_data = join_all(audio_data).await;
    let zipped = izip!(
        form.source_articles.iter(),
        form.source_words.iter(),
        form.translated_articles.iter(),
        form.translated_words.iter(),
        audio_data.iter()
    )
    .filter(|(_, _, _, _, audio_data)| audio_data.is_ok())
    .map(|(s_article, s_word, t_article, t_word, audio_data)| {
        (
            s_article,
            s_word,
            t_article,
            t_word,
            audio_data.as_ref().unwrap(),
        )
    })
    .collect::<Vec<_>>();

    let tsv = zipped
        .iter()
        .map(|(s_article, s_word, t_article, t_word, _)| {
            format!(
                "{}\t{}\t{}\t{}\t[{}.mp3]",
                s_article, s_word, t_article, t_word, t_word
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let files = zipped
        .iter()
        .map(|(_, _, _, t_word, audio_data)| {
            let filename = format!("{}.mp3", t_word);
            (filename, audio_data)
        })
        .collect::<Vec<_>>();

    let mut zip_data = Vec::new();
    let mut zip = zip::ZipWriter::new(Cursor::new(&mut zip_data));
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        // .unix_permissions(0o755);
        ;
    for (filename, data) in files {
        zip.start_file(filename, options)?;
        zip.write_all(data)?;
    }
    // writer.flush().unwrap();
    zip.finish()?;

    // Ok(tsv)
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
