use std::cell::Cell;

struct ReturnUnit {
    calls: Cell<i32>,
}

#[fnmock::fakeable]
impl ReturnUnit {
    fn record(&self) {
        self.calls.set(self.calls.get() + 1);
    }
}

#[test]
fn test_return_unit() {
    let s = ReturnUnit { calls: Cell::new(0) };
    s.record();
    assert_eq!(s.calls.get(), 1);
}

#[test]
fn test_return_unit_fake() {
    ReturnUnit::record_fake().setup(|_| ());

    let s = ReturnUnit { calls: Cell::new(0) };
    s.record();
    assert_eq!(s.calls.get(), 0);
}
