extern crate sagume;
use sagume::vector::Vector;

#[test]
fn test_magnitude() {
    let v = Vector::from(vec![4.0, 5.0, 6.0]);
    assert_eq!(v.magnitude(), 77.0f64.sqrt());
}

#[test]
fn test_dot() {
    let v1 = Vector::from(vec![1.0, 3.0, -5.0]);
    let v2 = Vector::from(vec![4.0, -2.0, -1.0]);
    assert_eq!(v1.dot(&v2), 3.0);
    assert_eq!(v2.dot(&v1), 3.0);
}

#[test]
fn test_similarity() {
    let v1 = Vector::from(vec![1.0, 3.0, -5.0]);
    let v2 = Vector::from(vec![4.0, -2.0, -1.0]);
    assert_eq!(v1.similarity(&v2).floor(), 0.5f64.floor());
    assert_eq!(v2.similarity(&v1).floor(), 0.5f64.floor());
}

#[test]
fn test_non_overlapping_vector() {
    let mut v1 = Vector::new();
    let mut v2 = Vector::new();
    v1.insert(0, 1.0);
    v2.insert(1, 1.0);

    assert_eq!(v1.similarity(&v2), 0.0);
    assert_eq!(v2.similarity(&v1), 0.0);
}

#[test]
fn test_empty_vector() {
    let v1 = Vector::from(vec![]);
    let v2 = Vector::from(vec![1.0]);
    assert_eq!(v1.similarity(&v2), 0.0);
    assert_eq!(v2.similarity(&v1), 0.0);
}

#[test]
#[should_panic]
fn test_insert_panic() {
    let mut v1 = Vector::new();
    v1.insert(0, 10.0);
    v1.insert(1, 20.0);
    v1.insert(1, 30.0);
}
