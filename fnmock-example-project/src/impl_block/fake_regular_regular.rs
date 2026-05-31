pub struct UserRepository;

impl UserRepository {
    pub fn get_user(&self, user_id: u32) -> Option<String> {
        #[cfg(test)]
        if UserRepositoryFake::get_user_fake::is_set() {
            let impl_fn = UserRepositoryFake::get_user_fake::get();
            return impl_fn(self, user_id);
        }

        Some(format!("User{}", user_id))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub(crate) mod UserRepositoryFake {
    use std::rc::Rc;

    use fnmock::{ fake_store::FakeStore };

    use crate::impl_block::fake_regular_regular::UserRepository;

    thread_local! {
        static GET_USER_FAKE: std::cell::RefCell<
            FakeStore<fn(&UserRepository, u32) -> Option<String>>
        > = std::cell::RefCell::new(FakeStore::new("get_user"));
    }

    #[allow(non_camel_case_types)]
    pub(crate) struct get_user_fake;

    impl get_user_fake {
        pub(crate) fn setup(function: fn(&UserRepository, u32) -> Option<String>) {
            GET_USER_FAKE.with_borrow_mut(|fake| {
                fake.setup(function);
            });
        }

        pub(crate) fn clear() {
            GET_USER_FAKE.with_borrow_mut(|fake| {
                fake.clear();
            })
        }

        pub(crate) fn is_set() -> bool {
            GET_USER_FAKE.with_borrow(|fake| { fake.is_set() })
        }

        pub(crate) fn get() -> Rc<fn(&UserRepository, u32) -> Option<String>> {
            GET_USER_FAKE.with_borrow(|fake| { fake.get() })
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
        UserRepositoryFake::get_user_fake::setup(|_, user_id| {
            if user_id == 1 { Some("FakeUser1".to_string()) } else { None }
        });

        let result = handle_user(1);
        assert_eq!(result, "Found: FakeUser1");

        let result = handle_user(2);
        assert_eq!(result, "User not found");
    }
}
