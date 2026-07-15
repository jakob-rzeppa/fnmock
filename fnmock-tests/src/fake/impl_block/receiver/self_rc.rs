use std::rc::Rc;

struct SelfRc {
    value: i32,
}

#[fnmock::fakeable]
impl SelfRc {
    fn get(self: Rc<Self>) -> i32 {
        self.value
    }
}

#[test]
fn test_self_rc() {
    let s = Rc::new(SelfRc { value: 42 });
    assert_eq!(SelfRc::get(s), 42);
}

#[test]
fn test_self_rc_fake() {
    SelfRc::get_fake().setup(|_| 5);

    let s = Rc::new(SelfRc { value: 42 });
    assert_eq!(SelfRc::get(s), 5);
}
