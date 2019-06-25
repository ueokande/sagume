use sagume::builder::Builder;
use sagume::document::Document;
use sagume::field::Field;
use sagume::index::Index;
use sagume::query::{Clause, Query};

fn get_index() -> Index {
    let mut doc1 = Document::new("a".into());
    doc1.add_field(Field::new_text(
        "title".into(),
        "Mr. Green kills Colonel Mustard".into(),
    ));
    doc1.add_field(Field::new_text(
            "body".into(),
            "Mr. Green killed Colonel Mustard in the study with the candlestick. Mr. Green is not a very nice fellow.".into()));

    let mut doc2 = Document::new("b".into());
    doc2.add_field(Field::new_text("title".into(), "Plumb waters plant".into()));
    doc2.add_field(Field::new_text(
        "body".into(),
        "Professor Plumb has a green plant in his study".into(),
    ));

    let mut doc3 = Document::new("c".into());
    doc3.add_field(Field::new_text(
        "title".into(),
        "Scarlett helps Professor".into(),
    ));
    doc3.add_field(Field::new_text( "body".into(), "Miss Scarlett watered Professor Plumbs green plant while he was away from his office last week.".into()));

    let mut builder = Builder::new();
    builder.add_field("title".into());
    builder.add_field("body".into());
    for doc in vec![doc1, doc2, doc3] {
        builder.add_document(doc);
    }
    builder.build()
}

#[test]
fn test_search_with_single_term() {
    // one match
    let mut q = Query::new();
    q.add_clause(Clause::new("scarlett".into()));

    let index = get_index();
    let results = index.query(&q);

    assert_eq!(results.len(), 1);
    assert_eq!(results.first().unwrap().doc_ref(), "c")
}

#[test]
fn test_search_result_order() {
    let mut q = Query::new();
    q.add_clause(Clause::new("professor".into()));

    let index = get_index();
    let results = index.query(&q);

    assert_eq!(results.first().unwrap().doc_ref(), "b");
}
