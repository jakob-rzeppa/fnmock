#[fnmock::spyable]
fn associated_type_bounds<I: Iterator<Item = u8> + 'static>(iter: I, tag: &str) -> usize {
    let _ = tag;
    iter.count()
}

#[test]
fn test_associated_type_bounds() {
    let spy = associated_type_bounds_spy::<std::vec::IntoIter<u8>>();
    spy.expectf(|_iter, tag: &str| tag == "tag").once();

    let res = associated_type_bounds(vec![1u8, 2, 3].into_iter(), "tag");

    assert_eq!(res, 3);
    spy.assert();
}
