struct InlineStruct;

#[fnmock::fakeable]
impl InlineStruct {
    #[inline]
    fn inline(&self) -> i32 {
        42
    }
}

#[test]
fn test_inline() {
    let s = InlineStruct;
    assert_eq!(s.inline(), 42);
}

#[test]
fn test_inline_mock() {
    InlineStruct::inline_fake().setup(|_| 5);

    let s = InlineStruct;
    assert_eq!(s.inline(), 5);
}
