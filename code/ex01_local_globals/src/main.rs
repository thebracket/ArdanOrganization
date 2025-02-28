use crate::master_cache::Document;

mod master_cache;

fn main() {
    let doc = Document {
        id: 1,
        tags: vec!["test".to_string()],
        author: "Test".to_string(),
        body: "Test".to_string(),
    };
    master_cache::insert_snippet(1, doc);

    let doc = master_cache::snipped_by_id(1);
    println!("{doc:?}");
}
