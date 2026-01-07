

pub(super) fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
pub(super) fn add_two_fake(x: i32) -> i32 {
    add_two_fake::get_implementation()(x)
}

#[cfg(test)]
pub(crate) mod add_two_fake {
    use fnmock::function_fake::FunctionFake;

    type Function = fn(i32) -> i32;

    thread_local! {
        static FAKE: std::cell::RefCell<FunctionFake<Function>> =
            std::cell::RefCell::new(FunctionFake::new("add_two"));
    }

    pub(crate) fn fake_implementation(new_f: Function) {
        FAKE.with(|fake| { fake.borrow_mut().fake_implementation(new_f) })
    }

    pub(crate) fn clear_fake() {
        FAKE.with(|fake| { fake.borrow_mut().clear_fake() })
    }

    pub(crate) fn get_implementation() -> Function {
        FAKE.with(|fake| { fake.borrow().get_implementation() })
    }
}