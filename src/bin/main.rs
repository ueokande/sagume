use sagume::builder::Builder;
use sagume::document::Document;
use sagume::field::Field;
use sagume::query::{Clause, Query};

fn main() {
    let mut builder = Builder::new();
    builder.add_field("author".to_string());
    builder.add_field("isbn".to_string());

    let data = vec![
        ("1234567890", "Lunr.js in Action", "John Smith"),
        ("5678901234", "Management Petabytes", "Nur Major"),
        ("0022446688", "The Art of Full-test search", "Shreya Gamble"),
        ("0987676543", "Test search dressing", "Peyton Weston"),
        ("4567846395", "Lunr.js: The definitive guide", "Fariha Le"),
    ];
    for (isbn, title, author) in data.iter() {
        let mut doc = Document::new(isbn.to_string());
        doc.add_field(Field::new_text("isbn".to_string(), isbn.to_string()));
        doc.add_field(Field::new_text("title".to_string(), title.to_string()));
        doc.add_field(Field::new_text("author".to_string(), author.to_string()));

        builder.add_document(doc);
    }

    let idx = builder.build();

    let mut c = Clause::new("art".to_string());
    c.set_fields(vec!["title".to_string()]);

    let mut q = Query::new();
    q.add_clause(c);

    let results = idx.query(&q);
    println!("matched {} documents", results.len());
    for r in results.iter() {
        println!("  isbn={} ({})", r.doc_ref(), r.score());
    }
}
