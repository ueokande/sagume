extern crate sagume;

use sagume::builder::Builder;
use sagume::document::Document;
use sagume::field::{Field, FieldRef};
use sagume::token::TokenSet;

#[test]
fn test_build() {
    let mut doc = Document::new("1".into());
    doc.add_field(Field::new_text("title".into(), "Lucene in Action".into()));
    doc.add_field(Field::new_text("isbn".into(), "9781932394283".into()));
    doc.add_field(Field::new_text("memo".into(), "good".into()));

    let mut b = Builder::new();
    b.add_field("title".into());
    b.add_field("isbn".into());
    b.add_document(doc);
    let index = b.build();

    assert!(index
        .inverted_index()
        .get("lucene")
        .unwrap()
        .documents
        .get("title")
        .unwrap()
        .contains("1"));
    assert!(index
        .field_vectors()
        .contains_key(&FieldRef::new("1".into(), "title".into())));

    assert_eq!(
        index
            .token_set()
            .intersect(&TokenSet::from_string("action".into()))
            .to_vec(),
        vec!["action"],
    );
}
