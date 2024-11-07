use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter};

fn main() -> tantivy::Result<()> {
    let mut schema_builder = Schema::builder();

    let opts = DateOptions::from(INDEXED)
        .set_stored()
        .set_fast()
        .set_precision(tantivy::schema::DateTimePrecision::Seconds);

    let wrong_date = schema_builder.add_date_field("wrong_date", opts);

    let schema = schema_builder.build();

    let index = Index::create_in_ram(schema.clone());

    let index_writer: IndexWriter = index.writer(50_000_000)?;

    for i in 0..100_000_u64 {
        println!("Indexing doc {}", i);
        index_writer
            .add_document(doc!(
                wrong_date => i,
            ))
            .unwrap();
    }

    Ok(())
}
