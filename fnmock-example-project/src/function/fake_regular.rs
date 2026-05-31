fn get_user(id: i32) -> String {
    if get_user_fake::is_set() {
        let impl_fn = get_user_fake::get();
        return impl_fn(id);
    }

    format!("User {}", id)
}

pub(crate) mod get_user_fake {
    use std::rc::Rc;

    use fnmock::{ fake_store::FakeStore };

    thread_local! {
        static FAKE: std::cell::RefCell<FakeStore<fn(i32) -> String>> = std::cell::RefCell::new(
            FakeStore::new("get_user")
        );
    }

    pub(crate) fn setup(function: fn(i32) -> String) {
        FAKE.with_borrow_mut(|fake| {
            fake.setup(function);
        });
    }

    pub(crate) fn clear() {
        FAKE.with_borrow_mut(|fake| {
            fake.clear();
        })
    }

    pub(crate) fn is_set() -> bool {
        FAKE.with_borrow(|fake| { fake.is_set() })
    }

    pub(crate) fn get() -> Rc<fn(i32) -> String> {
        FAKE.with_borrow(|fake| { fake.get() })
    }
}

fn handle_user(id: i32) -> String {
    let user = get_user(id);
    user
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_user_no_fake() {
        let result = handle_user(1);

        assert_eq!(result, "User 1");
    }

    #[test]
    fn test_handle_user_with_fake() {
        get_user_fake::setup(|id| format!("Fake User {}", id));

        let result = handle_user(1);

        assert_eq!(result, "Fake User 1");
    }
}
