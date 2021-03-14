use docsearch::{index_dir, register_tokenizers, FieldMap, FileField, DocSearchError};
use std::env;
use tantivy::schema::{Schema, TextFieldIndexing, TextOptions, STORED, STRING};
use tantivy::Index;

const INDEX_DIR: &str = ".index/";

const DOC_DIR_ENV_NAME: &str = "DOCSEARCH_DOC_DIR";
fn main() -> docsearch::Result<()> {
    let doc_dir = env::var(DOC_DIR_ENV_NAME)
        .map_err(|_| DocSearchError::EnvVarError(DOC_DIR_ENV_NAME.to_owned()))?;
    let mut schema_builder = Schema::builder();
    let text_field_indexing = TextFieldIndexing::default().set_tokenizer("ru_token");
    let text_options = TextOptions::default().set_indexing_options(text_field_indexing);
    schema_builder.add_text_field("path", STRING | STORED);
    schema_builder.add_text_field("body", text_options);
    let schema = schema_builder.build();
    let path = schema.get_field("path").unwrap();
    let body = schema.get_field("body").unwrap();

    let index = Index::create_in_dir(INDEX_DIR, schema.clone())?;
    register_tokenizers(&index);

    let mut index_writer = index.writer(50_000_000)?;

    let mut field_map = FieldMap::new();
    field_map.insert(FileField::Path, path);
    field_map.insert(FileField::Body, body);
    index_dir(doc_dir, &index_writer, &field_map);

    index_writer.commit()?;
    Ok(())
}
