use document_cache::{DOCUMENTS, Document};

mod document_cache;

fn article_by_id(id: usize) -> Document {
    let key = format!("ID: {id}");
    if let Some(document) = DOCUMENTS.lock().unwrap().get(&key) {
        return document;
    }

    // We didn't find it. So we need to load it from the database.
    let dummy_doc = Document {
        id,
        tags: vec!["blob".to_string()],
        author: "Herbert".to_string(),
        body: "It was the best of times, it was the worst of times".to_string(),
    };
    DOCUMENTS.lock().unwrap().insert(key, dummy_doc.clone());
    dummy_doc
}

fn article_by_tag(tag: &str) -> Vec<Document> {
    DOCUMENTS
        .lock()
        .unwrap()
        .find_many::<Document>(|(_, d)| d.tags.iter().any(|t| t.to_lowercase() == tag.to_lowercase()))
        .into_iter()
        .map(|(_, d)| d)
        .collect()
}

fn main() {
    let doc = article_by_id(1);
    println!("{doc:?}");
    let doc = article_by_tag("blob");
    println!("{doc:?}");
}
