use docsearch::register_tokenizers;
use std::env;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{Index, ReloadPolicy};

const INDEX_DIR: &str = ".index/";

fn main() -> tantivy::Result<()> {
    let search_query = env::args().nth(1).unwrap();
    let index = Index::open_in_dir(INDEX_DIR)?;
    register_tokenizers(&index);
    let schema = index.schema();
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;
    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, schema.fields().map(|t| t.0).collect());
    let query = query_parser.parse_query(&search_query)?;
    // println!("{:?}", index.tokenizers());
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("{}", schema.to_json(&retrieved_doc));
    }
    Ok(())
}
