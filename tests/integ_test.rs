//! some integration tests

mod shared;

#[test]
fn it_works() {
    let comparator = shared::add(2, 2);
    let testee = tinyapplause::add(2, 2);
    assert_eq!(testee, comparator);
}
