extern crate sagume;

use sagume::token::TokenSet;

#[test]
fn test_to_vec() {
    let set = TokenSet::from_string("bat".into());
    assert_eq!(set.to_vec(), vec!["bat"]);
}

#[test]
fn test_intersect() {
    let x = TokenSet::from_string("cat".into());
    let y = TokenSet::from_string("dog".into());
    let z = x.intersect(&y);
    assert!(z.to_vec().is_empty());

    let x = TokenSet::from_string("cat".into());
    let y = TokenSet::from_string("cat".into());
    let z = x.intersect(&y);
    assert_eq!(z.to_vec(), vec!["cat"]);

    let x = TokenSet::from_string("cat".into());
    let y = TokenSet::from_string("c*".into());
    let z = x.intersect(&y);
    assert_eq!(z.to_vec(), vec!["cat"]);

    let x = TokenSet::from_string("cat".into());
    let y = TokenSet::from_string("d*".into());
    let z = x.intersect(&y);
    assert!(z.to_vec().is_empty());

    let x = TokenSet::from_string("cat".into());
    let y = TokenSet::from_string("*t".into());
    let z = x.intersect(&y);
    assert_eq!(z.to_vec(), vec!["cat"]);

    let x = TokenSet::from_string("aaacbab".into());
    let y = TokenSet::from_string("*ab".into());
    let z = x.intersect(&y);
    assert_eq!(z.to_vec(), vec!["aaacbab"]);

    let x = TokenSet::from_string("cat".into());
    let y = TokenSet::from_string("*r".into());
    let z = x.intersect(&y);
    assert!(z.to_vec().is_empty());

    let x = TokenSet::from_string("aaabdcbc".into());
    let y = TokenSet::from_string("*abc".into());
    let z = x.intersect(&y);
    assert!(z.to_vec().is_empty());

    let x = TokenSet::from_string("foo".into());
    let y = TokenSet::from_string("f*o".into());
    let z = x.intersect(&y);
    assert_eq!(z.to_vec(), vec!["foo"]);

    let x = TokenSet::from_string("ababc".into());
    let y = TokenSet::from_string("a*bc".into());
    let z = x.intersect(&y);
    assert_eq!(z.to_vec(), vec!["ababc"]);

    let x = TokenSet::from_string("foo".into());
    let y = TokenSet::from_string("b*r".into());
    let z = x.intersect(&y);
    assert!(z.to_vec().is_empty());

    let x = TokenSet::from_string("ababc".into());
    let y = TokenSet::from_string("a*ac".into());
    let z = x.intersect(&y);
    assert!(z.to_vec().is_empty());

    let x = TokenSet::from_string("foo".into());
    let y = TokenSet::from_string("foo*".into());
    let z = x.intersect(&y);
    assert_eq!(z.to_vec(), vec!["foo"]);

    let x = TokenSet::from_string(
        "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".into(),
    );
    let y = TokenSet::from_string("*ff".into());
    let z = x.intersect(&y);
    assert_eq!(z.to_vec().len(), 1);

    let x = TokenSet::from_string("acbaabab".into());
    let y = TokenSet::from_string("*ab*".into());
    let z = x.intersect(&y);
    assert_eq!(z.to_vec(), vec!["acbaabab"]);

    let x = TokenSet::from_string("acbaabab".into());
    let y = TokenSet::from_string("a*ba*b".into());
    let z = x.intersect(&y);
    assert_eq!(z.to_vec(), vec!["acbaabab"]);
}
