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

#[test]
fn test_intersect_with_fuzzy_string() {
    let x1 = TokenSet::from_string("bar".into());
    let x2 = TokenSet::from_string("cur".into());
    let x3 = TokenSet::from_string("cat".into());
    let x4 = TokenSet::from_string("car".into());
    let x5 = TokenSet::from_string("for".into());
    let y = TokenSet::from_fuzzy_string("car".into(), 1);
    assert_eq!(x1.intersect(&y).to_vec(), vec!["bar"]);
    assert_eq!(x2.intersect(&y).to_vec(), vec!["cur"]);
    assert_eq!(x3.intersect(&y).to_vec(), vec!["cat"]);
    assert_eq!(x4.intersect(&y).to_vec(), vec!["car"]);
    assert!(x5.intersect(&y).to_vec().is_empty());

    let x1 = TokenSet::from_string("ar".into());
    let x2 = TokenSet::from_string("br".into());
    let x3 = TokenSet::from_string("ba".into());
    let x4 = TokenSet::from_string("bar".into());
    let x5 = TokenSet::from_string("foo".into());
    let y = TokenSet::from_fuzzy_string("bar".into(), 1);
    assert_eq!(x1.intersect(&y).to_vec(), vec!["ar"]);
    assert_eq!(x2.intersect(&y).to_vec(), vec!["br"]);
    assert_eq!(x3.intersect(&y).to_vec(), vec!["ba"]);
    assert_eq!(x4.intersect(&y).to_vec(), vec!["bar"]);
    assert!(x5.intersect(&y).to_vec().is_empty());

    let x1 = TokenSet::from_string("bbar".into());
    let x2 = TokenSet::from_string("baar".into());
    let x3 = TokenSet::from_string("barr".into());
    let x4 = TokenSet::from_string("bar".into());
    let x5 = TokenSet::from_string("ba".into());
    let x6 = TokenSet::from_string("foo".into());
    let x7 = TokenSet::from_string("bara".into());
    let y = TokenSet::from_fuzzy_string("bar".into(), 1);
    assert_eq!(x1.intersect(&y).to_vec(), vec!["bbar"]);
    assert_eq!(x2.intersect(&y).to_vec(), vec!["baar"]);
    assert_eq!(x3.intersect(&y).to_vec(), vec!["barr"]);
    assert_eq!(x4.intersect(&y).to_vec(), vec!["bar"]);
    assert_eq!(x5.intersect(&y).to_vec(), vec!["ba"]);
    assert!(x6.intersect(&y).to_vec().is_empty());
    assert_eq!(x7.intersect(&y).to_vec(), vec!["bara"]);

    let x1 = TokenSet::from_string("abr".into());
    let x2 = TokenSet::from_string("bra".into());
    let x3 = TokenSet::from_string("foo".into());
    let y = TokenSet::from_fuzzy_string("bar".into(), 1);
    assert_eq!(x1.intersect(&y).to_vec(), vec!["abr"]);
    assert_eq!(x2.intersect(&y).to_vec(), vec!["bra"]);
    assert!(x3.intersect(&y).to_vec().is_empty());

    let x = TokenSet::from_string("abcxx".into());
    let y = TokenSet::from_fuzzy_string("abc".into(), 2);
    assert_eq!(x.intersect(&y).to_vec(), vec!["abcxx"]);

    let x = TokenSet::from_string("axx".into());
    let y = TokenSet::from_fuzzy_string("abc".into(), 2);
    assert_eq!(x.intersect(&y).to_vec(), vec!["axx"]);

    let x = TokenSet::from_string("a".into());
    let y = TokenSet::from_fuzzy_string("abc".into(), 2);
    assert_eq!(x.intersect(&y).to_vec(), vec!["a"]);

    let x = TokenSet::from_string("bca".into());
    let y = TokenSet::from_fuzzy_string("abc".into(), 2);
    assert_eq!(x.intersect(&y).to_vec(), vec!["bca"]);
}
