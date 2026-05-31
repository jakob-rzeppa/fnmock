use std::fmt::Display;

pub struct UserRepository;

impl UserRepository {
    pub fn get_user<T: Display + 'static>(&self, user_id: T) -> Option<String> {
        #[cfg(test)]
        if UserRepositoryFake::get_user_fake::is_set_for::<T>() {
            let impl_fn: std::rc::Rc<
                fn(&UserRepository, T) -> Option<String>
            > = UserRepositoryFake::get_user_fake::get_for::<T>();
            return impl_fn(self, user_id);
        }

        Some(format!("User{}", user_id))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub(crate) mod UserRepositoryFake {
    use std::{ any::TypeId, rc::Rc };

    use fnmock::generic_fake_store::GenericFakeStore;

    use super::UserRepository;

    thread_local! {
        static GET_USER_FAKE: std::cell::RefCell<GenericFakeStore<1>> = std::cell::RefCell::new(
            GenericFakeStore::new("get_user")
        );
    }

    #[allow(non_camel_case_types)]
    pub(crate) struct get_user_fake;

    impl get_user_fake {
        pub(crate) fn setup<T: 'static>(function: fn(&UserRepository, T) -> Option<String>) {
            let generic_types = [TypeId::of::<T>()];

            GET_USER_FAKE.with_borrow_mut(|fake| {
                fake.setup_for(generic_types, function);
            });
        }

        pub(crate) fn clear() {
            GET_USER_FAKE.with_borrow_mut(|fake| {
                fake.clear();
            })
        }

        pub(crate) fn clear_for<T: 'static>() {
            let generic_types = [TypeId::of::<T>()];

            GET_USER_FAKE.with_borrow_mut(|fake| {
                fake.clear_for(generic_types);
            })
        }

        pub(crate) fn is_set_for<T: 'static>() -> bool {
            let generic_types = [TypeId::of::<T>()];

            GET_USER_FAKE.with_borrow(|fake| { fake.is_set_for(generic_types) })
        }

        pub(crate) fn get_for<T: 'static>() -> Rc<fn(&UserRepository, T) -> Option<String>> {
            let generic_types = [TypeId::of::<T>()];

            GET_USER_FAKE.with_borrow(|fake| {
                fake.get_for::<fn(&UserRepository, T) -> Option<String>>(generic_types)
            })
        }
    }
}

fn handle_user(user_id: u32) -> String {
    let repo = UserRepository;
    match repo.get_user(user_id) {
        Some(user) => format!("Found: {}", user),
        None => "User not found".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_user_with_real_repo() {
        let result = handle_user(1);
        assert_eq!(result, "Found: User1");
    }

    #[test]
    fn test_handle_user_with_fake_repo() {
        UserRepositoryFake::get_user_fake::setup::<u32>(|_, user_id| {
            if user_id == 1 { Some("FakeUser1".to_string()) } else { None }
        });

        let result = handle_user(1);
        assert_eq!(result, "Found: FakeUser1");

        let result = handle_user(2);
        assert_eq!(result, "User not found");
    }
}
