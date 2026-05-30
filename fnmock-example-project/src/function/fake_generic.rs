use std::{ any::TypeId, fmt::Display };

fn get_user<T: Display + 'static, N: Display + 'static>(id: T, name: N) -> String {
    if get_user_fake::is_set_for::<T, N>() {
        let impl_fn = get_user_fake::get_for::<T, N>();
        return impl_fn(id, name);
    }

    format!("User {} ({})", id, name)
}

pub(crate) mod get_user_fake {
    use std::rc::Rc;

    use fnmock::generic_fake_store::GenericFakeStore;

    use super::*;

    thread_local! {
        static FAKE: std::cell::RefCell<GenericFakeStore<2>> = std::cell::RefCell::new(
            GenericFakeStore::new("get_user")
        );
    }

    pub(crate) fn setup<T: Display + 'static, N: Display + 'static>(function: fn(T, N) -> String) {
        let generic_types = [TypeId::of::<T>(), TypeId::of::<N>()];

        FAKE.with_borrow_mut(|fake| {
            fake.setup_for(generic_types, function);
        });
    }

    pub(crate) fn clear() {
        FAKE.with_borrow_mut(|fake| {
            fake.clear();
        })
    }

    pub(crate) fn clear_for<T: Display + 'static, N: Display + 'static>() {
        let generic_types = [TypeId::of::<T>(), TypeId::of::<N>()];

        FAKE.with_borrow_mut(|fake| {
            fake.clear_for(generic_types);
        })
    }

    pub(crate) fn is_set_for<T: Display + 'static, N: Display + 'static>() -> bool {
        let generic_types = [TypeId::of::<T>(), TypeId::of::<N>()];

        FAKE.with_borrow(|fake| { fake.is_set_for(generic_types) })
    }

    pub(crate) fn get_for<T: Display + 'static, N: Display + 'static>() -> Rc<fn(T, N) -> String> {
        let generic_types = [TypeId::of::<T>(), TypeId::of::<N>()];

        FAKE.with_borrow(|fake| { fake.get_for::<fn(T, N) -> String>(generic_types) })
    }
}

fn handle_user(id: i32, name: String) -> String {
    let user = get_user(id, name);
    user
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_user_no_fake() {
        let result = handle_user(1, "Alice".into());

        assert_eq!(result, "User 1 (Alice)");
    }

    #[test]
    fn test_handle_user_with_fake() {
        get_user_fake::setup::<i32, String>(|id, name| format!("Fake User {} ({})", id, name));

        let result = handle_user(1, "Alice".into());

        assert_eq!(result, "Fake User 1 (Alice)");
    }
}
