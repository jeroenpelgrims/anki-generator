use std::io::Cursor;

use futures::future::join_all;
use itertools::izip;
use std::io::Write;
use zip::write::SimpleFileOptions;

use crate::{audio, error::AppError, router::TsvForm};

pub async fn generate_zip(form: TsvForm) -> Result<Vec<u8>, AppError> {
    let words = izip!(
        form.source_articles.into_iter(),
        form.source_words.into_iter(),
        form.translated_articles.into_iter(),
        form.translated_words.into_iter(),
    )
    .collect::<Vec<_>>();
    let words_with_audio = add_audio_data(words, &form.target_language).await;
    let tsv_content = generate_tsv(&words_with_audio);
    let zip_data = add_files_to_zip(&tsv_content, words_with_audio)?;

    Ok(zip_data)
}

async fn add_audio_data(
    words: Vec<(String, String, String, String)>,
    target_language: &str,
) -> Vec<(String, String, String, String, Vec<u8>)> {
    let requests = words
        .iter()
        .map(|(_, _, _, t_word)| audio::get_audio(t_word, target_language));
    let audio_data = join_all(requests).await;
    let combined = words
        .into_iter()
        .zip(audio_data.into_iter())
        .filter(|(_, audio)| audio.is_ok())
        .map(|(word_data, audio_data)| {
            (
                word_data.0,
                word_data.1,
                word_data.2,
                word_data.3,
                audio_data.unwrap(),
            )
        })
        .collect::<Vec<_>>();

    combined
}

fn generate_tsv(words: &Vec<(String, String, String, String, Vec<u8>)>) -> String {
    let mut tsv_content = String::new();
    for (s_article, s_word, t_article, t_word, _audio) in words {
        let line = format!("{s_article}\t{s_word}\t{t_article}\t{t_word}\t[sound:{t_word}.mp3]\n");
        tsv_content.push_str(&line);
    }
    tsv_content
}

fn add_files_to_zip(
    tsv_content: &str,
    words: Vec<(String, String, String, String, Vec<u8>)>,
) -> Result<Vec<u8>, AppError> {
    let mut zip_data = Vec::new();
    let mut zip = zip::ZipWriter::new(Cursor::new(&mut zip_data));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    // Add audio files
    for (_s_article, _s_word, _t_article, t_word, audio_data) in words {
        let filename = format!("{}.mp3", t_word);
        zip.start_file(filename, options)?;
        zip.write_all(&audio_data)?;
    }

    // Add TSV file
    zip.start_file("words.tsv", options)?;
    zip.write_all(tsv_content.as_bytes())?;
    zip.finish()?;

    Ok(zip_data)
}
