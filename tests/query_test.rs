extern crate sagume;
use sagume::query::{ClauseOption, Presence, Query};

#[test]
fn test_single_string_term() {
    let mut query = Query::new(vec!["title".into(), "body".into()]);
    query.add_term("foo".into(), ClauseOption::new());
    assert_eq!(query.clauses.len(), 1);
    assert_eq!(query.clauses[0].term(), "foo");
}

#[test]
fn test_single_token_term() {}

#[test]
fn multiple_string_terms() {
    let mut query = Query::new(vec!["title".into(), "body".into()]);
    query.add_terms(vec!["foo".into(), "bar".into()], ClauseOption::new());

    let mut terms: Vec<String> = query.clauses.iter().map(|c| c.term()).collect();
    terms.sort();
    assert_eq!(terms, vec!["bar", "foo"]);
}

#[test]
fn multiple_token_terms() {}

#[test]
fn set_default_fields() {
    let mut query = Query::new(vec!["title".into(), "body".into()]);
    query.add_term("foo".into(), ClauseOption::new());

    let clause = &query.clauses[0];
    assert_eq!(clause.option.fields, vec!["title", "body"])
}

#[test]
fn set_specific_fields() {
    let opt = ClauseOption::new().with_fields(vec!["foo".into(), "bar".into()]);
    let mut query = Query::new(vec!["title".into(), "body".into()]);
    query.add_term("foo".into(), opt);

    let clause = &query.clauses[0];
    assert_eq!(clause.option.fields, vec!["foo", "bar"])
}

#[test]
fn test_is_negated() {
    let mut query = Query::new(vec!["title".into(), "body".into()]);
    query.add_term(
        "foo".into(),
        ClauseOption::new().with_presence(Presence::Prohibited),
    );
    query.add_term(
        "bar".into(),
        ClauseOption::new().with_presence(Presence::Prohibited),
    );
    assert!(query.is_negated());

    let mut query = Query::new(vec!["title".into(), "body".into()]);
    query.add_term(
        "foo".into(),
        ClauseOption::new().with_presence(Presence::Prohibited),
    );
    query.add_term(
        "bar".into(),
        ClauseOption::new().with_presence(Presence::Required),
    );
    assert!(!query.is_negated());

    let mut query = Query::new(vec!["title".into(), "body".into()]);
    query.add_term(
        "foo".into(),
        ClauseOption::new().with_presence(Presence::Optional),
    );
    query.add_term(
        "bar".into(),
        ClauseOption::new().with_presence(Presence::Required),
    );
    assert!(!query.is_negated());
}
