use std::path::PathBuf;
use tantivy::{query, Index};

use tantivy::schema::*;
use tantivy::collector::TopDocs;

fn path_for_version_read(version: &str) -> String {
    format!("./tests/compat_tests_data/index_v{}/", version)
}

fn main() -> tantivy::Result<()> {

    let path = "./tests/compat_tests_data/01JXGAR7N8TJPHVZGVJEBRBQAN.split";//path_for_version_read("8");

    let split_file = PathBuf::from(path);

    let index = Index::open_in_dir(split_file.as_path()).expect("Failed to open index");

    let reader = index.reader().expect("Failed to create reader");
    let searcher = reader.searcher();

    let schema = index.schema();

    let span_name_field = schema.get_field("span_name").expect("Field 'span_name' not found");
    let query_parser = query::QueryParser::for_index(&index, vec![span_name_field]);

    let query = query_parser.parse_query("*").expect("Failed to parse query");

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        println!("{}", retrieved_doc.to_json(&schema));
    }

    // println!("Index schema: {:?}", schema);

    Ok(())
}
