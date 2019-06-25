extern crate sagume;
use sagume::query::{Clause, Presence, Query};

#[test]
fn test_single_string_term() {
    let mut c = Clause::new("foo".into());
    c.set_fields(vec!["title".into(), "body".into()]);

    let mut query = Query::new();
    query.add_clause(c);

    assert_eq!(query.clauses.len(), 1);
    assert_eq!(query.clauses[0].term(), "foo");
}

#[test]
fn test_is_negated() {
    let mut c1 = Clause::new("foo".into());
    c1.set_presence(Presence::Prohibited);
    let mut c2 = Clause::new("bar".into());
    c2.set_presence(Presence::Prohibited);
    let mut query = Query::new();
    query.add_clause(c1);
    query.add_clause(c2);
    assert!(query.is_negated());

    let mut c1 = Clause::new("foo".into());
    c1.set_presence(Presence::Prohibited);
    let mut c2 = Clause::new("bar".into());
    c2.set_presence(Presence::Required);
    query.add_clause(c1);
    query.add_clause(c2);
    assert!(!query.is_negated());

    let mut c1 = Clause::new("foo".into());
    c1.set_presence(Presence::Optional);
    let mut c2 = Clause::new("bar".into());
    c2.set_presence(Presence::Required);
    query.add_clause(c1);
    query.add_clause(c2);
    assert!(!query.is_negated());
}
