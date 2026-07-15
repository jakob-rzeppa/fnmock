use std::pin::Pin;

struct SelfPinMut {
    value: i32,
}

#[fnmock::fakeable]
impl SelfPinMut {
    fn get(self: Pin<&mut Self>) -> i32 {
        self.value
    }
}

#[test]
fn test_self_pin_mut() {
    let mut s = SelfPinMut { value: 42 };
    assert_eq!(SelfPinMut::get(Pin::new(&mut s)), 42);
}

#[test]
fn test_self_pin_mut_fake() {
    SelfPinMut::get_fake().setup(|_| 5);

    let mut s = SelfPinMut { value: 42 };
    assert_eq!(SelfPinMut::get(Pin::new(&mut s)), 5);
}
