struct SelfBoxed {
    value: i32,
}

#[fnmock::fakeable]
impl SelfBoxed {
    fn get(me: Box<Self>) -> i32 {
        me.value
    }
}

#[test]
fn test_self_boxed() {
    let s = Box::new(SelfBoxed { value: 42 });
    assert_eq!(SelfBoxed::get(s), 42);
}

#[test]
fn test_self_boxed_fake() {
    SelfBoxed::get_fake().setup(|_| 5);

    let s = Box::new(SelfBoxed { value: 42 });
    assert_eq!(SelfBoxed::get(s), 5);
}
