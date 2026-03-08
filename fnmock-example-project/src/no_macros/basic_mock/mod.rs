pub mod db {
    pub fn fetch_user(id: u32) -> Result<String, String> {
        // Call the mock implementation if set (only in test mode)
        #[cfg(test)]
        if fetch_user_mock::is_set() {
            return fetch_user_mock::call(id);
        }

        // Real implementation
        Ok(format!("user_{}", id))
    }

    pub(crate) mod fetch_user_mock {
        thread_local! {
            static MOCK: std::cell::RefCell<fnmock::function_mock::FunctionMock<
                u32,
                Result<String, String>,
            >> = std::cell::RefCell::new(fnmock::function_mock::FunctionMock::new("fetch_user_mock"));
        }

        pub(crate) fn call(params: u32) -> Result<String, String> {
            MOCK.with(|mock| {
                mock.borrow_mut().call(params)
            })
        }

        pub(crate) fn setup(new_f: fn(u32) -> Result<String, String>) {
            MOCK.with(|mock| {
                mock.borrow_mut().setup(new_f)
            })
        }

        pub(crate) fn is_set() -> bool {
            MOCK.with(|mock| {
                mock.borrow().is_set()
            })
        }

        pub(crate) fn assert_with(id: u32) {
            MOCK.with(|mock| {
                mock.borrow().assert_with(id)
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let result = fetch_user(4);

            assert!(result.is_ok());
            let result = result.unwrap();
            assert_eq!(result, "user_4".to_string());
        }
    }
}

use db::fetch_user;

pub fn handle_user(id: u32) {
    let _user = fetch_user(id);

    // Do something with the user
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::db::fetch_user_mock;

    #[test]
    fn test_with_mock() {
        // Set up mock behavior
        fetch_user_mock::setup(|_| {
            Ok("mock user".to_string())
        });

        handle_user(42);

        // Verify behavior
        fetch_user_mock::assert_with(42);

        // No cleanup needed, since mocks are thread / test specific
    }
}