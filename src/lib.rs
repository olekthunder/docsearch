extern crate tantivy;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use tantivy::schema::{Document, Field};
use tantivy::tokenizer::{
    Language, LowerCaser, RemoveLongFilter, SimpleTokenizer, Stemmer, TextAnalyzer,
};
use tantivy::Index;
use tantivy::IndexWriter;
use walkdir::WalkDir;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DocSearchError {
    #[error(transparent)]
    TantivyError(#[from] tantivy::error::TantivyError),

    #[error("Variable `{0}` is not set")]
    EnvVarError(String),
}

pub type Result<T> = std::result::Result<T, DocSearchError>;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum FileField {
    Path,
    Body,
}

pub type FieldMap<'a> = HashMap<FileField, Field>;

pub fn index_dir(dir_path: impl AsRef<Path>, index_writer: &IndexWriter, field_map: &FieldMap) {
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let mut doc = Document::default();

            let filepath = match entry.path().to_str() {
                Some(f) => f,
                None => continue,
            };
            if let Some(&path_field) = field_map.get(&FileField::Path) {
                doc.add_text(path_field, filepath);
            }

            if let Some(&content_field) = field_map.get(&FileField::Body) {
                let file_content = match read_to_string(filepath) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                doc.add_text(content_field, file_content);
            }
            index_writer.add_document(doc);
        }
    }
}

pub fn register_tokenizers(index: &Index) {
    let ru_tokenizer = TextAnalyzer::from(SimpleTokenizer)
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(Stemmer::new(Language::Russian))
        .filter(Stemmer::new(Language::English));
    index.tokenizers().register("ru_token", ru_tokenizer);
}
